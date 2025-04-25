use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use anchor_spl::token_interface::TokenAccount;
use anchor_spl::token_interface::TokenInterface;

use crate::libraries::tick_math;
use crate::state::PoolState;
use crate::state::TickStateArrayBitMap;
use crate::constants::ANCHOR_SIZE;
use crate::constants::POOL_SEED;
use crate::constants::POOL_VAULT_SEED;
use crate::constants::SQRT_PRICE_X64_MAX;
use crate::constants::SQRT_PRICE_X64_MIN;
use crate::constants::TICK_ARRAY_BITMAP_SEED;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub pool_creator: Signer<'info>,

    #[account(
        init,
        payer = pool_creator,
        space = ANCHOR_SIZE + PoolState::LEN,
        seeds = [
            POOL_SEED.as_bytes(), 
            token_0.key().as_ref(), 
            token_1.key().as_ref()
        ],
        bump
    )]
    pub pool_state: AccountLoader<'info, PoolState>,

    #[account(
        init,
        payer = pool_creator,
        space = ANCHOR_SIZE + TickStateArrayBitMap::LEN,
        seeds = [
            TICK_ARRAY_BITMAP_SEED.as_bytes(), 
            pool_state.key().as_ref()
        ],
        bump
    )]
    pub tick_array_bitmap: AccountLoader<'info, TickStateArrayBitMap>,

    #[account(
        constraint = token_0.key() < token_1.key(),
        mint::token_program = token_program_0,
    )]
    pub token_0: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mint::token_program = token_program_1
    )]
    pub token_1: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        seeds = [
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            token_0.key().as_ref(),
        ],
        bump,
        payer = pool_creator,
        token::mint = token_0,
        token::authority = pool_state,
        token::token_program = token_program_0,
    )]
    pub token_vault0: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        seeds = [
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            token_1.key().as_ref(),
        ],
        bump,
        payer = pool_creator,
        token::mint = token_1,
        token::authority = pool_state,
        token::token_program = token_program_1,
    )]
    pub token_vault_1: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program_0: Interface<'info, TokenInterface>,

    pub token_program_1: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_pool_impl(ctx: Context<InitializePool>, 
               tick_spacing: u16,sqrt_price_x64: u128,
               fee_rate: u32, protocol_fee_rate: u32) -> Result<()> {
    require!(tick_spacing > 0, ErrorCode::TickSpacingZero);
    require!(sqrt_price_x64 <= SQRT_PRICE_X64_MAX && sqrt_price_x64 >= SQRT_PRICE_X64_MIN,
            ErrorCode::SqrtPriceX64);
    let pool_state = &mut ctx.accounts.pool_state.load_init()?;
    pool_state.initialize(
        ctx.bumps.pool_state, 
        tick_spacing, 
        fee_rate, 
        protocol_fee_rate, 
        sqrt_price_x64, 
        tick_math::get_tick_at_sqrt_price(sqrt_price_x64)?, 
        ctx.accounts.token_vault0.key(), 
        ctx.accounts.token_vault_1.key(), 
        ctx.accounts.token_0.as_ref(), 
        ctx.accounts.token_1.as_ref(),
        ctx.accounts.tick_array_bitmap.key()
    )?;

    ctx.accounts
       .tick_array_bitmap
       .load_mut()?
       .initialize(pool_state.key());

    msg!("Pool for {} and {} has been created", 
          ctx.accounts.token_0.key(), 
          ctx.accounts.token_1.key());
    Ok(())
}