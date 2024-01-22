//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::Key;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InscriptionShard {
    pub key: Key,
    pub bump: u8,
    pub shard_number: u8,
    pub count: u64,
}

impl InscriptionShard {
    pub const LEN: usize = 11;

    pub fn create_pda(
        shard_number: u8,
        bump: u8,
    ) -> Result<domichain_program::pubkey::Pubkey, domichain_program::pubkey::PubkeyError> {
        domichain_program::pubkey::Pubkey::create_program_address(
            &[
                "Inscription".as_bytes(),
                "Shard".as_bytes(),
                crate::MPL_INSCRIPTION_ID.as_ref(),
                shard_number.to_string().as_ref(),
                &[bump],
            ],
            &crate::MPL_INSCRIPTION_ID,
        )
    }

    pub fn find_pda(shard_number: u8) -> (domichain_program::pubkey::Pubkey, u8) {
        domichain_program::pubkey::Pubkey::find_program_address(
            &[
                "Inscription".as_bytes(),
                "Shard".as_bytes(),
                crate::MPL_INSCRIPTION_ID.as_ref(),
                shard_number.to_string().as_ref(),
            ],
            &crate::MPL_INSCRIPTION_ID,
        )
    }

    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

impl<'a> TryFrom<&domichain_program::account_info::AccountInfo<'a>> for InscriptionShard {
    type Error = std::io::Error;

    fn try_from(
        account_info: &domichain_program::account_info::AccountInfo<'a>,
    ) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}
