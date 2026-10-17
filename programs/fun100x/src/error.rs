use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow error")]
    ArithmeticOverflow,

    // ==================== Trade Calculation Overflow Errors ====================
    #[msg("Buy trade calculation overflow")]
    BuyCalculationOverflow,

    #[msg("Buy fee calculation overflow")]
    BuyFeeCalculationOverflow,

    #[msg("Sell trade calculation  overflow")]
    SellCalculationOverflow,

    #[msg("Sell fee calculation overflow")]
    SellFeeCalculationOverflow,

    // ==================== Margin Trade Overflow Errors ====================
    #[msg("Long margin calculation overflow")]
    LongMarginCalculationOverflow,

    #[msg("Long borrow calculation overflow")]
    LongBorrowCalculationOverflow,

    #[msg("Long fee calculation overflow")]
    LongFeeCalculationOverflow,

    #[msg("Long price calculation overflow")]
    LongPriceCalculationOverflow,

    #[msg("Short margin calculation overflow")]
    ShortMarginCalculationOverflow,

    #[msg("Short borrow calculation overflow")]
    ShortBorrowCalculationOverflow,

    #[msg("Short fee calculation overflow")]
    ShortFeeCalculationOverflow,

    #[msg("Short price calculation overflow")]
    ShortPriceCalculationOverflow,

    // ==================== Position Close Overflow Errors ====================
    #[msg("Close long profit calculation overflow")]
    CloseLongProfitOverflow,

    #[msg("Close long repayment calculation overflow")]
    CloseLongRepaymentOverflow,

    #[msg("Close long remaining calculation overflow")]
    CloseLongRemainingOverflow,

    #[msg("Close long fee overflow")]
    CloseLongFeeOverflow,

    #[msg("Close short profit calculation overflow")]
    CloseShortProfitOverflow,

    #[msg("Close short repayment calculation overflow")]
    CloseShortRepaymentOverflow,

    #[msg("Close short remaining calculation overflow")]
    CloseShortRemainingOverflow,

    #[msg("Close short fee overflow")]
    CloseShortFeeOverflow,

    #[msg("Close short impossible state: current_total should always be greater than remaining_close_reduced_sol_with_fee")]
    CloseShortImpossibleState,

    // ==================== Fee Management Overflow Errors ====================
    #[msg("Fee split calculation overflow")]
    FeeSplitCalculationOverflow,

    #[msg("Fee accumulation overflow")]
    FeeAccumulationOverflow,

    #[msg("Partner fee addition overflow")]
    PartnerFeeAdditionOverflow,

    #[msg("Base fee addition overflow")]
    BaseFeeAdditionOverflow,

    #[msg("Pool fee deduction overflow")]
    PoolFeeDeductionOverflow,

    #[msg("Fee random discount calculation overflow")]
    FeeRandomDiscountOverflow,

    // ==================== Liquidity Management Overflow Errors ====================
    #[msg("SOL reserve addition overflow")]
    SolReserveAdditionOverflow,

    #[msg("SOL reserve deduction overflow")]
    SolReserveDeductionOverflow,

    #[msg("Token reserve addition overflow")]
    TokenReserveAdditionOverflow,

    // ==================== Transfer Operation Overflow Errors ====================
    #[msg("Lamports addition overflow")]
    LamportsAdditionOverflow,

    #[msg("Lamports deduction overflow")]
    LamportsDeductionOverflow,

    // ==================== Time and Counter Overflow Errors ====================
    #[msg("Deadline calculation overflow")]
    DeadlineCalculationOverflow,

    #[msg("Fee discount flag calculation overflow")]
    FeeDiscountFlagOverflow,

    // ==================== Permission and Authorization Errors ====================
    #[msg("Unauthorized operation")]
    Unauthorized,

    #[msg("All parameters are required during initialization")]
    RequiredParameter,

    // ==================== Curve Calculation Errors ====================
    #[msg("Curve calculation error")]
    CurveCalculationError,

    #[msg("Initial price calculation failed")]
    InitialPriceCalculationError,

    #[msg("Reserve recalculation failed (after buy)")]
    BuyReserveRecalculationError,

    #[msg("Reserve recalculation failed (after sell)")]
    SellReserveRecalculationError,

    #[msg("Total amount with fee calculation failed")]
    TotalAmountWithFeeError,

    #[msg("Amount after fee calculation failed")]
    AmountAfterFeeError,

    #[msg("Buy price range calculation failed")]
    BuyPriceRangeCalculationError,

    #[msg("Sell price range calculation failed")]
    SellPriceRangeCalculationError,

    #[msg("Price has reached stop-loss boundary, head order must be liquidated first")]
    PriceAtStopLossBoundary,

    #[msg("Remaining range calculation failed")]
    RemainingRangeCalculationError,

    #[msg("Full range calculation failed")]
    FullRangeCalculationError,

    #[msg("Curve function returned None: buy_from_price_with_token_output")]
    BuyFromPriceWithTokenNoneError,

    #[msg("Curve function returned None: sell_from_price_with_token_input")]
    SellFromPriceWithTokenNoneError,

    // ==================== Trade Amount and Limit Errors ====================
    #[msg("Required SOL amount exceeds user-set maximum")]
    ExceedsMaxSolAmount,

    #[msg("Insufficient SOL output")]
    InsufficientSolOutput,

    #[msg("Insufficient proceeds to repay loan")]
    InsufficientRepayment,

    #[msg("Insufficient borrowing reserve")]
    InsufficientBorrowingReserve,

    #[msg("Insufficient token sale amount")]
    InsufficientTokenSale,

    #[msg("Insufficient liquidity in current order")]
    InsufficientLiquidity,

    #[msg("Insufficient market liquidity, cannot satisfy trade even after liquidating all stop-loss orders")]
    InsufficientMarketLiquidity,

    // ==================== Margin Trade Errors ====================
    #[msg("Token amount difference exceeds valid range in margin trade")]
    TokenAmountDifferenceOutOfRange,

    #[msg("Borrow amount mismatch with locked tokens")]
    BorrowAmountMismatch,

    #[msg("Close fee calculation error")]
    CloseFeeCalculationError,

    #[msg("Insufficient margin")]
    InsufficientMargin,

    #[msg("Margin below minimum requirement")]
    InsufficientMinimumMargin,

    #[msg("Payer wallet balance insufficient for long margin and fees")]
    InsufficientLongPayerBalance,

    #[msg("Payer wallet balance insufficient for short margin and fees")]
    InsufficientShortPayerBalance,

    // ==================== Account Validation Errors ====================
    #[msg("Invalid account owner")]
    InvalidAccountOwner,

    #[msg("Sell amount exceeds order's token holdings")]
    SellAmountExceedsOrderAmount,

    // ==================== Order Permission Errors ====================
    #[msg("Non-expired order must be closed by owner")]
    OrderNotExpiredMustCloseByOwner,

    #[msg("Settlement address must be owner address")]
    SettlementAddressMustBeOwnerAddress,

    #[msg("Buy amount exceeds order's token holdings")]
    BuyAmountExceedsOrderAmount,

    // ==================== Trade Requirement Errors ====================
    #[msg("Trade amount below minimum requirement")]
    InsufficientTradeAmount,

    // #[msg("Trade amount exceeds maximum limit (30% of pool reserve)")]
    // ExcessiveTradeAmount,
    #[msg("SOL amount exceeds maximum limit (10000000 SOL per transaction)")]
    SolAmountTooLarge,

    #[msg("Remaining token amount below minimum trade requirement")]
    RemainingTokenAmountTooSmall,

    // ==================== Cooldown and Approval Errors ====================
    #[msg("Trade cooldown period not expired, please try again later")]
    TradeCooldownNotExpired,

    #[msg("Sell amount exceeds approved amount, please call approval function first")]
    ExceedApprovalAmount,

    #[msg("Sell trade requires calling approval or buy function first to initialize cooldown PDA")]
    CooldownNotInitialized,

    #[msg("Cannot close cooldown PDA with non-zero token balance")]
    CannotCloseCooldownWithBalance,

    // ==================== Price and Fee Calculation Errors ====================
    #[msg("Price calculation error")]
    PriceCalculationError,

    #[msg("Invalid partner fee recipient account")]
    InvalidPartnerFeeRecipientAccount,

    #[msg("Invalid base fee recipient account")]
    InvalidBaseFeeRecipientAccount,

    #[msg("Orderbook address does not match curve account orderbook")]
    InvalidOrderbookAddress,

    // ==================== Parameter Validation Errors ====================
    #[msg("Fee percentage must be between 0-100")]
    InvalidFeePercentage,

    #[msg("Fee rate exceeds maximum limit (10%)")]
    InvalidFeeRate,

    #[msg("Custom fee rate must be between 1000 (1%) and 5000 (5%)")]
    InvalidCustomFeeRate,

    #[msg("Borrow duration out of valid range (3-30 days)")]
    InvalidBorrowDuration,

    #[msg("Stop loss price does not meet minimum interval requirement")]
    InvalidStopLossPrice,

    // ==================== Pool Funds Errors ====================
    #[msg("No profitable funds to transfer")]
    NoProfitableFunds,

    #[msg("Insufficient pool funds")]
    InsufficientPoolFunds,

    #[msg("Pool SOL account balance would fall below rent-exempt minimum")]
    InsufficientPoolBalance,

    // ==================== OrderBook Manager Errors ====================
    #[msg("Math operation overflow")]
    OrderBookManagerOverflow,

    #[msg("Invalid slot index")]
    OrderBookManagerInvalidSlotIndex,

    #[msg("Invalid account data")]
    OrderBookManagerInvalidAccountData,

    #[msg("New capacity exceeds maximum limit")]
    OrderBookManagerExceedsMaxCapacity,

    #[msg("Account size exceeds 10MB limit")]
    OrderBookManagerExceedsAccountSizeLimit,

    #[msg("Order ID mismatch")]
    OrderBookManagerOrderIdMismatch,

    #[msg("Order book is empty")]
    OrderBookManagerEmptyOrderBook,

    #[msg("Account is not writable")]
    OrderBookManagerAccountNotWritable,

    #[msg("Account not rent-exempt")]
    OrderBookManagerNotRentExempt,

    #[msg("Invalid rent balance")]
    OrderBookManagerInvalidRentBalance,

    #[msg("Insufficient funds")]
    OrderBookManagerInsufficientFunds,

    #[msg("OrderBook account owner mismatch")]
    OrderBookManagerInvalidAccountOwner,

    #[msg("Data access out of bounds")]
    OrderBookManagerDataOutOfBounds,

    // ==================== Long/Short Order Insert Errors ====================
    #[msg("Cannot find valid insert position, all candidates failed due to price range overlap")]
    NoValidInsertPosition,

    #[msg("close_insert_indices array cannot be empty")]
    EmptyCloseInsertIndices,

    #[msg("close_insert_indices array cannot exceed 20 elements")]
    TooManyCloseInsertIndices,

    #[msg("Specified close order not found")]
    CloseOrderNotFound,

    #[msg("Linked list delete count mismatch: count inconsistent before/after deletion")]
    LinkedListDeleteCountMismatch,

    // ==================== Token Metadata Validation Errors ====================
    #[msg("Token name too long, max 32 bytes")]
    NameTooLong,

    #[msg("Token name cannot be empty")]
    NameEmpty,

    #[msg("Token symbol too long, max 10 bytes")]
    SymbolTooLong,

    #[msg("Token symbol cannot be empty")]
    SymbolEmpty,

    #[msg("URI too long, max 200 bytes")]
    UriTooLong,

    #[msg("URI cannot be empty")]
    UriEmpty,

    // ==================== Advanced Pool Configuration Errors ====================
    #[msg("Incomplete advanced pool parameters: custom_lp_sol, custom_lp_token, custom_borrow_ratio, custom_borrow_duration must be provided together")]
    IncompleteAdvancedPoolParams,

    #[msg("Initial virtual SOL out of valid range")]
    InvalidInitialVirtualSol,

    #[msg("Initial virtual Token out of valid range")]
    InvalidInitialVirtualToken,

    #[msg("Borrow pool ratio out of valid range")]
    InvalidBorrowPoolRatio,

    #[msg("Borrow pool token amount calculation overflow")]
    BorrowTokenCalculationOverflow,

    #[msg("Borrow pool token amount cannot be zero")]
    BorrowTokenAmountZero,
}
