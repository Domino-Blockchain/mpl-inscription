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
  u64,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type AllocateInstructionAccounts = {
  /** The account to store the metadata in. */
  inscriptionAccount: PublicKey | Pda;
  /** The account to store the inscription account's metadata in. */
  inscriptionMetadataAccount: PublicKey | Pda;
  /** The account that will pay for the transaction and rent. */
  payer?: Signer;
  /** The authority of the inscription account. */
  authority?: Signer;
  /** System program */
  systemProgram?: PublicKey | Pda;
};

// Data.
export type AllocateInstructionData = {
  discriminator: number;
  associatedTag: Option<string>;
  targetSize: bigint;
};

export type AllocateInstructionDataArgs = {
  associatedTag: OptionOrNullable<string>;
  targetSize: number | bigint;
};

export function getAllocateInstructionDataSerializer(): Serializer<
  AllocateInstructionDataArgs,
  AllocateInstructionData
> {
  return mapSerializer<
    AllocateInstructionDataArgs,
    any,
    AllocateInstructionData
  >(
    struct<AllocateInstructionData>(
      [
        ['discriminator', u8()],
        ['associatedTag', option(string())],
        ['targetSize', u64()],
      ],
      { description: 'AllocateInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 9 })
  ) as Serializer<AllocateInstructionDataArgs, AllocateInstructionData>;
}

// Args.
export type AllocateInstructionArgs = AllocateInstructionDataArgs;

// Instruction.
export function allocate(
  context: Pick<Context, 'payer' | 'programs'>,
  input: AllocateInstructionAccounts & AllocateInstructionArgs
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
  const resolvedArgs: AllocateInstructionArgs = { ...input };

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
  const data = getAllocateInstructionDataSerializer().serialize(
    resolvedArgs as AllocateInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
