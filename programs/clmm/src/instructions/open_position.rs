
use anchor_lang::{prelude::*, Accounts};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::constants::{ANCHOR_SIZE, POSITION_SEED, TICK_ARRAY_SEED};
use crate::libraries::liquidity_math;
use crate::state::{PoolState, PositionState, TickStateArray, TickStateArrayBitMap};
use crate::error::ErrorCode;
use crate::util::{self, AccountLoad};

#[derive(Accounts)]
#[instruction(tick_lower: i32, tick_upper: i32)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub lp: Signer<'info>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    #[account(
        init,
        payer = lp,
        space = ANCHOR_SIZE + PositionState::LEN,
        seeds = [
            POSITION_SEED.as_bytes(), 
            &tick_lower.to_le_bytes(),
            &tick_upper.to_le_bytes(),
            lp.key().as_ref()],
        bump,
    )]
    pub position: Box<Account<'info, PositionState>>,

    #[account(mut, constraint = token_vault_0.key() == pool_state.load_mut()?.token_vault_0)]
    pub token_vault_0: Box<Account<'info, TokenAccount>>,
    #[account(mut, constraint = token_vault_1.key() == pool_state.load_mut()?.token_vault_1)]
    pub token_vault_1: Box<Account<'info, TokenAccount>>,
    #[account(mut, token::mint = token_vault_0.mint)]
    pub token_account_0: Box<Account<'info, TokenAccount>>,
    #[account(mut, token::mint = token_vault_1.mint)]
    pub token_account_1: Box<Account<'info, TokenAccount>>,

    // Just avoid to compute the PDA on chain.
    #[account(
        seeds = [
            TICK_ARRAY_SEED.as_bytes(),
            pool_state.key().as_ref(),
            &tick_lower.to_le_bytes(),
        ],
        bump
    )]
    pub tick_array_lower_pda: UncheckedAccount<'info>,

    // Just avoid to compute the PDA on chain.
    #[account(
        seeds = [
            TICK_ARRAY_SEED.as_bytes(),
            pool_state.key().as_ref(),
            &tick_upper.to_le_bytes(),
        ],
        bump
    )]
    pub tick_array_upper_pda: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    // remaining account
    // #[account(
    //     seeds = [
    //         TICK_ARRAY_BITMAP_SEED.as_bytes(),
    //         pool_state.key().as_ref(),
    //     ],
    //     bump
    // )]
    // pub tick_array_bitmap: AccountLoader<'info, TickStateArrayBitmap>,
}

pub fn open_position_impl(
        ctx: Context<OpenPosition>,
        tick_lower: i32,
        tick_upper: i32,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
    ) -> Result<()> {
    // === Validation ===
    let pool_state = ctx.accounts.pool_state.load()?;
    let tick_spacing = pool_state.tick_spacing;
    crate::state::tick_index_check!(tick_lower, tick_spacing);
    crate::state::tick_index_check!(tick_upper, tick_spacing);
    require_gt!(tick_upper, tick_lower, ErrorCode::InvalidTickIndexOrder);
    require!(liquidity > 0, ErrorCode::LiquidityZero);

    // === Calculate Actual Token Amounts ===
    let (amount_0, amount_1) = liquidity_math::get_delta_amounts_signed(
            pool_state.tick_current, 
            pool_state.sqrt_price_x64, 
            tick_lower, 
            tick_upper, liquidity as i128)?;
    require!(amount_0 <= amount_0_max, ErrorCode::SlippageLimitExceeded);
    require!(amount_1 <= amount_1_max, ErrorCode::SlippageLimitExceeded);

    // === Transfer Tokens ===
    if amount_0 > 0 {
        token::transfer(ctx.accounts.transfer_ctx_0(), 0)?;
    }
    if amount_1 > 0 {
        token::transfer(ctx.accounts.transfer_ctx_1(), amount_1)?;
    }

    // === Update Tick State ===
    let lower_bump = ctx.bumps.tick_array_lower_pda;
    let upper_bump = ctx.bumps.tick_array_upper_pda;
    let lower_flipped = ctx.accounts
       .load_tick_array_lower(tick_lower, tick_spacing, lower_bump)?
       .load_mut()?
       .update_tick(&pool_state, tick_lower, liquidity as i128, false)?;
    let upper_flipped = ctx.accounts
       .load_tick_array_upper(tick_upper, tick_spacing, upper_bump)?
       .load_mut()?
       .update_tick(&pool_state, tick_upper, liquidity as i128, true)?;

    // === Update TickStateArrayBitmap ===
    if lower_flipped || upper_flipped {
        require!(ctx.remaining_accounts.len() >= 1, ErrorCode::RemainingAccountMissed);
        require_keys_eq!(ctx.remaining_accounts[0].key(), pool_state.tick_array_bitmap);
        util::account_map_mut(
            &ctx.remaining_accounts[0], 
            |bitmap: &mut TickStateArrayBitMap| -> Result<()> {
                if lower_flipped {
                    bitmap.flip(tick_lower, tick_spacing)?;
                }
                if upper_flipped {
                    bitmap.flip(tick_upper, tick_spacing)?;
                }
                Ok(())
            })??;
    }
    
    // === Initialize Position Account ===
    let position = &mut ctx.accounts.position;
    position.initialize(
        liquidity, 
        pool_state.fee_growth_global_0_x64, 
        pool_state.fee_growth_global_1_x64);

    // === Update PoolState ===
    drop(pool_state);
    {
        let pool_state = &mut ctx.accounts.pool_state.load_mut()?;
        if pool_state.tick_current >= tick_lower &&
           pool_state.tick_current < tick_upper {
            pool_state.liquidity.checked_add(liquidity)
                .ok_or(ErrorCode::LiquidityAddValueErr)?;
        }
    }
    Ok(())
}

impl<'info> OpenPosition<'info> {
    pub fn transfer_ctx_0(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.token_account_0.to_account_info(),
            to: self.token_vault_0.to_account_info(),
            authority: self.lp.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    pub fn transfer_ctx_1(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.token_account_1.to_account_info(),
            to: self.token_vault_1.to_account_info(),
            authority: self.lp.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    pub fn load_tick_array_lower(&self, tick_lower: i32, tick_spacing: u16, bump: u8) -> Result<AccountLoad<'info, TickStateArray>> {
        TickStateArray::get_or_create_tick_array(
            self.tick_array_lower_pda.to_account_info(), 
            self.lp.to_account_info(),
            self.system_program.to_account_info(), 
            &self.pool_state, 
            TickStateArray::get_array_start_index(tick_lower, tick_spacing), 
            tick_spacing, 
            bump)
    }

    pub fn load_tick_array_upper(&self, tick_upper: i32, tick_spacing: u16, bump: u8) -> Result<AccountLoad<'info, TickStateArray>> {
        TickStateArray::get_or_create_tick_array(
            self.tick_array_upper_pda.to_account_info(), 
            self.lp.to_account_info(),
            self.system_program.to_account_info(), 
            &self.pool_state, 
            TickStateArray::get_array_start_index(tick_upper, tick_spacing), 
            tick_spacing, 
            bump)
    }
}