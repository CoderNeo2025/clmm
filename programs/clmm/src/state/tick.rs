use anchor_lang::zero_copy;
use anchor_lang::prelude::*;

use crate::libraries::big_num::U256;
use crate::{TICK_ARRAY_BITMAP_SIZE, TICK_ARRAY_SIZE};


#[zero_copy]
#[repr(C, packed)]
#[derive(Default, Debug)]
pub struct TickState {
    /// The amount of liquidity added (or, if negative, removed)
    /// when the tick is crossed going left to right
    pub liquidity_net: i128,
    /// A gross tally of liquidity referencing the tick
    pub liquidity_gross: u128,
    pub fee_growth_outside0_x64: u128,
    pub fee_growth_outside1_64: u128,
    pub seconds_outside: U256,
    pub tick_cumulative_outside: U256,
    pub seconds_per_liquidity_outside_x128: U256,
}

impl TickState {
    pub const LEN: usize = 
        16 +
        16 +
        16 +
        16 +
        32 +
        32 +
        32;

}

#[account(zero_copy)]
#[repr(C, packed)]
#[derive(Debug)]
pub struct TickStateArray {
    pub pool_id: Pubkey,
    pub tick_start_idx: i32,
    pub tick_valid_cnt: u32,
    pub tick_states: [TickState; TICK_ARRAY_SIZE],
}

impl TickStateArray {
    pub const LEN: usize =
        32 +
        4 +
        4 +
        TickState::LEN * TICK_ARRAY_SIZE;
}

#[account(zero_copy)]
#[repr(C, packed)]
#[derive(Debug)]
pub struct TickStateArrayBitMap {
    pub pool_id: Pubkey,
    pub bitmap_pos: [u64; TICK_ARRAY_BITMAP_SIZE],
    pub bitmap_neg: [u64; TICK_ARRAY_BITMAP_SIZE],
}

impl TickStateArrayBitMap {
    pub const LEN: usize =
        32 +
        8 * TICK_ARRAY_BITMAP_SIZE +
        8 * TICK_ARRAY_BITMAP_SIZE;

    pub fn initialize(&mut self, pool_id: Pubkey) {
        self.pool_id = pool_id;
        self.bitmap_pos = [0; TICK_ARRAY_BITMAP_SIZE];
        self.bitmap_neg = [0; TICK_ARRAY_BITMAP_SIZE];
    }
}