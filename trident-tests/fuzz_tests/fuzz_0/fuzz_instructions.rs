pub mod lending_pool_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use anchor_spl::{associated_token::get_associated_token_address, token_2022};
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        Initialize(Initialize),
        RegisterDepositor(RegisterDepositor),
        UnregisterDepositor(UnregisterDepositor),
        Deposit(Deposit),
        // Borrow(Borrow),
        // AmountToUiAmount(AmountToUiAmount),
    }
    #[derive(Arbitrary, Debug)]
    pub struct Initialize {
        pub accounts: InitializeAccounts,
        pub data: InitializeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeAccounts {
        pub payer: AccountId,
        pub pool: AccountId,
        pub collateral_pool_pda: AccountId,
        pub collateral_mint: AccountId,
        pub system_program: AccountId,
        pub token_program: AccountId,
        pub associated_token_program: AccountId,
        pub rent: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeData {}
    #[derive(Arbitrary, Debug)]
    pub struct RegisterDepositor {
        pub accounts: RegisterDepositorAccounts,
        pub data: RegisterDepositorData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct RegisterDepositorAccounts {
        pub depositor: AccountId,
        pub depositor_ata: AccountId,
        pub mint: AccountId,
        pub system_program: AccountId,
        pub token_program: AccountId,
        pub associated_token_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct RegisterDepositorData {}
    #[derive(Arbitrary, Debug)]
    pub struct UnregisterDepositor {
        pub accounts: UnregisterDepositorAccounts,
        pub data: UnregisterDepositorData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UnregisterDepositorAccounts {
        pub depositor: AccountId,
        pub depositor_ata: AccountId,
        pub mint: AccountId,
        pub system_program: AccountId,
        pub token_program: AccountId,
        pub associated_token_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UnregisterDepositorData {}
    #[derive(Arbitrary, Debug)]
    pub struct Deposit {
        pub accounts: DepositAccounts,
        pub data: DepositData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct DepositAccounts {
        pub depositor: AccountId,
        pub depositor_ata: AccountId,
        pub isol_mint: AccountId,
        pub isol_mint_authority: AccountId,
        pub pool_pda: AccountId,
        pub token_program: AccountId,
        pub system_program: AccountId,
        pub associated_token_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct DepositData {
        #[arbitrary(
            with = |u: &mut arbitrary::Unstructured| u.int_in_range(1..=10*LAMPORTS_PER_SOL)
        )]
        pub amount: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct Borrow {
        pub accounts: BorrowAccounts,
        pub data: BorrowData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct BorrowAccounts {
        pub borrower: AccountId,
        pub borrower_ata: AccountId,
        pub collateral_mint: AccountId,
        pub pool_pda: AccountId,
        pub collateral_pool_pda: AccountId,
        pub isol_mint: AccountId,
        pub isol_mint_authority: AccountId,
        pub token_program: AccountId,
        pub system_program: AccountId,
        pub associated_token_program: AccountId,
        pub loan_record: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct BorrowData {
        pub amount: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct AmountToUiAmount {
        pub accounts: AmountToUiAmountAccounts,
        pub data: AmountToUiAmountData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct AmountToUiAmountAccounts {
        pub isol_mint: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct AmountToUiAmountData {
        pub amount: u64,
        pub unix_timestamp: u64,
    }
    impl<'info> IxOps<'info> for Initialize {
        type IxData = lending_pool::instruction::Initialize;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = lending_pool::instruction::Initialize {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let lamports = 10 * LAMPORTS_PER_SOL;

            let payer =
                fuzz_accounts
                    .payer
                    .get_or_create_account(self.accounts.payer, client, lamports);
            let pool = fuzz_accounts
                .pool
                .get_or_create_account(self.accounts.pool, &[b"pool"], &lending_pool::ID)
                .unwrap();

            //USDC mint account
            let collateral_mint = fuzz_accounts
                .collateral_mint
                .get_or_create_account(
                    self.accounts.collateral_mint,
                    client,
                    6,
                    &anchor_spl::token_2022::ID,
                    None,
                )
                .unwrap();

            let collateral_pool_pda = fuzz_accounts
                .collateral_pool_pda
                .get_or_create_account(
                    self.accounts.collateral_pool_pda,
                    client,
                    collateral_mint,
                    lending_pool::ID,
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let signers = vec![payer.clone()];

            let acc_meta = lending_pool::accounts::Initialize {
                payer: payer.pubkey(),
                pool: pool.pubkey(),
                collateral_pool_pda: collateral_pool_pda,
                collateral_mint: collateral_mint,
                system_program: solana_sdk::system_program::ID,
                token_program: anchor_spl::token_2022::ID,
                associated_token_program: anchor_spl::associated_token::ID,
                rent: solana_sdk::sysvar::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for RegisterDepositor {
        type IxData = lending_pool::instruction::RegisterDepositor;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = RegisterDepositorSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = lending_pool::instruction::RegisterDepositor {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let depositor = fuzz_accounts.depositor.get_or_create_account(
                self.accounts.depositor,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            //iSOL mint account
            let mint = fuzz_accounts
                .collateral_mint
                .get_or_create_account(
                    self.accounts.mint,
                    client,
                    9,
                    &anchor_spl::token_2022::ID,
                    None,
                )
                .unwrap();

            let depositor_ata = get_associated_token_address(&depositor.pubkey(), &mint);

            let signers = vec![depositor.clone()];
            let acc_meta = lending_pool::accounts::Registration {
                depositor: depositor.pubkey(),
                depositor_ata: depositor_ata,
                mint: mint,
                system_program: solana_sdk::system_program::ID,
                token_program: anchor_spl::token_2022::ID,
                associated_token_program: anchor_spl::associated_token::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for UnregisterDepositor {
        type IxData = lending_pool::instruction::UnregisterDepositor;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = UnregisterDepositorSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = lending_pool::instruction::UnregisterDepositor {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let depositor = fuzz_accounts.depositor.get_or_create_account(
                self.accounts.depositor,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            //iSOL mint account
            let mint = fuzz_accounts
                .collateral_mint
                .get_or_create_account(
                    self.accounts.mint,
                    client,
                    9,
                    &anchor_spl::token_2022::ID,
                    None,
                )
                .unwrap();

            let depositor_ata = get_associated_token_address(&depositor.pubkey(), &mint);
            let signers = vec![depositor.clone()];
            let acc_meta = lending_pool::accounts::Registration {
                depositor: depositor.pubkey(),
                depositor_ata: depositor_ata,
                mint: mint,
                system_program: solana_sdk::system_program::ID,
                token_program: anchor_spl::token_2022::ID,
                associated_token_program: anchor_spl::associated_token::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Deposit {
        type IxData = lending_pool::instruction::Deposit;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = DepositSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = lending_pool::instruction::Deposit {
                amount: self.data.amount,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let depositor = fuzz_accounts.depositor.get_or_create_account(
                self.accounts.depositor,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let pool_pda = fuzz_accounts
                .pool
                .get_or_create_account(self.accounts.pool_pda, &[b"pool"], &lending_pool::ID)
                .unwrap();

            //pda owned by the program as the mint authority
            let isol_mint_authority = fuzz_accounts
                .isol_mint_authority
                .get_or_create_account(
                    self.accounts.isol_mint_authority,
                    &[b"isol_mint_auth"],
                    &lending_pool::ID,
                )
                .unwrap();

            //iSOL mint account
            let isol_mint = fuzz_accounts
                .collateral_mint
                .get_or_create_account(
                    self.accounts.isol_mint,
                    client,
                    9,
                    &isol_mint_authority.pubkey(),
                    None,
                )
                .unwrap();

            let depositor_ata = get_associated_token_address(&depositor.pubkey(), &isol_mint);
            let signers = vec![depositor.clone()];
            let acc_meta = lending_pool::accounts::Deposit {
                depositor: depositor.pubkey(),
                depositor_ata: depositor_ata,
                isol_mint: isol_mint,
                isol_mint_authority: isol_mint_authority.pubkey(),
                pool_pda: pool_pda.pubkey(),
                system_program: solana_sdk::system_program::ID,
                token_program: anchor_spl::token_2022::ID,
                associated_token_program: anchor_spl::associated_token::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Borrow {
        type IxData = lending_pool::instruction::Borrow;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = BorrowSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = lending_pool::instruction::Borrow { amount: todo!() };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = lending_pool::accounts::Borrow {
                borrower: todo!(),
                borrower_ata: todo!(),
                collateral_mint: todo!(),
                pool_pda: todo!(),
                collateral_pool_pda: todo!(),
                isol_mint: todo!(),
                isol_mint_authority: todo!(),
                token_program: todo!(),
                system_program: todo!(),
                associated_token_program: todo!(),
                loan_record: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for AmountToUiAmount {
        type IxData = lending_pool::instruction::AmountToUiAmount;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = AmountToUiAmountSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = lending_pool::instruction::AmountToUiAmount {
                amount: todo!(),
                unix_timestamp: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = lending_pool::accounts::AmountToUiAmount { isol_mint: todo!() }
                .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        // associated_token_program: AccountsStorage<todo!()>,
        borrower: AccountsStorage<Keypair>,
        // borrower_ata: AccountsStorage<TokenStore>,
        collateral_mint: AccountsStorage<MintStore>,
        collateral_pool_pda: AccountsStorage<TokenStore>,
        depositor: AccountsStorage<Keypair>,
        // depositor_ata: AccountsStorage<TokenStore>,
        isol_mint: AccountsStorage<MintStore>,
        isol_mint_authority: AccountsStorage<PdaStore>,
        loan_record: AccountsStorage<PdaStore>,
        mint: AccountsStorage<MintStore>,
        payer: AccountsStorage<Keypair>,
        pool: AccountsStorage<PdaStore>,
        pool_pda: AccountsStorage<PdaStore>,
        // rent: AccountsStorage<ProgramStore>,
        // system_program: AccountsStorage<ProgramStore>,
        // token_program: AccountsStorage<ProgramStore>,
    }
}
