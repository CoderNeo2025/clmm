use anchor_lang::zero_copy;
use anchor_lang::prelude::*;
use solana_program::program::invoke_signed;
use solana_program::system_instruction;

use crate::constants::ANCHOR_SIZE;
use crate::constants::TICK_ARRAY_SEED;
use crate::libraries::big_num::U256;
use crate::{constants::TICK_ARRAY_BITMAP_SIZE, constants::TICK_ARRAY_SIZE};

use super::PoolState;

#[macro_export]
macro_rules! tick_index_check{
    ($tick: expr, $spacing: expr) => {
        require!(
            ($tick).abs()%(($spacing) as i32) == 0 &&
            ($tick).abs() <= crate::constants::TICK_MAX, 
            crate::error::ErrorCode::InvalidTickIndex);
        
    }
}

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
    pub fn initialize(&mut self, pool_id: Pubkey, start_idx: i32) {
        self.pool_id = pool_id;
        self.tick_start_idx = start_idx;
        self.tick_valid_cnt = 0;
        self.tick_states = [TickState::default(); TICK_ARRAY_SIZE];
    }

    pub fn start_idx_from_tick(tick: i32) -> i32 {
        if tick >= 0 {
            tick / (TICK_ARRAY_SIZE as i32)
        } else {
            tick / (TICK_ARRAY_SIZE as i32) - 1
        }
    }

    pub fn get_or_create_tick_array<'info>(
        tick_array_account: &'info mut AccountInfo<'info>,
        payer: AccountInfo<'info>,
        system_program: AccountInfo<'info>,
        pool_state_loader: &AccountLoader<'info, PoolState>,
        start_idx: i32,
        bump: u8
    ) -> Result<AccountLoader<'info, TickStateArray>> {
        let space = TickStateArray::LEN + ANCHOR_SIZE;
        let rent = Rent::get()?;
        let lamports = rent.minimum_balance(space);
        let account_lamports = tick_array_account.lamports();
        if account_lamports >= lamports {
            // TickStateArray has been created.
            return AccountLoader::<'info, TickStateArray>::try_from(tick_array_account);
        } else if account_lamports < lamports && account_lamports > 0 {
            unreachable!("TickStateArray's space is fixed. This condition should never be true.")
        } else { // == 0
            let instr = system_instruction::create_account(
                &payer.key(), 
                &tick_array_account.key(), 
                lamports, 
                space as u64, 
                &crate::id());
            invoke_signed(
            &instr, 
            &[
                payer, 
                tick_array_account.clone(), 
                system_program.to_account_info()], 
            &[
                &[TICK_ARRAY_SEED.as_bytes()],
                &[pool_state_loader.key().as_ref()],
                &[start_idx.to_le_bytes().as_ref()],
                &[bump.to_le_bytes().as_ref()]]
            )?;

            
        }
        todo!()
    }
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