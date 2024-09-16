use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount},
};
use trident_client::fuzzing::{anchor_lang, FuzzingError};
pub struct InitializeSnapshot<'info> {
    pub payer: Signer<'info>,
    pub pool: Option<Account<'info, lending_pool::PoolState>>,
    pub collateral_pool_pda: Option<InterfaceAccount<'info, TokenAccount>>,
    pub collateral_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
pub struct RegisterDepositorSnapshot<'info> {
    pub depositor: Signer<'info>,
    pub depositor_ata: Option<InterfaceAccount<'info, TokenAccount>>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
pub struct DepositSnapshot<'info> {
    pub depositor: Signer<'info>,
    pub depositor_ata: Option<InterfaceAccount<'info, TokenAccount>>,
    pub isol_mint: InterfaceAccount<'info, Mint>,
    pub isol_mint_authority: UncheckedAccount<'info>,
    pub pool_pda: Account<'info, lending_pool::PoolState>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
pub struct BorrowSnapshot<'info> {
    pub borrower: Signer<'info>,
    pub borrower_ata: Option<InterfaceAccount<'info, TokenAccount>>,
    pub collateral_mint: InterfaceAccount<'info, Mint>,
    pub pool_pda: Account<'info, lending_pool::PoolState>,
    pub collateral_pool_pda: InterfaceAccount<'info, TokenAccount>,
    pub isol_mint: InterfaceAccount<'info, Mint>,
    pub isol_mint_authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub loan_record: Option<Account<'info, lending_pool::LoanRecord>>,
}
pub struct AmountToUiAmountSnapshot<'info> {
    pub isol_mint: InterfaceAccount<'info, Mint>,
}
impl<'info> InitializeSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let payer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("payer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("payer".to_string()))?;
        let pool: Option<anchor_lang::accounts::account::Account<lending_pool::PoolState>> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("pool".to_string()))?
                .as_ref()
                .map(|acc| {
                    if acc.key() != *_program_id {
                        anchor_lang::accounts::account::Account::try_from(acc)
                            .map_err(|_| FuzzingError::CannotDeserializeAccount("pool".to_string()))
                    } else {
                        Err(FuzzingError::OptionalAccountNotProvided("pool".to_string()))
                    }
                })
                .transpose()
                .unwrap_or(None);
        let collateral_pool_pda: Option<
            anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "collateral_pool_pda".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::interface_account::InterfaceAccount::try_from(acc)
                        .map_err(|_| {
                            FuzzingError::CannotDeserializeAccount(
                                "collateral_pool_pda".to_string(),
                            )
                        })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "collateral_pool_pda".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let collateral_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "collateral_mint".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("collateral_mint".to_string()))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("collateral_mint".to_string())
                })?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        let token_program: anchor_lang::accounts::program::Program<Token2022> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        let associated_token_program: anchor_lang::accounts::program::Program<AssociatedToken> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "associated_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::program::Program::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "associated_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("associated_token_program".to_string())
                })?;
        let rent: Sysvar<Rent> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("rent".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::sysvar::Sysvar::from_account_info)
            .ok_or(FuzzingError::AccountNotFound("rent".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("rent".to_string()))?;
        Ok(Self {
            payer,
            pool,
            collateral_pool_pda,
            collateral_mint,
            system_program,
            token_program,
            associated_token_program,
            rent,
        })
    }
}
impl<'info> RegisterDepositorSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let depositor: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("depositor".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("depositor".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("depositor".to_string()))?;
        let depositor_ata: Option<
            anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("depositor_ata".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::interface_account::InterfaceAccount::try_from(acc)
                        .map_err(|_| {
                            FuzzingError::CannotDeserializeAccount("depositor_ata".to_string())
                        })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "depositor_ata".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("mint".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("mint".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("mint".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        let token_program: anchor_lang::accounts::program::Program<Token2022> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        let associated_token_program: anchor_lang::accounts::program::Program<AssociatedToken> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "associated_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::program::Program::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "associated_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("associated_token_program".to_string())
                })?;
        Ok(Self {
            depositor,
            depositor_ata,
            mint,
            system_program,
            token_program,
            associated_token_program,
        })
    }
}
impl<'info> DepositSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let depositor: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("depositor".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("depositor".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("depositor".to_string()))?;
        let depositor_ata: Option<
            anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("depositor_ata".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::interface_account::InterfaceAccount::try_from(acc)
                        .map_err(|_| {
                            FuzzingError::CannotDeserializeAccount("depositor_ata".to_string())
                        })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "depositor_ata".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let isol_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("isol_mint".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("isol_mint".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("isol_mint".to_string()))?;
        let isol_mint_authority = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "isol_mint_authority".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::unchecked_account::UncheckedAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "isol_mint_authority".to_string(),
            ))?;
        let pool_pda: anchor_lang::accounts::account::Account<lending_pool::PoolState> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("pool_pda".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::account::Account::try_from)
                .ok_or(FuzzingError::AccountNotFound("pool_pda".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("pool_pda".to_string()))?;
        let token_program: anchor_lang::accounts::program::Program<Token2022> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        let associated_token_program: anchor_lang::accounts::program::Program<AssociatedToken> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "associated_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::program::Program::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "associated_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("associated_token_program".to_string())
                })?;
        Ok(Self {
            depositor,
            depositor_ata,
            isol_mint,
            isol_mint_authority,
            pool_pda,
            token_program,
            system_program,
            associated_token_program,
        })
    }
}
impl<'info> BorrowSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let borrower: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("borrower".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("borrower".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("borrower".to_string()))?;
        let borrower_ata: Option<
            anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("borrower_ata".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::interface_account::InterfaceAccount::try_from(acc)
                        .map_err(|_| {
                            FuzzingError::CannotDeserializeAccount("borrower_ata".to_string())
                        })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "borrower_ata".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let collateral_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "collateral_mint".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("collateral_mint".to_string()))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("collateral_mint".to_string())
                })?;
        let pool_pda: anchor_lang::accounts::account::Account<lending_pool::PoolState> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("pool_pda".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::account::Account::try_from)
                .ok_or(FuzzingError::AccountNotFound("pool_pda".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("pool_pda".to_string()))?;
        let collateral_pool_pda: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "collateral_pool_pda".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "collateral_pool_pda".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("collateral_pool_pda".to_string())
            })?;
        let isol_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("isol_mint".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("isol_mint".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("isol_mint".to_string()))?;
        let isol_mint_authority = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "isol_mint_authority".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::unchecked_account::UncheckedAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "isol_mint_authority".to_string(),
            ))?;
        let token_program: anchor_lang::accounts::program::Program<Token2022> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        let associated_token_program: anchor_lang::accounts::program::Program<AssociatedToken> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "associated_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::program::Program::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "associated_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("associated_token_program".to_string())
                })?;
        let loan_record: Option<anchor_lang::accounts::account::Account<lending_pool::LoanRecord>> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("loan_record".to_string()))?
                .as_ref()
                .map(|acc| {
                    if acc.key() != *_program_id {
                        anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                            FuzzingError::CannotDeserializeAccount("loan_record".to_string())
                        })
                    } else {
                        Err(FuzzingError::OptionalAccountNotProvided(
                            "loan_record".to_string(),
                        ))
                    }
                })
                .transpose()
                .unwrap_or(None);
        Ok(Self {
            borrower,
            borrower_ata,
            collateral_mint,
            pool_pda,
            collateral_pool_pda,
            isol_mint,
            isol_mint_authority,
            token_program,
            system_program,
            associated_token_program,
            loan_record,
        })
    }
}
impl<'info> AmountToUiAmountSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let isol_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("isol_mint".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("isol_mint".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("isol_mint".to_string()))?;
        Ok(Self { isol_mint })
    }
}
pub type UnregisterDepositorSnapshot<'info> = RegisterDepositorSnapshot<'info>;
