use std::cell::RefMut;

use anchor_lang::{prelude::*, Accounts};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use solana_program::example_mocks::solana_sdk::system_instruction;
use solana_program::program::{invoke_signed, invoke_signed_unchecked};

use crate::constants::{ANCHOR_SIZE, POSITION_SEED, TICK_ARRAY_SEED};
use crate::libraries::liquidity_math;
use crate::state::{PoolState, PositionState, TickStateArray};
use crate::error::ErrorCode;

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

    // Use Option<Account<...>> to check existence + validity
    // Anchor attempts deserialization. If it fails (account doesn't exist,
    // wrong owner, wrong data type), this will be None.
    // Seeds constraint ensures we're checking the *correct* PDA address.
    #[account(
        seeds = [
            TICK_ARRAY_SEED.as_bytes(),
            pool_state.key().as_ref(),
            &tick_lower.to_le_bytes(),
        ],
        bump
    )]
    pub tick_array_lower_account: Option<AccountLoader<'info, TickStateArray>>,

    // Just avoid to compute the PDA on chain when
    // `tick_array_lower_account` is None.
    #[account(
        seeds = [
            TICK_ARRAY_SEED.as_bytes(),
            pool_state.key().as_ref(),
            &tick_lower.to_le_bytes(),
        ],
        bump
    )]
    pub tick_array_lower_pda: UncheckedAccount<'info>,

    // Use Option<Account<...>> to check existence + validity
    // Anchor attempts deserialization. If it fails (account doesn't exist,
    // wrong owner, wrong data type), this will be None.
    // Seeds constraint ensures we're checking the *correct* PDA address.
    #[account(
        seeds = [
            TICK_ARRAY_SEED.as_bytes(),
            pool_state.key().as_ref(),
            &tick_upper.to_le_bytes(),
        ],
        bump
    )]
    pub tick_array_upper_account: Option<AccountLoader<'info, TickStateArray>>,

    // Just avoid to compute the PDA on chain when
    // `tick_array_upper_account` is None.
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
}

pub fn open_position_impl(
        ctx: Context<OpenPosition>,
        tick_lower: i32,
        tick_upper: i32,
        liquidity: u128,
        amount0_max: u64,
        amount1_max: u64,
    ) -> Result<()> {
    let pool_state = &mut ctx.accounts.pool_state.load_mut()?;
    let tick_spacing = pool_state.tick_spacing;
    crate::state::tick_index_check!(tick_lower, tick_spacing);
    crate::state::tick_index_check!(tick_upper, tick_spacing);
    require_gt!(tick_upper, tick_lower, ErrorCode::InvalidTickIndexOrder);
    require!(liquidity > 0, ErrorCode::LiquidityZero);

    let (amount_0, amount_1) = liquidity_math::get_delta_amounts_signed(
            pool_state.tick_current, 
            pool_state.sqrt_price_x64, 
            tick_lower, 
            tick_upper, liquidity as i128)?;
    require!(amount_0 <= amount0_max, ErrorCode::SlippageLimitExceeded);
    require!(amount_1 <= amount1_max, ErrorCode::SlippageLimitExceeded);
    
    if amount_0 > 0 {
        token::transfer(ctx.accounts.transfer_ctx_0(), 0)?;
    }
    if amount_1 > 0 {
        token::transfer(ctx.accounts.transfer_ctx_1(), amount_1)?;
    }

    

    todo!()
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

    pub fn load_tick_array_lower(&'info mut self, tick_lower: i32, bump: u8) -> Result<RefMut<'info, TickStateArray>> {
        self.load_tick_array(
            &mut self.tick_array_lower_account, 
            &self.tick_array_lower_pda, 
            tick_lower, 
            bump)?;
        todo!()
    }

    pub fn load_tick_array(
            &self, 
            maybe_account: &'info mut Option<AccountLoader<'info, TickStateArray>>,
            pda: &'info UncheckedAccount<'info>,
            tick:i32, bump: u8
        ) -> Result<()> {
        
        if maybe_account.is_none() {
            self.create_tick_array_account(pda, tick, bump)?;
            let account = AccountLoader::<'info, TickStateArray>::try_from_unchecked(&crate::id(), pda)?;
            *maybe_account = Some(account);
            let account = maybe_account.as_mut().unwrap();
            let mut tick_array = account.load_mut()?;
            tick_array.initialize(self.pool_state.key(), TickStateArray::start_idx_from_tick(tick));
        }
        Ok(())
    }

    pub fn create_tick_array_account(&self, pda: &UncheckedAccount<'info>, tick:i32, bump: u8) -> Result<()> {
        let space = TickStateArray::LEN + ANCHOR_SIZE;
        let rent = Rent::get()?;
        let lamports = rent.minimum_balance(space);
        if pda.lamports() == 0 {
            let instr = system_instruction::create_account(
                &self.lp.key(), &pda.key(), lamports, space as u64, &crate::id());
            invoke_signed(
            &instr, 
            &[
                self.lp.to_account_info(), 
                pda.to_account_info(), 
                self.system_program.to_account_info()], 
            &[
                &[TICK_ARRAY_SEED.as_bytes()],
                &[self.pool_state.key().as_ref()],
                &[tick.to_le_bytes().as_ref()],
                &[bump.to_le_bytes().as_ref()]]
            )?;
        } 

        Ok(())
    }
}