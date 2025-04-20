pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod libraries;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FAsGDFLK4uPpSuJPJYzXx6iWR3f3w6hvtcXCsVX5maS5");

#[program]
pub mod clmm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn initialize_pool(_ctx: Context<Initialize>) -> Result<()> {
        todo!()
    }

    pub fn add_liquidity(_ctx: Context<Initialize>) -> Result<()> {
        todo!()
    }

    pub fn remove_liquidity(_ctx: Context<Initialize>) -> Result<()> {
        todo!()
    }

    pub fn swap(_ctx: Context<Initialize>) -> Result<()> {
        todo!()
    }

    pub fn collect_fees(_ctx: Context<Initialize>) -> Result<()> {
        todo!()
    }

    pub fn update_protocal_parameters(_ctx: Context<Initialize>) -> Result<()> {
        todo!()
    }

    pub fn create_position(_ctx: Context<Initialize>) -> Result<()> {
        todo!()
    }

    pub fn update_tick(_ctx: Context<Initialize>) -> Result<()> {
        todo!()
    }

    pub fn update_oracle(_ctx: Context<Initialize>) -> Result<()> {
        todo!()
    }
}
