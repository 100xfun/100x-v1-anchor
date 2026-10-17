#![allow(unused_imports, unused_variables, unused_assignments)]

// Import required modules and dependencies
use {
    // Import constant definitions
    crate::constants::{
        MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH, POOL_SOL_INITIAL_ROUNDING_BUFFER,
    },
    // Import curve AMM module
    crate::curve::curve_amm::CurveAMM,
    // Import error types
    crate::error::ErrorCode,
    // Import parameter structures and account structures
    crate::instructions::contexts::CreateToken,
    crate::instructions::events::TokenCreatedEvent,
    crate::instructions::utils::pool_sol_required_balance,
    // Import Anchor framework base components
    anchor_lang::prelude::*,
    // Import system program related functionality
    anchor_lang::solana_program::program::invoke,
    // Import Anchor support for SPL token standard
    anchor_spl::token::{
        mint_to, set_authority, spl_token::instruction::AuthorityType, MintTo, SetAuthority,
    },
    // Import Metaplex Token Metadata core components
    mpl_token_metadata::instructions::{
        CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs,
    },
    mpl_token_metadata::types::DataV2,
};

// Handler function for create basic token instruction
pub fn create_token(
    ctx: Context<CreateToken>,
    // Token name
    name: String,
    // Token symbol
    symbol: String,
    // Token metadata URI
    uri: String,
    // ============ New parameters: Advanced pool configuration (all using Option type) ============
    // Custom liquidity pool SOL amount (lamports, 9 decimal places)
    // - None: Use default value (standard version)
    // - Some(value): Create advanced pool, must be within MIN_INITIAL_VIRTUAL_SOL ~ MAX_INITIAL_VIRTUAL_SOL range
    custom_lp_sol: Option<u64>,
    // Custom total token supply (smallest unit, 9 decimal places)
    // - None: Use default value (standard version)
    // - Some(value): Create advanced pool, must be within MIN_INITIAL_VIRTUAL_TOKEN ~ MAX_INITIAL_VIRTUAL_TOKEN range
    // - This is the on-chain mint total supply; the borrow pool share is deducted from it,
    //   and the remainder goes to the liquidity pool (= curve virtual token reserve)
    custom_lp_token: Option<u64>,
    // Custom borrow pool token ratio (2-8, representing 2%-8%, deducted from total supply)
    // - None: Use default value 4% (standard version)
    // - Some(value): Create advanced pool, must be within MIN_BORROW_POOL_RATIO ~ MAX_BORROW_POOL_RATIO range
    custom_borrow_ratio: Option<u8>,
    // Custom borrow duration (unit: seconds)
    // - None: Use default value from Params account (standard version)
    // - Some(value): Create advanced pool, must be within MIN_BORROW_DURATION ~ MAX_BORROW_DURATION range (3 days - 6 months)
    custom_borrow_duration: Option<u32>,
    // Custom fee rate (unit: basis points, range 1000-5000 representing 1%-5%)
    // - None: Use default value from Params account (standard version)
    // - Some(value): Custom pool, must be within MIN_CUSTOM_FEE ~ MAX_CUSTOM_FEE range
    // - When any of custom_lp_sol, custom_lp_token, custom_borrow_ratio, custom_borrow_duration is provided,
    //   custom_fee MUST be Some, otherwise returns error
    custom_fee: Option<u16>,
) -> Result<()> {
    // ======== Parameter validation ========
    // Validate name
    require!(!name.is_empty(), ErrorCode::NameEmpty);
    require!(name.len() <= MAX_NAME_LENGTH, ErrorCode::NameTooLong);

    // Validate symbol - allow Unicode characters
    require!(!symbol.is_empty(), ErrorCode::SymbolEmpty);
    require!(symbol.len() <= MAX_SYMBOL_LENGTH, ErrorCode::SymbolTooLong);

    // Validate uri
    require!(!uri.is_empty(), ErrorCode::UriEmpty);
    require!(uri.len() <= MAX_URI_LENGTH, ErrorCode::UriTooLong);

    // ======== Custom fee parameter validation ========
    let (actual_swap_fee, actual_borrow_fee) = if let Some(fee) = custom_fee {
        // Validate fee is within range
        require!(
            fee >= crate::constants::MIN_CUSTOM_FEE && fee <= crate::constants::MAX_CUSTOM_FEE,
            ErrorCode::InvalidCustomFeeRate
        );

        // Calculate borrow fee with 20% uplift
        let borrow_fee = (fee as u32)
            .checked_mul(crate::constants::CUSTOM_FEE_BORROW_MULTIPLIER)
            .ok_or(ErrorCode::ArithmeticOverflow)?
            .checked_div(crate::constants::BORROW_MULTIPLIER_DENOMINATOR)
            .ok_or(ErrorCode::ArithmeticOverflow)?
            .try_into()
            .map_err(|_| ErrorCode::ArithmeticOverflow)?;

        (fee, borrow_fee)
    } else {
        // Use default values from Params account (only allowed in standard pool mode)
        (
            ctx.accounts.params.base_swap_fee,
            ctx.accounts.params.base_borrow_fee,
        )
    };

    // ======== New: Advanced pool parameter validation and normalization ========

    // Determine pool type: if all five Option parameters are None, it's standard version; otherwise advanced version
    let is_advanced_pool = custom_lp_sol.is_some()
        || custom_lp_token.is_some()
        || custom_borrow_ratio.is_some()
        || custom_borrow_duration.is_some()
        || custom_fee.is_some();

    // Normalize parameters: set actual values based on pool type
    let (
        actual_pool_type,
        actual_initial_virtual_sol,
        actual_initial_virtual_token,
        actual_borrow_pool_ratio,
        actual_borrow_duration,
        actual_lp_token_amount,
        actual_borrow_token_amount,
    ) = if is_advanced_pool {
        // ========== Advanced pool ==========

        // Validate: all five parameters must be provided together (cannot provide only partial parameters)
        require!(
            custom_lp_sol.is_some()
                && custom_lp_token.is_some()
                && custom_borrow_ratio.is_some()
                && custom_borrow_duration.is_some()
                && custom_fee.is_some(),
            ErrorCode::IncompleteAdvancedPoolParams
        );

        // Unwrap parameters
        let sol_value = custom_lp_sol.unwrap();
        let token_value = custom_lp_token.unwrap();
        let ratio_value = custom_borrow_ratio.unwrap();
        let duration_value = custom_borrow_duration.unwrap();

        // Validate: custom_lp_sol is within valid range
        require!(
            sol_value >= crate::constants::MIN_INITIAL_VIRTUAL_SOL
                && sol_value <= crate::constants::MAX_INITIAL_VIRTUAL_SOL,
            ErrorCode::InvalidInitialVirtualSol
        );

        // Validate: custom_lp_token (total token supply) is within valid range
        // Note: the range check is applied to the TOTAL supply (user-facing, intuitive);
        // the curve virtual reserve will be slightly smaller after the borrow pool deduction
        require!(
            token_value >= crate::constants::MIN_INITIAL_VIRTUAL_TOKEN
                && token_value <= crate::constants::MAX_INITIAL_VIRTUAL_TOKEN,
            ErrorCode::InvalidInitialVirtualToken
        );

        // Validate: custom_borrow_ratio is within valid range
        require!(
            ratio_value >= crate::constants::MIN_BORROW_POOL_RATIO
                && ratio_value <= crate::constants::MAX_BORROW_POOL_RATIO,
            ErrorCode::InvalidBorrowPoolRatio
        );

        // Validate: custom_borrow_duration is within valid range
        require!(
            duration_value >= crate::constants::MIN_BORROW_DURATION
                && duration_value <= crate::constants::MAX_BORROW_DURATION,
            ErrorCode::InvalidBorrowDuration
        );

        // Calculate liquidity pool and borrow pool token amounts
        // The borrow pool share is deducted from the total supply (token_value = total supply):
        //                  so the on-chain mint total supply exactly equals token_value)
        let total_supply = token_value;
        let borrow_token: u64 = (total_supply as u128)
            .checked_mul(ratio_value as u128)
            .ok_or(ErrorCode::BorrowTokenCalculationOverflow)?
            .checked_div(100)
            .ok_or(ErrorCode::BorrowTokenCalculationOverflow)?
            .try_into()
            .map_err(|_| ErrorCode::BorrowTokenCalculationOverflow)?;
        let lp_token = total_supply
            .checked_sub(borrow_token)
            .ok_or(ErrorCode::BorrowTokenCalculationOverflow)?;

        // Validate: borrow pool token amount cannot be 0
        require!(borrow_token > 0, ErrorCode::BorrowTokenAmountZero);

        (
            crate::constants::POOL_TYPE_ADVANCED,
            sol_value,
            lp_token, // Curve virtual token reserve = liquidity pool share (keeps virtual == physical)
            ratio_value,
            duration_value,
            lp_token,
            borrow_token,
        )
    } else {
        // ========== Standard pool (using default values) ==========
        // Note: Standard version's borrow duration uses Params.base_borrow_duration (from Params account)
        // instead of DEFAULT_BORROW_DURATION constant, which differs from other parameters
        (
            crate::constants::POOL_TYPE_STANDARD,
            crate::constants::DEFAULT_INITIAL_VIRTUAL_SOL,
            crate::constants::DEFAULT_INITIAL_VIRTUAL_TOKEN,
            crate::constants::DEFAULT_BORROW_POOL_RATIO,
            ctx.accounts.params.base_borrow_duration,
            crate::constants::STANDARD_LP_TOKEN_RESERVE,
            crate::constants::STANDARD_BORROW_TOKEN_RESERVE,
        )
    };

    // msg!("Creating pool type: {}", if actual_pool_type == 0 { "Standard" } else { "Advanced" });
    // msg!("Initial virtual SOL: {} lamports", actual_initial_virtual_sol);
    // msg!("Initial virtual Token: {}", actual_initial_virtual_token);
    // msg!("Borrow pool ratio: {}%", actual_borrow_pool_ratio);
    // msg!("Borrow duration: {} seconds ({} days)", actual_borrow_duration, actual_borrow_duration / 86400);
    // msg!("Liquidity pool token amount: {}", actual_lp_token_amount);
    // msg!("Borrow pool token amount: {}", actual_borrow_token_amount);

    // Output log message, visible in transaction logs
    // msg!("Creating basic token");
    // msg!("Name: {}", name);
    // msg!("Symbol: {}", symbol);
    // msg!("URI: {}", uri);

    // Output mint account address
    // msg!("Mint account address: {}", ctx.accounts.mint_account.key());
    // msg!("Mint authority: {}", ctx.accounts.curve_account.key());

    // Get borrowing liquidity pool account address for log output
    let curve_address = ctx.accounts.curve_account.key();

    // Create a new BorrowingBondingCurve account for managing the funding pool
    // Initialize all values to 0
    ctx.accounts.curve_account.lp_token_reserve = 0;
    ctx.accounts.curve_account.lp_sol_reserve = 0;
    ctx.accounts.curve_account.price = 0;
    ctx.accounts.curve_account.borrow_token_reserve = 0;
    ctx.accounts.curve_account.borrow_sol_reserve = 0;

    // Read fee rate settings from params account
    ctx.accounts.curve_account.swap_fee = actual_swap_fee;
    ctx.accounts.curve_account.borrow_fee = actual_borrow_fee;
    ctx.accounts.curve_account.fee_discount_flag = 0; // Original price
    ctx.accounts.curve_account.borrow_duration = ctx.accounts.params.base_borrow_duration;
    //ctx.accounts.curve_account.borrow_start_price_deviation = ctx.accounts.params.base_borrow_start_price_deviation;
    // Set fee recipient accounts
    ctx.accounts.curve_account.base_fee_recipient = ctx.accounts.params.base_fee_recipient;
    ctx.accounts.curve_account.fee_recipient = ctx.accounts.params.fee_recipient;
    ctx.accounts.curve_account.fee_split = ctx.accounts.params.fee_split;

    ctx.accounts.curve_account.mint = ctx.accounts.mint_account.key();
    ctx.accounts.curve_account.creator = ctx.accounts.payer.key(); // Set token creator address

    // msg!("Borrowing liquidity pool account address: {}", curve_address);
    // msg!("Set swap fee rate: {}", ctx.accounts.curve_account.swap_fee);
    // msg!("Set borrow fee rate: {}", ctx.accounts.curve_account.borrow_fee);
    // msg!(
    //     "Set fee recipient account: {}",
    //     ctx.accounts.curve_account.fee_recipient
    // );

    // Get curve_account's bump value
    let (_, curve_bump) = Pubkey::find_program_address(
        &[b"borrowing_curve", ctx.accounts.mint_account.key().as_ref()],
        &ctx.program_id,
    );

    // Initialize liquidity pool using normalized parameter values
    ctx.accounts.curve_account.lp_token_reserve = actual_lp_token_amount;
    ctx.accounts.curve_account.mint = ctx.accounts.mint_account.key();

    // Set new orderbook addresses
    ctx.accounts.curve_account.up_orderbook = ctx.accounts.up_orderbook.key();
    ctx.accounts.curve_account.down_orderbook = ctx.accounts.down_orderbook.key();

    // Initialize up_orderbook (short order book)
    {
        let mut up_orderbook_data = ctx.accounts.up_orderbook.load_init()?;
        up_orderbook_data.version = crate::instructions::structs::OrderBook::CURRENT_VERSION;
        up_orderbook_data.order_type = 2; // 2=short/Up
        up_orderbook_data.bump = ctx.bumps.up_orderbook;
        up_orderbook_data.authority = ctx.accounts.payer.key();
        up_orderbook_data.order_id_counter = 0;
        up_orderbook_data.created_at = Clock::get()?.unix_timestamp;
        up_orderbook_data.last_modified = Clock::get()?.unix_timestamp;
        up_orderbook_data.total_capacity = 0;
        up_orderbook_data.head = u16::MAX; // Empty linked list
        up_orderbook_data.tail = u16::MAX; // Empty linked list
        up_orderbook_data.total = 0;
    }

    // Initialize down_orderbook (long order book)
    {
        let mut down_orderbook_data = ctx.accounts.down_orderbook.load_init()?;
        down_orderbook_data.version = crate::instructions::structs::OrderBook::CURRENT_VERSION;
        down_orderbook_data.order_type = 1; // 1=long/Down
        down_orderbook_data.bump = ctx.bumps.down_orderbook;
        down_orderbook_data.authority = ctx.accounts.payer.key();
        down_orderbook_data.order_id_counter = 0;
        down_orderbook_data.created_at = Clock::get()?.unix_timestamp;
        down_orderbook_data.last_modified = Clock::get()?.unix_timestamp;
        down_orderbook_data.total_capacity = 0;
        down_orderbook_data.head = u16::MAX; // Empty linked list
        down_orderbook_data.tail = u16::MAX; // Empty linked list
        down_orderbook_data.total = 0;
    }

    // // Old linked list heads (kept for deletion)
    // ctx.accounts.curve_account.up_head = None;
    // ctx.accounts.curve_account.down_head = None;

    // Mint to liquidity pool token account (using normalized amount)
    msg!(
        "Minting to liquidity pool token account: {} tokens",
        actual_lp_token_amount
    );
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.pool_token_account.to_account_info(),
                authority: ctx.accounts.curve_account.to_account_info(),
            },
            &[&[
                b"borrowing_curve",
                ctx.accounts.mint_account.key().as_ref(),
                &[curve_bump],
            ]],
        ),
        actual_lp_token_amount,
    )?;

    // Update liquidity pool token balance
    ctx.accounts.curve_account.lp_token_reserve = actual_lp_token_amount;

    // Mint to borrow pool dedicated token account (using normalized amount)
    msg!(
        "Minting to borrow pool token account: {} tokens",
        actual_borrow_token_amount
    );
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.pool_borrow_token_account.to_account_info(),
                authority: ctx.accounts.curve_account.to_account_info(),
            },
            &[&[
                b"borrowing_curve",
                ctx.accounts.mint_account.key().as_ref(),
                &[curve_bump],
            ]],
        ),
        actual_borrow_token_amount,
    )?;

    // Update borrow pool token balance
    ctx.accounts.curve_account.borrow_token_reserve = actual_borrow_token_amount;
    // Permanently record borrow pool initial funding (never changes after creation)
    ctx.accounts.curve_account.initial_borrow_token_reserve = actual_borrow_token_amount;
    // Update virtual borrow pool SOL amount - 10 million SOL will never be exhausted
    ctx.accounts.curve_account.borrow_sol_reserve = 10000000000000000;
    // Update virtual liquidity pool SOL amount (using normalized value)
    ctx.accounts.curve_account.lp_sol_reserve = actual_initial_virtual_sol;

    // ============ Initialize virtual liquidity pool parameters (using normalized values) ============
    ctx.accounts.curve_account.initial_virtual_sol = actual_initial_virtual_sol;
    ctx.accounts.curve_account.initial_virtual_token = actual_initial_virtual_token;

    // ============ Phase 2 addition: Set pool type, borrow pool ratio, and borrow duration ============
    ctx.accounts.curve_account.pool_type = actual_pool_type;
    ctx.accounts.curve_account.borrow_pool_ratio = actual_borrow_pool_ratio;
    ctx.accounts.curve_account.borrow_duration = actual_borrow_duration;

    // Calculate and set precise initial price
    let initial_price =
        CurveAMM::get_initial_price(actual_initial_virtual_sol, actual_initial_virtual_token)
            .ok_or(error!(
                crate::error::ErrorCode::InitialPriceCalculationError
            ))?;
    ctx.accounts.curve_account.price = initial_price;

    msg!("Calculated precise initial price: {}", initial_price);

    // Check if fee discount needs to be applied
    crate::apply_fee_discount_if_needed!(ctx)?;

    // Top up pool_sol_account to: live rent-exempt minimum + rounding buffer.
    // The outflow floor is the same live rent-exempt minimum, so the buffer stays consumable.
    let pool_sol_target = pool_sol_required_balance(&ctx.accounts.pool_sol_account)?
        .checked_add(POOL_SOL_INITIAL_ROUNDING_BUFFER)
        .ok_or(crate::error::ErrorCode::ArithmeticOverflow)?;
    let pool_sol_top_up = pool_sol_target.saturating_sub(ctx.accounts.pool_sol_account.lamports());
    if pool_sol_top_up > 0 {
        invoke(
            &anchor_lang::solana_program::system_instruction::transfer(
                ctx.accounts.payer.key,
                ctx.accounts.pool_sol_account.key,
                pool_sol_top_up,
            ),
            &[
                ctx.accounts.payer.to_account_info().clone(),
                ctx.accounts.pool_sol_account.clone(),
                ctx.accounts.system_program.to_account_info().clone(),
            ],
        )?;
    }
    // msg!("Successfully transferred 10000 lamports to pool_sol_account");
    // msg!(
    //     "After token creation, pool_sol_account pool SOL balance: {}",
    //     ctx.accounts.pool_sol_account.lamports()
    // );

    // ======================================================================
    // Create Metaplex Token Metadata (must be completed before destroying mint authority)
    // ======================================================================
    // msg!("Creating Metaplex Token Metadata...");

    // Build CreateMetadataAccountV3 instruction structure
    let metadata_instruction = CreateMetadataAccountV3 {
        metadata: ctx.accounts.metadata.key(),
        mint: ctx.accounts.mint_account.key(),
        mint_authority: ctx.accounts.curve_account.key(),
        payer: ctx.accounts.payer.key(),
        update_authority: (ctx.accounts.curve_account.key(), true),
        system_program: ctx.accounts.system_program.key(),
        rent: Some(ctx.accounts.rent.key()),
    };

    // Build DataV2 structure - contains all metadata information for the token
    let data = DataV2 {
        name: name.clone(),
        symbol: symbol.clone(),
        uri: uri.clone(),
        seller_fee_basis_points: 0,
        creators: Some(vec![mpl_token_metadata::types::Creator {
            address: ctx.accounts.payer.key(),
            verified: false,
            share: 100,
        }]),
        collection: None,
        uses: None,
    };

    // Build CreateMetadataAccountV3InstructionArgs - instruction arguments
    let args = CreateMetadataAccountV3InstructionArgs {
        data,
        is_mutable: false,
        collection_details: None,
    };

    // Generate final Solana instruction
    let create_metadata_ix = metadata_instruction.instruction(args);

    // Execute cross-program invocation (CPI) to create metadata, using curve_account as signing authority
    anchor_lang::solana_program::program::invoke_signed(
        &create_metadata_ix,
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint_account.to_account_info(),
            ctx.accounts.curve_account.to_account_info(), // Use curve_account as mint_authority
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.curve_account.to_account_info(), // Use curve_account as update_authority
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        &[&[
            b"borrowing_curve",
            ctx.accounts.mint_account.key().as_ref(),
            &[curve_bump],
        ]],
    )?;

    // // Output success log
    // msg!("Basic token and borrowing liquidity pool created successfully!");

    // // Permanently destroy mint authority to ensure no further token minting
    // msg!("Destroying mint authority to make token non-mintable...");
    set_authority(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            SetAuthority {
                account_or_mint: ctx.accounts.mint_account.to_account_info(),
                current_authority: ctx.accounts.curve_account.to_account_info(),
            },
            &[&[
                b"borrowing_curve",
                ctx.accounts.mint_account.key().as_ref(),
                &[curve_bump],
            ]],
        ),
        AuthorityType::MintTokens,
        None, // Setting to None means permanently deleting mint authority
    )?;

    // msg!("Mint authority permanently destroyed, token supply is now fixed!");

    // // Permanently destroy freeze authority to ensure tokens cannot be frozen
    // msg!("Destroying freeze authority to make tokens non-freezable...");
    set_authority(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            SetAuthority {
                account_or_mint: ctx.accounts.mint_account.to_account_info(),
                current_authority: ctx.accounts.curve_account.to_account_info(),
            },
            &[&[
                b"borrowing_curve",
                ctx.accounts.mint_account.key().as_ref(),
                &[curve_bump],
            ]],
        ),
        AuthorityType::FreezeAccount,
        None, // Setting to None means permanently deleting freeze authority
    )?;

    // msg!("Freeze authority permanently destroyed, tokens cannot be frozen!");

    // // Print all curve_account parameter information
    // msg!("========== create_basic_token curve_account all parameter information ==========");
    // msg!("lp_token_reserve (token amount in liquidity pool): {}", ctx.accounts.curve_account.lp_token_reserve);
    // msg!("lp_sol_reserve (SOL amount in liquidity pool): {}", ctx.accounts.curve_account.lp_sol_reserve);
    // msg!("price (current price): {}", ctx.accounts.curve_account.price);
    // msg!("borrow_token_reserve (token amount in virtual borrow pool): {}", ctx.accounts.curve_account.borrow_token_reserve);
    // msg!("borrow_sol_reserve (SOL amount in virtual borrow pool): {}", ctx.accounts.curve_account.borrow_sol_reserve);
    // msg!("swap_fee (spot trading fee): {}", ctx.accounts.curve_account.swap_fee);
    // msg!("borrow_fee (margin trading fee): {}", ctx.accounts.curve_account.borrow_fee);
    // msg!("fee_discount_flag (fee discount flag): {}", ctx.accounts.curve_account.fee_discount_flag);
    // msg!("base_fee_recipient (technology provider base fee recipient account): {}", ctx.accounts.curve_account.base_fee_recipient);
    // msg!("fee_recipient (partner fee recipient account): {}", ctx.accounts.curve_account.fee_recipient);
    // msg!("fee_split (fee split ratio): {}", ctx.accounts.curve_account.fee_split);
    // msg!("borrow_duration (loan duration in seconds): {}", ctx.accounts.curve_account.borrow_duration);
    // msg!("mint (associated token mint account): {}", ctx.accounts.curve_account.mint);
    // msg!("up_head (linked list head for short orders): {:?}", ctx.accounts.curve_account.up_head);
    // msg!("down_head (linked list head for long orders): {:?}", ctx.accounts.curve_account.down_head);
    // msg!("bump (PDA bump): {}", ctx.accounts.curve_account.bump);
    // msg!("===============================================");

    // Emit token creation event
    emit!(TokenCreatedEvent {
        payer: ctx.accounts.payer.key(),
        mint_account: ctx.accounts.mint_account.key(),
        curve_account: curve_address,
        pool_token_account: ctx.accounts.pool_token_account.key(),
        pool_borrow_token_account: ctx.accounts.pool_borrow_token_account.key(), // Borrow pool dedicated token account
        pool_sol_account: ctx.accounts.pool_sol_account.key(),
        fee_recipient: ctx.accounts.curve_account.fee_recipient,
        base_fee_recipient: ctx.accounts.params.base_fee_recipient, // Add base fee recipient account
        params_account: ctx.accounts.params.key(), // Add partner params account PDA address
        swap_fee: ctx.accounts.curve_account.swap_fee, // Add spot trading fee
        borrow_fee: ctx.accounts.curve_account.borrow_fee, // Add margin trading fee
        fee_discount_flag: ctx.accounts.curve_account.fee_discount_flag, // Add fee discount flag
        name: name.clone(),
        symbol: symbol.clone(),
        uri: uri.clone(),
        up_orderbook: ctx.accounts.up_orderbook.key(), // Add short orderbook address
        down_orderbook: ctx.accounts.down_orderbook.key(), // Add long orderbook address
        latest_price: ctx.accounts.curve_account.price, // Add latest price

        // Add dynamic liquidity pool parameters
        initial_virtual_sol: ctx.accounts.curve_account.initial_virtual_sol, // Initial virtual SOL reserve
        initial_virtual_token: ctx.accounts.curve_account.initial_virtual_token, // Initial virtual Token reserve
        initial_borrow_token_reserve: actual_borrow_token_amount, // Borrow pool initial funding (initial token amount)

        // Add advanced pool parameters
        pool_type: ctx.accounts.curve_account.pool_type, // Pool type
        borrow_pool_ratio: ctx.accounts.curve_account.borrow_pool_ratio, // Borrow pool token ratio
    });

    // Return success result
    Ok(())
}
