// Instruction Contexts
use {
    super::pdas::{Admin, BorrowingBondingCurve, Params, TradeCooldown},
    super::structs::OrderBook,
    crate::error::ErrorCode,
    anchor_lang::prelude::*,
    anchor_spl::associated_token::AssociatedToken,
    anchor_spl::metadata::Metadata,
    anchor_spl::token::{Mint, Token, TokenAccount},
    // Import core components from Metaplex Token Metadata v5.1.1
    mpl_token_metadata::accounts::Metadata as MetaplexMetadata,
};

// Account structure required for Admin account updates, supports initialization and updates
#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
    // Super admin account, must be a transaction signer
    #[account(mut)]
    pub admin: Signer<'info>,

    // Admin account - derived using PDA, globally unique
    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + Admin::INIT_SPACE,
        seeds = [b"admin"],
        bump,
    )]
    pub admin_account: Account<'info, Admin>,

    // Solana system program
    pub system_program: Program<'info, System>,
}

// Account structure required for partner parameter creation
#[derive(Accounts)]
pub struct CreateParams<'info> {
    // Partner account, must be a transaction signer
    #[account(mut)]
    pub partner: Signer<'info>,

    // Admin account - used to retrieve default values
    #[account(
        seeds = [b"admin"],
        bump,
    )]
    pub admin_account: Account<'info, Admin>,

    // Partner parameters account - uses partner address as seed, ensures one address can only create one
    #[account(
        init,
        payer = partner,
        space = 8 + Params::INIT_SPACE,
        seeds = [b"params", partner.key().as_ref()],
        bump,
    )]
    pub params: Account<'info, Params>,

    // Solana system program
    pub system_program: Program<'info, System>,
}

// Account structure required for partner parameter updates
#[derive(Accounts)]
#[instruction(partner_pubkey: Pubkey)]
pub struct UpdateParams<'info> {
    // Super admin account, must be a transaction signer
    #[account(mut)]
    pub admin: Signer<'info>,

    // Admin account - used to verify permissions
    #[account(
        seeds = [b"admin"],
        bump,
        constraint = admin_account.admin == admin.key() @ ErrorCode::Unauthorized
    )]
    pub admin_account: Account<'info, Admin>,

    // Partner parameters account - the account to be updated
    #[account(
        mut,
        seeds = [b"params", partner_pubkey.as_ref()],
        bump,
    )]
    pub params: Account<'info, Params>,

    // Solana system program
    pub system_program: Program<'info, System>,
}

