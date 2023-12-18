//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct InitializeFromMint {
    /// The account to store the metadata in.
    pub mint_inscription_account: solana_program::pubkey::Pubkey,
    /// The account to store the inscription account's metadata in.
    pub metadata_account: solana_program::pubkey::Pubkey,
    /// The mint that will be used to derive the PDA.
    pub mint_account: solana_program::pubkey::Pubkey,
    /// The metadata for the mint.
    pub token_metadata_account: solana_program::pubkey::Pubkey,
    /// The token account for the mint.
    pub token_account: solana_program::pubkey::Pubkey,
    /// The shard account for the inscription counter.
    pub inscription_shard_account: solana_program::pubkey::Pubkey,
    /// The account that will pay for the transaction and rent.
    pub payer: solana_program::pubkey::Pubkey,
    /// System program
    pub system_program: solana_program::pubkey::Pubkey,
}

impl InitializeFromMint {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.mint_inscription_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.metadata_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.mint_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_metadata_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.inscription_shard_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = InitializeFromMintInstructionData::new()
            .try_to_vec()
            .unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::MPL_INSCRIPTION_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct InitializeFromMintInstructionData {
    discriminator: u8,
}

impl InitializeFromMintInstructionData {
    fn new() -> Self {
        Self { discriminator: 1 }
    }
}

/// Instruction builder.
#[derive(Default)]
pub struct InitializeFromMintBuilder {
    mint_inscription_account: Option<solana_program::pubkey::Pubkey>,
    metadata_account: Option<solana_program::pubkey::Pubkey>,
    mint_account: Option<solana_program::pubkey::Pubkey>,
    token_metadata_account: Option<solana_program::pubkey::Pubkey>,
    token_account: Option<solana_program::pubkey::Pubkey>,
    inscription_shard_account: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeFromMintBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The account to store the metadata in.
    #[inline(always)]
    pub fn mint_inscription_account(
        &mut self,
        mint_inscription_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.mint_inscription_account = Some(mint_inscription_account);
        self
    }
    /// The account to store the inscription account's metadata in.
    #[inline(always)]
    pub fn metadata_account(
        &mut self,
        metadata_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.metadata_account = Some(metadata_account);
        self
    }
    /// The mint that will be used to derive the PDA.
    #[inline(always)]
    pub fn mint_account(&mut self, mint_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mint_account = Some(mint_account);
        self
    }
    /// The metadata for the mint.
    #[inline(always)]
    pub fn token_metadata_account(
        &mut self,
        token_metadata_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_metadata_account = Some(token_metadata_account);
        self
    }
    /// The token account for the mint.
    #[inline(always)]
    pub fn token_account(&mut self, token_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_account = Some(token_account);
        self
    }
    /// The shard account for the inscription counter.
    #[inline(always)]
    pub fn inscription_shard_account(
        &mut self,
        inscription_shard_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.inscription_shard_account = Some(inscription_shard_account);
        self
    }
    /// The account that will pay for the transaction and rent.
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// System program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = InitializeFromMint {
            mint_inscription_account: self
                .mint_inscription_account
                .expect("mint_inscription_account is not set"),
            metadata_account: self.metadata_account.expect("metadata_account is not set"),
            mint_account: self.mint_account.expect("mint_account is not set"),
            token_metadata_account: self
                .token_metadata_account
                .expect("token_metadata_account is not set"),
            token_account: self.token_account.expect("token_account is not set"),
            inscription_shard_account: self
                .inscription_shard_account
                .expect("inscription_shard_account is not set"),
            payer: self.payer.expect("payer is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `initialize_from_mint` CPI accounts.
pub struct InitializeFromMintCpiAccounts<'a, 'b> {
    /// The account to store the metadata in.
    pub mint_inscription_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account to store the inscription account's metadata in.
    pub metadata_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The mint that will be used to derive the PDA.
    pub mint_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The metadata for the mint.
    pub token_metadata_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The token account for the mint.
    pub token_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The shard account for the inscription counter.
    pub inscription_shard_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account that will pay for the transaction and rent.
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_from_mint` CPI instruction.
pub struct InitializeFromMintCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account to store the metadata in.
    pub mint_inscription_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account to store the inscription account's metadata in.
    pub metadata_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The mint that will be used to derive the PDA.
    pub mint_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The metadata for the mint.
    pub token_metadata_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The token account for the mint.
    pub token_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The shard account for the inscription counter.
    pub inscription_shard_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account that will pay for the transaction and rent.
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> InitializeFromMintCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeFromMintCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            mint_inscription_account: accounts.mint_inscription_account,
            metadata_account: accounts.metadata_account,
            mint_account: accounts.mint_account,
            token_metadata_account: accounts.token_metadata_account,
            token_account: accounts.token_account,
            inscription_shard_account: accounts.inscription_shard_account,
            payer: accounts.payer,
            system_program: accounts.system_program,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.mint_inscription_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.metadata_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.mint_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_metadata_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.inscription_shard_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = InitializeFromMintInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MPL_INSCRIPTION_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(8 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.mint_inscription_account.clone());
        account_infos.push(self.metadata_account.clone());
        account_infos.push(self.mint_account.clone());
        account_infos.push(self.token_metadata_account.clone());
        account_infos.push(self.token_account.clone());
        account_infos.push(self.inscription_shard_account.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.system_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// `initialize_from_mint` CPI instruction builder.
pub struct InitializeFromMintCpiBuilder<'a, 'b> {
    instruction: Box<InitializeFromMintCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeFromMintCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeFromMintCpiBuilderInstruction {
            __program: program,
            mint_inscription_account: None,
            metadata_account: None,
            mint_account: None,
            token_metadata_account: None,
            token_account: None,
            inscription_shard_account: None,
            payer: None,
            system_program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The account to store the metadata in.
    #[inline(always)]
    pub fn mint_inscription_account(
        &mut self,
        mint_inscription_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.mint_inscription_account = Some(mint_inscription_account);
        self
    }
    /// The account to store the inscription account's metadata in.
    #[inline(always)]
    pub fn metadata_account(
        &mut self,
        metadata_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.metadata_account = Some(metadata_account);
        self
    }
    /// The mint that will be used to derive the PDA.
    #[inline(always)]
    pub fn mint_account(
        &mut self,
        mint_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.mint_account = Some(mint_account);
        self
    }
    /// The metadata for the mint.
    #[inline(always)]
    pub fn token_metadata_account(
        &mut self,
        token_metadata_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_metadata_account = Some(token_metadata_account);
        self
    }
    /// The token account for the mint.
    #[inline(always)]
    pub fn token_account(
        &mut self,
        token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_account = Some(token_account);
        self
    }
    /// The shard account for the inscription counter.
    #[inline(always)]
    pub fn inscription_shard_account(
        &mut self,
        inscription_shard_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.inscription_shard_account = Some(inscription_shard_account);
        self
    }
    /// The account that will pay for the transaction and rent.
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// System program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = InitializeFromMintCpi {
            __program: self.instruction.__program,

            mint_inscription_account: self
                .instruction
                .mint_inscription_account
                .expect("mint_inscription_account is not set"),

            metadata_account: self
                .instruction
                .metadata_account
                .expect("metadata_account is not set"),

            mint_account: self
                .instruction
                .mint_account
                .expect("mint_account is not set"),

            token_metadata_account: self
                .instruction
                .token_metadata_account
                .expect("token_metadata_account is not set"),

            token_account: self
                .instruction
                .token_account
                .expect("token_account is not set"),

            inscription_shard_account: self
                .instruction
                .inscription_shard_account
                .expect("inscription_shard_account is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct InitializeFromMintCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    mint_inscription_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    metadata_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mint_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_metadata_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    inscription_shard_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
