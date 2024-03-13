/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Option,
  OptionOrNullable,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  mapSerializer,
  option,
  string,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type CloseInstructionAccounts = {
  /** The account where data is stored. */
  inscriptionAccount: PublicKey | Pda;
  /** The account to store the inscription account's metadata in. */
  inscriptionMetadataAccount: PublicKey | Pda;
  /** The account that will pay for the rent. */
  payer?: Signer;
  /** The authority of the inscription account. */
  authority?: Signer;
  /** System program */
  systemProgram?: PublicKey | Pda;
};

// Data.
export type CloseInstructionData = {
  discriminator: number;
  associatedTag: Option<string>;
};

export type CloseInstructionDataArgs = {
  associatedTag: OptionOrNullable<string>;
};

export function getCloseInstructionDataSerializer(): Serializer<
  CloseInstructionDataArgs,
  CloseInstructionData
> {
  return mapSerializer<CloseInstructionDataArgs, any, CloseInstructionData>(
    struct<CloseInstructionData>(
      [
        ['discriminator', u8()],
        ['associatedTag', option(string())],
      ],
      { description: 'CloseInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 2 })
  ) as Serializer<CloseInstructionDataArgs, CloseInstructionData>;
}

// Args.
export type CloseInstructionArgs = CloseInstructionDataArgs;

// Instruction.
export function close(
  context: Pick<Context, 'payer' | 'programs'>,
  input: CloseInstructionAccounts & CloseInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'mplInscription',
    '1NSA9E2dwbXfhmvP3VnnjpT8G5R89qnyw7AkXCjhzoB'
  );

  // Accounts.
  const resolvedAccounts: ResolvedAccountsWithIndices = {
    inscriptionAccount: {
      index: 0,
      isWritable: true,
      value: input.inscriptionAccount ?? null,
    },
    inscriptionMetadataAccount: {
      index: 1,
      isWritable: true,
      value: input.inscriptionMetadataAccount ?? null,
    },
    payer: { index: 2, isWritable: true, value: input.payer ?? null },
    authority: { index: 3, isWritable: false, value: input.authority ?? null },
    systemProgram: {
      index: 4,
      isWritable: false,
      value: input.systemProgram ?? null,
    },
  };

  // Arguments.
  const resolvedArgs: CloseInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
  }
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      'splSystem',
      '11111111111111111111111111111111'
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getCloseInstructionDataSerializer().serialize(
    resolvedArgs as CloseInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
