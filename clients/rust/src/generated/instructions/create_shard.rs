//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct CreateShard {
    /// The account to store the shard data in.
    pub shard_account: domichain_program::pubkey::Pubkey,
    /// The account that will pay for the transaction and rent.
    pub payer: domichain_program::pubkey::Pubkey,
    /// System program
    pub system_program: domichain_program::pubkey::Pubkey,
}

impl CreateShard {
    pub fn instruction(
        &self,
        args: CreateShardInstructionArgs,
    ) -> domichain_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateShardInstructionArgs,
        remaining_accounts: &[domichain_program::instruction::AccountMeta],
    ) -> domichain_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(domichain_program::instruction::AccountMeta::new(
            self.shard_account,
            false,
        ));
        accounts.push(domichain_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(domichain_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CreateShardInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        domichain_program::instruction::Instruction {
            program_id: crate::MPL_INSCRIPTION_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct CreateShardInstructionData {
    discriminator: u8,
}

impl CreateShardInstructionData {
    fn new() -> Self {
        Self { discriminator: 7 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateShardInstructionArgs {
    pub shard_number: u8,
}

/// Instruction builder.
#[derive(Default)]
pub struct CreateShardBuilder {
    shard_account: Option<domichain_program::pubkey::Pubkey>,
    payer: Option<domichain_program::pubkey::Pubkey>,
    system_program: Option<domichain_program::pubkey::Pubkey>,
    shard_number: Option<u8>,
    __remaining_accounts: Vec<domichain_program::instruction::AccountMeta>,
}

impl CreateShardBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The account to store the shard data in.
    #[inline(always)]
    pub fn shard_account(&mut self, shard_account: domichain_program::pubkey::Pubkey) -> &mut Self {
        self.shard_account = Some(shard_account);
        self
    }
    /// The account that will pay for the transaction and rent.
    #[inline(always)]
    pub fn payer(&mut self, payer: domichain_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// System program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: domichain_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn shard_number(&mut self, shard_number: u8) -> &mut Self {
        self.shard_number = Some(shard_number);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: domichain_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[domichain_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> domichain_program::instruction::Instruction {
        let accounts = CreateShard {
            shard_account: self.shard_account.expect("shard_account is not set"),
            payer: self.payer.expect("payer is not set"),
            system_program: self.system_program.unwrap_or(domichain_program::pubkey!(
                "11111111111111111111111111111111"
            )),
        };
        let args = CreateShardInstructionArgs {
            shard_number: self.shard_number.clone().expect("shard_number is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_shard` CPI accounts.
pub struct CreateShardCpiAccounts<'a, 'b> {
    /// The account to store the shard data in.
    pub shard_account: &'b domichain_program::account_info::AccountInfo<'a>,
    /// The account that will pay for the transaction and rent.
    pub payer: &'b domichain_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b domichain_program::account_info::AccountInfo<'a>,
}

/// `create_shard` CPI instruction.
pub struct CreateShardCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b domichain_program::account_info::AccountInfo<'a>,
    /// The account to store the shard data in.
    pub shard_account: &'b domichain_program::account_info::AccountInfo<'a>,
    /// The account that will pay for the transaction and rent.
    pub payer: &'b domichain_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b domichain_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CreateShardInstructionArgs,
}

impl<'a, 'b> CreateShardCpi<'a, 'b> {
    pub fn new(
        program: &'b domichain_program::account_info::AccountInfo<'a>,
        accounts: CreateShardCpiAccounts<'a, 'b>,
        args: CreateShardInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            shard_account: accounts.shard_account,
            payer: accounts.payer,
            system_program: accounts.system_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> domichain_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b domichain_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> domichain_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> domichain_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b domichain_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> domichain_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(domichain_program::instruction::AccountMeta::new(
            *self.shard_account.key,
            false,
        ));
        accounts.push(domichain_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        accounts.push(domichain_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(domichain_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = CreateShardInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = domichain_program::instruction::Instruction {
            program_id: crate::MPL_INSCRIPTION_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.shard_account.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.system_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            domichain_program::program::invoke(&instruction, &account_infos)
        } else {
            domichain_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// `create_shard` CPI instruction builder.
pub struct CreateShardCpiBuilder<'a, 'b> {
    instruction: Box<CreateShardCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateShardCpiBuilder<'a, 'b> {
    pub fn new(program: &'b domichain_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateShardCpiBuilderInstruction {
            __program: program,
            shard_account: None,
            payer: None,
            system_program: None,
            shard_number: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The account to store the shard data in.
    #[inline(always)]
    pub fn shard_account(
        &mut self,
        shard_account: &'b domichain_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.shard_account = Some(shard_account);
        self
    }
    /// The account that will pay for the transaction and rent.
    #[inline(always)]
    pub fn payer(
        &mut self,
        payer: &'b domichain_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// System program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b domichain_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn shard_number(&mut self, shard_number: u8) -> &mut Self {
        self.instruction.shard_number = Some(shard_number);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b domichain_program::account_info::AccountInfo<'a>,
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
            &'b domichain_program::account_info::AccountInfo<'a>,
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
    pub fn invoke(&self) -> domichain_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> domichain_program::entrypoint::ProgramResult {
        let args = CreateShardInstructionArgs {
            shard_number: self
                .instruction
                .shard_number
                .clone()
                .expect("shard_number is not set"),
        };
        let instruction = CreateShardCpi {
            __program: self.instruction.__program,

            shard_account: self
                .instruction
                .shard_account
                .expect("shard_account is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct CreateShardCpiBuilderInstruction<'a, 'b> {
    __program: &'b domichain_program::account_info::AccountInfo<'a>,
    shard_account: Option<&'b domichain_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b domichain_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b domichain_program::account_info::AccountInfo<'a>>,
    shard_number: Option<u8>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b domichain_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
