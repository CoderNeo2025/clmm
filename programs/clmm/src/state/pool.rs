use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::POOL_SEED;

#[account(zero_copy)]
#[repr(C, packed)]
#[derive(Default, Debug)]
pub struct PoolState {
    pub bump: [u8; 1],
    pub token_decimals0: u8,
    pub token_decimals1: u8,
    pub tick_spacing: u16,
    pub tick_current: i32,
    pub token0: Pubkey,
    pub token1: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub fee_rate: u32, // units of hundredths of a basis point(0.0001%)
    pub protocol_fee_rate: u32,

    pub liquidity: u128,
    /// sqrt(token1/token0), Q64.64 value
    pub sqrt_price_x64: u128,
    pub fee_growth_global0_x64: u128,
    pub fee_growth_global1_x64: u128,
    pub protocol_fees0: u128,
    pub protocol_fees1: u128,
}

impl PoolState {
    pub const LEN: usize =
        1 +
        2 +
        4 +
        32 +
        32 +
        32 +
        32 +
        4 +
        4 +
        16 +
        16 +
        16 +
        16 +
        16 +
        16;

    pub fn seeds(&self) -> [&[u8]; 4] {
        [
            &POOL_SEED.as_bytes(),
            self.token0.as_ref(),
            self.token1.as_ref(),
            self.bump.as_ref(),
        ]
    }

    pub fn key(&self) -> Pubkey {
        Pubkey::create_program_address(&self.seeds(), &crate::id()).unwrap()
    }

    pub fn initialize(
        &mut self,
        bump: u8,
        tick_spacing: u16,
        fee_rate: u32,
        protocol_fee_rate: u32,
        sqrt_price_x64: u128,
        tick: i32,
        token_vault0: Pubkey,
        token_vault1: Pubkey,
        token0: &InterfaceAccount<Mint>,
        token1: &InterfaceAccount<Mint>,
    ) -> Result<()> {
        self.bump = [bump];
        self.fee_rate = fee_rate;
        self.protocol_fee_rate = protocol_fee_rate;
        self.token0 = token0.key();
        self.token1 = token1.key();
        self.token_decimals0 = token0.decimals;
        self.token_decimals1 = token1.decimals;
        self.token_vault_0 = token_vault0;
        self.token_vault_1 = token_vault1;
        self.tick_spacing = tick_spacing;
        self.tick_current = tick;
        self.sqrt_price_x64 = sqrt_price_x64;

        self.fee_growth_global0_x64 = 0;
        self.fee_growth_global1_x64 = 0;
        self.protocol_fees0 = 0;
        self.protocol_fees1 = 0;
        self.liquidity = 0;
        Ok(())
    }
}