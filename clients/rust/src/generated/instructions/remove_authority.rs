//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct RemoveAuthority {
    /// The account to store the metadata's metadata in.
    pub inscription_metadata_account: domichain_program::pubkey::Pubkey,
    /// The account paying for the transaction and rent.
    pub payer: domichain_program::pubkey::Pubkey,
    /// The authority of the inscription account to be removed.
    pub authority: Option<domichain_program::pubkey::Pubkey>,
    /// System program
    pub system_program: domichain_program::pubkey::Pubkey,
}

impl RemoveAuthority {
    pub fn instruction(&self) -> domichain_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[domichain_program::instruction::AccountMeta],
    ) -> domichain_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(domichain_program::instruction::AccountMeta::new(
            self.inscription_metadata_account,
            false,
        ));
        accounts.push(domichain_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        if let Some(authority) = self.authority {
            accounts.push(domichain_program::instruction::AccountMeta::new_readonly(
                authority, true,
            ));
        } else {
            accounts.push(domichain_program::instruction::AccountMeta::new_readonly(
                crate::MPL_INSCRIPTION_ID,
                false,
            ));
        }
        accounts.push(domichain_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = RemoveAuthorityInstructionData::new().try_to_vec().unwrap();

        domichain_program::instruction::Instruction {
            program_id: crate::MPL_INSCRIPTION_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct RemoveAuthorityInstructionData {
    discriminator: u8,
}

impl RemoveAuthorityInstructionData {
    fn new() -> Self {
        Self { discriminator: 6 }
    }
}

/// Instruction builder.
#[derive(Default)]
pub struct RemoveAuthorityBuilder {
    inscription_metadata_account: Option<domichain_program::pubkey::Pubkey>,
    payer: Option<domichain_program::pubkey::Pubkey>,
    authority: Option<domichain_program::pubkey::Pubkey>,
    system_program: Option<domichain_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<domichain_program::instruction::AccountMeta>,
}

impl RemoveAuthorityBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The account to store the metadata's metadata in.
    #[inline(always)]
    pub fn inscription_metadata_account(
        &mut self,
        inscription_metadata_account: domichain_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.inscription_metadata_account = Some(inscription_metadata_account);
        self
    }
    /// The account paying for the transaction and rent.
    #[inline(always)]
    pub fn payer(&mut self, payer: domichain_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// `[optional account]`
    /// The authority of the inscription account to be removed.
    #[inline(always)]
    pub fn authority(&mut self, authority: Option<domichain_program::pubkey::Pubkey>) -> &mut Self {
        self.authority = authority;
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
        let accounts = RemoveAuthority {
            inscription_metadata_account: self
                .inscription_metadata_account
                .expect("inscription_metadata_account is not set"),
            payer: self.payer.expect("payer is not set"),
            authority: self.authority,
            system_program: self.system_program.unwrap_or(domichain_program::pubkey!(
                "11111111111111111111111111111111"
            )),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `remove_authority` CPI accounts.
pub struct RemoveAuthorityCpiAccounts<'a, 'b> {
    /// The account to store the metadata's metadata in.
    pub inscription_metadata_account: &'b domichain_program::account_info::AccountInfo<'a>,
    /// The account paying for the transaction and rent.
    pub payer: &'b domichain_program::account_info::AccountInfo<'a>,
    /// The authority of the inscription account to be removed.
    pub authority: Option<&'b domichain_program::account_info::AccountInfo<'a>>,
    /// System program
    pub system_program: &'b domichain_program::account_info::AccountInfo<'a>,
}

/// `remove_authority` CPI instruction.
pub struct RemoveAuthorityCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b domichain_program::account_info::AccountInfo<'a>,
    /// The account to store the metadata's metadata in.
    pub inscription_metadata_account: &'b domichain_program::account_info::AccountInfo<'a>,
    /// The account paying for the transaction and rent.
    pub payer: &'b domichain_program::account_info::AccountInfo<'a>,
    /// The authority of the inscription account to be removed.
    pub authority: Option<&'b domichain_program::account_info::AccountInfo<'a>>,
    /// System program
    pub system_program: &'b domichain_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> RemoveAuthorityCpi<'a, 'b> {
    pub fn new(
        program: &'b domichain_program::account_info::AccountInfo<'a>,
        accounts: RemoveAuthorityCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            inscription_metadata_account: accounts.inscription_metadata_account,
            payer: accounts.payer,
            authority: accounts.authority,
            system_program: accounts.system_program,
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
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(domichain_program::instruction::AccountMeta::new(
            *self.inscription_metadata_account.key,
            false,
        ));
        accounts.push(domichain_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        if let Some(authority) = self.authority {
            accounts.push(domichain_program::instruction::AccountMeta::new_readonly(
                *authority.key,
                true,
            ));
        } else {
            accounts.push(domichain_program::instruction::AccountMeta::new_readonly(
                crate::MPL_INSCRIPTION_ID,
                false,
            ));
        }
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
        let data = RemoveAuthorityInstructionData::new().try_to_vec().unwrap();

        let instruction = domichain_program::instruction::Instruction {
            program_id: crate::MPL_INSCRIPTION_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(4 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.inscription_metadata_account.clone());
        account_infos.push(self.payer.clone());
        if let Some(authority) = self.authority {
            account_infos.push(authority.clone());
        }
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

/// `remove_authority` CPI instruction builder.
pub struct RemoveAuthorityCpiBuilder<'a, 'b> {
    instruction: Box<RemoveAuthorityCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> RemoveAuthorityCpiBuilder<'a, 'b> {
    pub fn new(program: &'b domichain_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(RemoveAuthorityCpiBuilderInstruction {
            __program: program,
            inscription_metadata_account: None,
            payer: None,
            authority: None,
            system_program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The account to store the metadata's metadata in.
    #[inline(always)]
    pub fn inscription_metadata_account(
        &mut self,
        inscription_metadata_account: &'b domichain_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.inscription_metadata_account = Some(inscription_metadata_account);
        self
    }
    /// The account paying for the transaction and rent.
    #[inline(always)]
    pub fn payer(
        &mut self,
        payer: &'b domichain_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// `[optional account]`
    /// The authority of the inscription account to be removed.
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: Option<&'b domichain_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.authority = authority;
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
        let instruction = RemoveAuthorityCpi {
            __program: self.instruction.__program,

            inscription_metadata_account: self
                .instruction
                .inscription_metadata_account
                .expect("inscription_metadata_account is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            authority: self.instruction.authority,

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

struct RemoveAuthorityCpiBuilderInstruction<'a, 'b> {
    __program: &'b domichain_program::account_info::AccountInfo<'a>,
    inscription_metadata_account: Option<&'b domichain_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b domichain_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b domichain_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b domichain_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b domichain_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
