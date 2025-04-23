use anchor_lang::{prelude::*, Accounts};

use crate::PoolState;

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
    ) -> Result<()> {
    todo!()
}