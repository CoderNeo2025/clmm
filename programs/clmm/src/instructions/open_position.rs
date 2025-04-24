use anchor_lang::{prelude::*, Accounts};

use crate::constants::{ANCHOR_SIZE, POSITION_SEED};
use crate::libraries::liquidity_math;
use crate::state::{PoolState, PositionState};
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

    pub system_program: Program<'info, System>,
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

    let (amount0, amount1) = liquidity_math::get_delta_amounts_signed(
            pool_state.tick_current, 
            pool_state.sqrt_price_x64, 
            tick_lower, 
            tick_upper, liquidity as i128)?;
    require!(amount0 <= amount0_max, ErrorCode::SlippageLimitExceeded);
    require!(amount1 <= amount1_max, ErrorCode::SlippageLimitExceeded);


    todo!()
}