use anchor_lang::prelude::*;

#[constant]
pub const ANCHOR_SIZE: usize = 8;

#[constant]
pub const SEED: &str = "anchor";

#[constant]
pub const POOL_SEED: &str = "clmm_pool";

#[constant]
pub const POOL_VAULT_SEED: &str = "token_vault";

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
pub const TICK_ARRAY_BITMAP_SIZE: usize = 128;

#[constant]
pub const TICK_MAX: i32 = 443636;
#[constant]
pub const TICK_MIN: i32 = -443636;

/// The minimum value that can be returned from #get_sqrt_price_at_tick. Equivalent to get_sqrt_price_at_tick(TICK_MIN)
#[constant]
pub const SQRT_PRICE_X64_MIN: u128 = 4295048016;
/// The maximum value that can be returned from #get_sqrt_price_at_tick. Equivalent to get_sqrt_price_at_tick(TICK_MAX)
#[constant]
pub const SQRT_PRICE_X64_MAX: u128 = 79226673521066979257578248091;