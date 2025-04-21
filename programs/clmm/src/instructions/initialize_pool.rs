use anchor_lang::prelude::*;

use crate::libraries::tick_math;
use crate::PoolState;
use crate::TickStateArrayBitMap;
use crate::ANCHOR_SIZE;
use crate::POOL_SEED;
use crate::SQRT_PRICE_X64_MAX;
use crate::SQRT_PRICE_X64_MIN;
use crate::TICK_ARRAY_BITMAP_SEED;
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(token0: Pubkey, token1: Pubkey)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = ANCHOR_SIZE + PoolState::LEN,
        seeds = [POOL_SEED.as_bytes(), token0.as_ref(), token1.as_ref()],
        bump
    )]
    pub pool_state: AccountLoader<'info, PoolState>,

    #[account(
        init,
        payer = signer,
        space = ANCHOR_SIZE + TickStateArrayBitMap::LEN,
        seeds = [
            TICK_ARRAY_BITMAP_SEED.as_bytes(), 
            pool_state.key().as_ref()
        ],
        bump
    )]
    pub tick_array_bitmap: AccountLoader<'info, TickStateArrayBitMap>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_pool_impl(ctx: Context<InitializePool>, tick_spacing: u16,
               token0: Pubkey, token1: Pubkey, sqrt_price_x64: u128,
               fee_ratio: u32, protocol_fee: u32) -> Result<()> {
    require!(tick_spacing > 0, ErrorCode::TickSpacingZero);
    require!(token0 < token1, ErrorCode::TokenPairOrder);
    require!(sqrt_price_x64 <= SQRT_PRICE_X64_MAX && sqrt_price_x64 >= SQRT_PRICE_X64_MIN,
            ErrorCode::SqrtPriceX64);
    let pool_state = &mut ctx.accounts.pool_state.load_init()?;
    pool_state.bump = ctx.bumps.pool_state;
    pool_state.tick_spacing = tick_spacing;
    pool_state.token0 = token0;
    pool_state.token1 = token1;
    pool_state.fee_rate = fee_ratio;
    pool_state.protocol_fee_ratio = protocol_fee;
    pool_state.sqrt_price_x64 = sqrt_price_x64;
    pool_state.tick_current = tick_math::get_tick_at_sqrt_price(sqrt_price_x64)?;

    pool_state.fee_growth_global0_x64 = 0;
    pool_state.fee_growth_global1_x64 = 0;
    pool_state.protocol_fees0 = 0;
    pool_state.protocol_fees1 = 0;
    pool_state.liquidity = 0;

    msg!("Pool for {} and {} has been created", token0, token1);
    Ok(())
}