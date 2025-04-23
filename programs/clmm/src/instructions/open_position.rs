use anchor_lang::{prelude::*, Accounts};

use crate::{state::tick, state::PoolState};

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub lp: Signer<'info>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

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
    todo!()
}