// Define the account structure required for creating a basic token
#[derive(Accounts)]
pub struct CreateToken<'info> {
    // Account that pays transaction fees, must be a signer
    #[account(mut)]
    pub payer: Signer<'info>,

    // Token mint account - stores basic token information
    #[account(
        // Initialize account
        init,
        // Payer account
        payer = payer,
        // Set token decimals
        mint::decimals = 9,
        // Set mint authority owner to curve_account (PDA)
        mint::authority = curve_account.key(),
        // Set freeze authority owner to curve_account (PDA)
        mint::freeze_authority = curve_account.key(),
    )]
    pub mint_account: Box<Account<'info, Mint>>,

    // Borrowing liquidity pool account - one-to-one correspondence with mint_account
    #[account(
        init,
        payer = payer,
        space = 8 + BorrowingBondingCurve::INIT_SPACE,
        seeds = [b"borrowing_curve", mint_account.key().as_ref()],
        bump,
    )]
    pub curve_account: Account<'info, BorrowingBondingCurve>,

    // Liquidity pool token account
    #[account(
        init,
        payer = payer,
        token::mint = mint_account,
        token::authority = curve_account,
        seeds = [b"pool_token", mint_account.key().as_ref()],
        bump
    )]
    pub pool_token_account: Box<Account<'info, TokenAccount>>,

    // Borrow pool dedicated token account - stores borrow pool tokens separately for on-chain querying
    #[account(
        init,
        payer = payer,
        token::mint = mint_account,
        token::authority = curve_account,
        seeds = [b"pool_borrow_token", mint_account.key().as_ref()],
        bump
    )]
    pub pool_borrow_token_account: Box<Account<'info, TokenAccount>>,

    // Liquidity pool SOL account - changed to system program account, used to store native SOL
    #[account(
        init,
        payer = payer,
        space = 0,
        seeds = [b"pool_sol", mint_account.key().as_ref()],
        bump
    )]
    pub pool_sol_account: AccountInfo<'info>,

    // Short order book (Up direction)
    #[account(
        init,
        seeds = [b"up_orderbook", mint_account.key().as_ref()],
        payer = payer,
        space = 8 + OrderBook::HEADER_SIZE, // Only allocate header space, no slots included
        bump,
    )]
    pub up_orderbook: AccountLoader<'info, OrderBook>,

    // Long order book (Down direction)
    #[account(
        init,
        seeds = [b"down_orderbook", mint_account.key().as_ref()],
        payer = payer,
        space = 8 + OrderBook::HEADER_SIZE, // Only allocate header space, no slots included
        bump,
    )]
    pub down_orderbook: AccountLoader<'info, OrderBook>,

    #[account(
        mut,
        address = MetaplexMetadata::find_pda(&mint_account.key()).0
    )]
    // CHECK: PDA address is validated through address constraint, using Metaplex official find_pda method
    // Metaplex metadata account (using concise PDA validation)
    pub metadata: UncheckedAccount<'info>,
    // Metaplex Token Metadata program (using Anchor type-safe validation)
    pub metadata_program: Program<'info, Metadata>,

    // Partner parameters account - caller directly passes the correct PDA address
    pub params: Account<'info, Params>,

    // SPL token program - system program that handles token operations
    pub token_program: Program<'info, Token>,
    // Solana system program - handles basic system operations
    pub system_program: Program<'info, System>,
    // Rent sysvar - used to calculate rent exemption required for accounts
    pub rent: Sysvar<'info, Rent>,
}

// Define the trading context structure (used for margin trading)
#[derive(Accounts)]
pub struct TradeLongShort<'info> {
    // Account that pays transaction fees, must be a signer
    #[account(mut)]
    pub payer: Signer<'info>,

    // Token mint account - stores basic token information
    #[account(mut)]
    pub mint_account: Box<Account<'info, Mint>>,

    // Borrowing liquidity pool account - one-to-one correspondence with mint_account
    #[account(
        mut,
        seeds = [b"borrowing_curve", mint_account.key().as_ref()],
        bump,
    )]
    pub curve_account: Account<'info, BorrowingBondingCurve>,

    // Liquidity pool token account
    #[account(
        mut,
        seeds = [b"pool_token", mint_account.key().as_ref()],
        bump
    )]
    pub pool_token_account: Box<Account<'info, TokenAccount>>,

    // Borrow pool dedicated token account
    #[account(
        mut,
        seeds = [b"pool_borrow_token", mint_account.key().as_ref()],
        bump
    )]
    pub pool_borrow_token_account: Box<Account<'info, TokenAccount>>,

    // Liquidity pool SOL account - changed to system program account
    #[account(
        mut,
        seeds = [b"pool_sol", mint_account.key().as_ref()],
        bump
    )]
    pub pool_sol_account: AccountInfo<'info>,

    // SPL token program - system program that handles token operations
    pub token_program: Program<'info, Token>,
    // Solana system program - handles basic system operations
    pub system_program: Program<'info, System>,
    // Rent sysvar - used to calculate rent exemption required for accounts
    pub rent: Sysvar<'info, Rent>,

    // Partner fee recipient account
    // CHECK: Partner fee recipient account, address validated from curve_account.fee_recipient at runtime
    #[account(mut)]
    pub fee_recipient_account: UncheckedAccount<'info>,

    // Technical provider base fee recipient account
    // CHECK: Base fee recipient account, address validated from curve_account.base_fee_recipient at runtime
    #[account(mut)]
    pub base_fee_recipient_account: UncheckedAccount<'info>,

    // Short order book (Up direction) - references PDA created during token creation
    #[account(
        mut,
        seeds = [b"up_orderbook", mint_account.key().as_ref()],
        bump,
    )]
    pub up_orderbook: AccountLoader<'info, OrderBook>,

    // Long order book (Down direction) - references PDA created during token creation
    #[account(
        mut,
        seeds = [b"down_orderbook", mint_account.key().as_ref()],
        bump,
    )]
    pub down_orderbook: AccountLoader<'info, OrderBook>,
}

