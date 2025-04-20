use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

#[constant]
pub const POOL_SEED: &str = "clmm_pool";

#[constant]
pub const TICK_SEED: &str = "clmm_tick";

#[constant]
pub const POSITION_SEED: &str = "clmm_position";

#[constant]
pub const FEE_RATE_DENOMINATOR_VALUE: u32 = 1_000_000;

#[constant]
pub const TICK_ARRAY_SIZE: usize = 50;

#[constant]
pub const TICK_ARRAY_BITMAP_SIZE: usize = (10240 - 128 - 8 - 32)/2/16;