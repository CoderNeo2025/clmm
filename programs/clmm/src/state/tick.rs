use anchor_lang::zero_copy;
use anchor_lang::prelude::*;
use solana_program::program::invoke_signed;
use solana_program::system_instruction;

use crate::constants::ANCHOR_SIZE;
use crate::constants::TICK_ARRAY_SEED;
use crate::libraries::big_num::U256;
use crate::libraries::liquidity_math;
use crate::util::AccountLoad;
use crate::error::ErrorCode;
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
    pub fee_growth_outside_0_x64: u128,
    pub fee_growth_outside_1_x64: u128,
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
    pub fn update(
        &mut self,
        pool_state: &PoolState,
        tick_index: i32, 
        liquidity_delta: i128,
        is_upper: bool,
    ) -> Result<bool> {
        let is_initializing = !self.valid();
        if is_initializing {
            require!(liquidity_delta > 0, ErrorCode::InitializeTickWithZeroOrNegLiquidity);
            if pool_state.tick_current >= tick_index {
                self.fee_growth_outside_0_x64 = pool_state.fee_growth_global_0_x64;
                self.fee_growth_outside_1_x64 = pool_state.fee_growth_global_1_x64;
            }
        }
        self.liquidity_gross = liquidity_math::add_delta(self.liquidity_gross, liquidity_delta)?;
        if is_upper {
            self.liquidity_net = self.liquidity_net.checked_sub(liquidity_delta).unwrap();
        } else {
            self.liquidity_net = self.liquidity_net.checked_add(liquidity_delta).unwrap();
        }

        if !self.valid() {
            *self = Self::default();
        }
        Ok(is_initializing)
    }

    pub fn valid(&self) -> bool {
        self.liquidity_gross != 0
    }
}

#[account(zero_copy)]
#[repr(C, packed)]
#[derive(Debug)]
pub struct TickStateArray {
    pub pool_id: Pubkey,
    pub tick_start_idx: i32,
    pub tick_valid_cnt: u32,
    pub tick_spacing: u16,
    pub tick_states: [TickState; TICK_ARRAY_SIZE],
}

impl TickStateArray {
    pub const LEN: usize =
        32 +
        4 +
        4 +
        2 +
        TickState::LEN * TICK_ARRAY_SIZE;
    pub fn initialize(&mut self, pool_id: Pubkey, start_idx: i32, tick_spacing: u16) {
        self.pool_id = pool_id;
        self.tick_start_idx = start_idx;
        self.tick_valid_cnt = 0;
        self.tick_spacing = tick_spacing;
        self.tick_states = [TickState::default(); TICK_ARRAY_SIZE];
    }

    pub fn get_or_create_tick_array<'info>(
        tick_array_account: AccountInfo<'info>,
        payer: AccountInfo<'info>,
        system_program: AccountInfo<'info>,
        pool_state_loader: &AccountLoader<'info, PoolState>,
        start_idx: i32,
        tick_spacing: u16,
        bump: u8
    ) -> Result<AccountLoad<'info, TickStateArray>> {
        let space = TickStateArray::LEN + ANCHOR_SIZE;
        let rent = Rent::get()?;
        let lamports = rent.minimum_balance(space);
        let account_lamports = tick_array_account.lamports();
        if account_lamports >= lamports {
            // TickStateArray has been created.
            AccountLoad::<'info, TickStateArray>::try_from(&tick_array_account)
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
            let loader = AccountLoad::<TickStateArray>::try_from_unchecked(
                &crate::ID, 
                &tick_array_account)?;
            loader.load_init()?
                .initialize(pool_state_loader.key().clone(), start_idx, tick_spacing);
            Ok(loader)
        }
    }

    pub fn update_tick(
        &mut self,
        pool_state: &PoolState,
        tick_index: i32, 
        liquidity_delta: i128,
        is_upper: bool,
    ) -> Result<bool> {
        let valid_before = self.valid();
        let array_index = Self::tick_index_to_array_index(
            tick_index, 
            self.tick_start_idx, 
            self.tick_spacing)?;
        let tick = &mut self.tick_states[array_index];
        let is_initializing = tick.update(pool_state, tick_index, liquidity_delta, is_upper)?;
        if is_initializing {
            self.tick_valid_cnt += 1;
        }
        if !tick.valid() {
            self.tick_valid_cnt -= 1;
        }
        let flipped = self.valid() != valid_before;
        Ok(flipped)
    }

    pub fn tick_index_to_array_index(tick_index: i32, start_index: i32, tick_spacing: u16) -> Result<usize> {
        require!(tick_index >= start_index, ErrorCode::TickLowerThanArrayStart);
        let offset = (tick_index - start_index) / (tick_spacing as i32);
        let offset = offset.try_into()?;
        require!(offset < TICK_ARRAY_SIZE, ErrorCode::TickIndexOutOfBounds);
        Ok(offset)
    }

    pub fn valid(&self) -> bool {
        self.tick_valid_cnt > 0
    }

    /// Input an arbitrary tick_index, output the start_index of the tick_array it sits on
    pub fn get_array_start_index(tick_index: i32, tick_spacing: u16) -> i32 {
        let ticks_in_array = TickStateArray::tick_count(tick_spacing);
        let mut start = tick_index / ticks_in_array;
        if tick_index < 0 && tick_index % ticks_in_array != 0 {
            start = start - 1
        }
        start * ticks_in_array
    }

    pub fn tick_count(tick_spacing: u16) -> i32 {
        TICK_ARRAY_SIZE as i32 * i32::from(tick_spacing)
    }
}

#[account(zero_copy)]
#[repr(C)]
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

    pub fn flip(&mut self, tick_index: i32, tick_spacing: u16) -> Result<()> {
        let (bitmap, (idx, bit_idx)) = if tick_index >= 0 {
            (&mut self.bitmap_pos, Self::locate_pos(tick_index, tick_spacing))
        } else {
            (&mut self.bitmap_neg, Self::locate_neg(tick_index, tick_spacing))
        };
        bitmap[idx] ^= 1u64 << bit_idx;
        Ok(())
    }

    pub fn locate_pos(tick_index: i32, tick_spacing: u16) -> (usize, usize) {
        let cnt = TickStateArray::tick_count(tick_spacing);
        let idx = tick_index / cnt;
        ((idx / 64) as usize, (idx % 64) as usize)
    }

    pub fn locate_neg(tick_index: i32, tick_spacing: u16) -> (usize, usize) {
        let cnt = TickStateArray::tick_count(tick_spacing);
        let idx = tick_index / cnt + 1;
        ((idx / 64) as usize, (idx % 64) as usize)
    }
}