// Define the trading context structure (used for buy/sell trading)
#[derive(Accounts)]
pub struct TradeBuySell<'info> {
    // Account that pays transaction fees, must be a signer
    #[account(mut)]
    pub payer: Signer<'info>,

    // Token mint account - stores basic token information
    #[account(mut)]
    pub mint_account: Box<Account<'info, Mint>>,

    // Borrowing liquidity pool account - one-to-one correspondence with mint_account
    #[account(
        mut,
        seeds = [b"borrowing_curve", mint_account.key().as_ref()],
        bump,
    )]
    pub curve_account: Account<'info, BorrowingBondingCurve>,

    // Liquidity pool token account
    #[account(
        mut,
        seeds = [b"pool_token", mint_account.key().as_ref()],
        bump
    )]
    pub pool_token_account: Box<Account<'info, TokenAccount>>,

    // Borrow pool dedicated token account
    #[account(
        mut,
        seeds = [b"pool_borrow_token", mint_account.key().as_ref()],
        bump
    )]
    pub pool_borrow_token_account: Box<Account<'info, TokenAccount>>,

    // Liquidity pool SOL account - changed to system program account
    #[account(
        mut,
        seeds = [b"pool_sol", mint_account.key().as_ref()],
        bump
    )]
    pub pool_sol_account: AccountInfo<'info>,

    // Short order book (Up direction) - references PDA created during token creation
    #[account(
        mut,
        seeds = [b"up_orderbook", mint_account.key().as_ref()],
        bump,
    )]
    pub up_orderbook: AccountLoader<'info, OrderBook>,

    // Long order book (Down direction) - references PDA created during token creation
    #[account(
        mut,
        seeds = [b"down_orderbook", mint_account.key().as_ref()],
        bump,
    )]
    pub down_orderbook: AccountLoader<'info, OrderBook>,

    // User token account - used to receive purchased tokens
    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    // SPL token program - system program that handles token operations
    pub token_program: Program<'info, Token>,
    // Solana system program - handles basic system operations
    pub system_program: Program<'info, System>,
    // Rent sysvar - used to calculate rent exemption required for accounts
    pub rent: Sysvar<'info, Rent>,
    // Associated Token Program - handles associated token accounts
    pub associated_token_program: Program<'info, AssociatedToken>,

    // Partner fee recipient account
    // CHECK: Partner fee recipient account, address validated from curve_account.fee_recipient at runtime
    #[account(mut)]
    pub fee_recipient_account: UncheckedAccount<'info>,

    // Technical provider base fee recipient account
    // CHECK: Base fee recipient account, address validated from curve_account.base_fee_recipient at runtime
    #[account(mut)]
    pub base_fee_recipient_account: UncheckedAccount<'info>,

    // Trade cooldown PDA
    // Automatically created on first trade, subsequent trades verify cooldown time
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + TradeCooldown::INIT_SPACE,
        seeds = [
            b"trade_cooldown",
            mint_account.key().as_ref(),
            payer.key().as_ref()
        ],
        bump,
    )]
    pub cooldown: Account<'info, TradeCooldown>,
}

