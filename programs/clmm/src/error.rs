use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The tick must be greater, or equal to the minimum tick(-443636)")]
    TickLowerOverflow,
    #[msg("The tick must be lesser than, or equal to the maximum tick(443636)")]
    TickUpperOverflow,
    // second inequality must be < because the price can never reach the price at the max tick
    #[msg("sqrt_price_x64 out of range")]
    SqrtPriceX64,
}
