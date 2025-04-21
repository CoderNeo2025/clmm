use anchor_lang::prelude::*;

#[constant]
pub const ANCHOR_SIZE: usize = 8;

#[constant]
pub const SEED: &str = "anchor";

#[constant]
pub const POOL_SEED: &str = "clmm_pool";

#[constant]
pub const TICK_SEED: &str = "clmm_tick";

#[constant]
pub const POSITION_SEED: &str = "clmm_position";

#[constant]
pub const TICK_ARRAY_BITMAP_SEED: &str = "tick_array_bitmap";

#[constant]
pub const FEE_RATE_DENOMINATOR_VALUE: u32 = 1_000_000;

#[constant]
pub const TICK_ARRAY_SIZE: usize = 60;

#[constant]
pub const TICK_ARRAY_BITMAP_SIZE: usize = (10240 - 128 - 8 - 32)/2/8;

#[constant]
pub const TICK_MIN: i32 = -(TICK_ARRAY_BITMAP_SIZE as i32 * TICK_ARRAY_SIZE as i32);
#[constant]
pub const TICK_MAX: i32 = -TICK_MIN;