// Define the trading context structure (used for active position closing)
#[derive(Accounts)]
pub struct TradeClose<'info> {
    // Account that pays transaction fees, must be a signer
    #[account(mut)]
    pub payer: Signer<'info>,

    // Token mint account - stores basic token information
    #[account(mut)]
    pub mint_account: Box<Account<'info, Mint>>,

    // Borrowing liquidity pool account - one-to-one correspondence with mint_account
    #[account(
        mut,
        seeds = [b"borrowing_curve", mint_account.key().as_ref()],
        bump,
    )]
    pub curve_account: Account<'info, BorrowingBondingCurve>,

    // Liquidity pool token account
    #[account(
        mut,
        seeds = [b"pool_token", mint_account.key().as_ref()],
        bump
    )]
    pub pool_token_account: Box<Account<'info, TokenAccount>>,

    // Borrow pool dedicated token account
    #[account(
        mut,
        seeds = [b"pool_borrow_token", mint_account.key().as_ref()],
        bump
    )]
    pub pool_borrow_token_account: Box<Account<'info, TokenAccount>>,

    // Liquidity pool SOL account - changed to system program account
    #[account(
        mut,
        seeds = [b"pool_sol", mint_account.key().as_ref()],
        bump
    )]
    pub pool_sol_account: AccountInfo<'info>,

    // SOL account of the user who opened close_order - used to receive SOL (margin return or position closing profit)
    // CHECK: User SOL account, address validated from close_order.user at runtime
    #[account(mut)]
    pub user_sol_account: UncheckedAccount<'info>,

    // SPL token program - system program that handles token operations
    pub token_program: Program<'info, Token>,
    // Solana system program - handles basic system operations
    pub system_program: Program<'info, System>,
    // Rent sysvar - used to calculate rent exemption required for accounts
    pub rent: Sysvar<'info, Rent>,

    // Partner fee recipient account
    // CHECK: Partner fee recipient account, address validated from curve_account.fee_recipient at runtime
    #[account(mut)]
    pub fee_recipient_account: UncheckedAccount<'info>,

    // Technical provider base fee recipient account
    // CHECK: Base fee recipient account, address validated from curve_account.base_fee_recipient at runtime
    #[account(mut)]
    pub base_fee_recipient_account: UncheckedAccount<'info>,

    // Short order book (Up direction) - references PDA created during token creation
    #[account(
        mut,
        seeds = [b"up_orderbook", mint_account.key().as_ref()],
        bump,
    )]
    pub up_orderbook: AccountLoader<'info, OrderBook>,

    // Long order book (Down direction) - references PDA created during token creation
    #[account(
        mut,
        seeds = [b"down_orderbook", mint_account.key().as_ref()],
        bump,
    )]
    pub down_orderbook: AccountLoader<'info, OrderBook>,
}

// Lightweight context for approving trade cooldown
// Forms a symmetrical operation pair with CloseCooldown
#[derive(Accounts)]
pub struct ApproveCooldown<'info> {
    // User initiating the approval
    #[account(mut)]
    pub payer: Signer<'info>,

    // Token mint account (used for PDA seed)
    pub mint_account: Box<Account<'info, Mint>>,

    // User's token account (read balance)
    #[account(
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    // Trade cooldown PDA
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + TradeCooldown::INIT_SPACE,
        seeds = [
            b"trade_cooldown",
            mint_account.key().as_ref(),
            payer.key().as_ref()
        ],
        bump,
    )]
    pub cooldown: Account<'info, TradeCooldown>,

    // SPL Token program
    pub token_program: Program<'info, Token>,

    // Associated Token program
    pub associated_token_program: Program<'info, AssociatedToken>,

    // System program
    pub system_program: Program<'info, System>,

    // Rent sysvar
    pub rent: Sysvar<'info, Rent>,
}

// Context structure for manually closing TradeCooldown PDA
#[derive(Accounts)]
pub struct CloseCooldown<'info> {
    // User initiating the close request (must be the PDA owner)
    #[account(mut)]
    pub payer: Signer<'info>,

    // Token mint address
    pub mint_account: Box<Account<'info, Mint>>,

    // User's token account (used to verify balance)
    #[account(
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    // TradeCooldown PDA to be closed
    #[account(
        mut,
        close = payer,  // Rent returned to payer
        seeds = [
            b"trade_cooldown",
            mint_account.key().as_ref(),
            payer.key().as_ref()
        ],
        bump = cooldown.bump,
    )]
    pub cooldown: Account<'info, TradeCooldown>,

    // SPL token program
    pub token_program: Program<'info, Token>,
    // AssociatedToken program
    pub associated_token_program: Program<'info, AssociatedToken>,
    // System program
    pub system_program: Program<'info, System>,
}
