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
    #[msg("tick_spacing must be greater than 0.")]
    TickSpacingZero,
    #[msg("token pair must be sorted.")]
    TokenPairOrder,
    #[msg("Invalid tick index value")]
    InvalidTickIndex,
    #[msg("Invalid tick index order: tick_lower should be smaller than tick_upper")]
    InvalidTickIndexOrder,

    // Liquidity Sub
    #[msg("Liquidity sub delta L must be smaller than before")]
    LiquiditySubValueErr,
    // Liquidity Add
    #[msg("Liquidity add delta L must be greater, or equal to before")]
    LiquidityAddValueErr,
    #[msg("Invaild liquidity when update position")]
    InvaildLiquidity,
    #[msg("Both token amount must not be zero while supply liquidity")]
    ForbidBothZeroForSupplyLiquidity,
    #[msg("Liquidity insufficient")]
    LiquidityInsufficient,
    #[msg("Liquidity can't be zero when open position")]
    LiquidityZero,

    #[msg("Max token overflow")]
    MaxTokenOverflow,
    #[msg("Slippage limit is exceeded")]
    SlippageLimitExceeded,
}
