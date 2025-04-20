use anchor_lang::prelude::*;

use crate::libraries::big_num::U256;
use crate::PoolState;
use crate::POOL_SEED;

#[derive(Accounts)]
#[instruction(token0: Pubkey, token1: Pubkey)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + PoolState::INIT_SPACE,
        seeds = [POOL_SEED.as_bytes(), token0.as_ref(), token1.as_ref()],
        bump
    )]
    pub pool_state: AccountLoader<'info, PoolState>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_pool_impl(ctx: Context<InitializePool>, tick_spacing: u16,
               token0: Pubkey, token1: Pubkey, 
               fee_ratio: u32, protocol_fee: u32) -> Result<()> {
    let pool_state = &mut ctx.accounts.pool_state.load_init()?;
    pool_state.bump = ctx.bumps.pool_state;
    pool_state.tick_spacing = tick_spacing;
    pool_state.token0 = token0;
    pool_state.token1 = token1;
    pool_state.fee_rate = fee_ratio;
    pool_state.protocol_fee_ratio = protocol_fee;

    pool_state.fee_growth_global0 = U256::zero();
    pool_state.fee_growth_global1 = U256::zero();
    pool_state.protocol_fees0 = 0;
    pool_state.protocol_fees1 = 0;
    pool_state.liquidity = 0;
    pool_state.tick_current = 0;
    
    Ok(())
}