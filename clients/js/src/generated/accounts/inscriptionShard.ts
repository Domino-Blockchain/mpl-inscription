/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Account,
  Context,
  Pda,
  PublicKey,
  RpcAccount,
  RpcGetAccountOptions,
  RpcGetAccountsOptions,
  assertAccountExists,
  deserializeAccount,
  gpaBuilder,
  publicKey as toPublicKey,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  publicKey as publicKeySerializer,
  string,
  struct,
  u64,
  u8,
} from '@metaplex-foundation/umi/serializers';
import { Key, KeyArgs, getKeySerializer } from '../types';

export type InscriptionShard = Account<InscriptionShardAccountData>;

export type InscriptionShardAccountData = {
  key: Key;
  bump: number;
  shardNumber: number;
  count: bigint;
};

export type InscriptionShardAccountDataArgs = {
  key: KeyArgs;
  bump: number;
  shardNumber: number;
  count: number | bigint;
};

export function getInscriptionShardAccountDataSerializer(): Serializer<
  InscriptionShardAccountDataArgs,
  InscriptionShardAccountData
> {
  return struct<InscriptionShardAccountData>(
    [
      ['key', getKeySerializer()],
      ['bump', u8()],
      ['shardNumber', u8()],
      ['count', u64()],
    ],
    { description: 'InscriptionShardAccountData' }
  ) as Serializer<InscriptionShardAccountDataArgs, InscriptionShardAccountData>;
}

export function deserializeInscriptionShard(
  rawAccount: RpcAccount
): InscriptionShard {
  return deserializeAccount(
    rawAccount,
    getInscriptionShardAccountDataSerializer()
  );
}

export async function fetchInscriptionShard(
  context: Pick<Context, 'rpc'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<InscriptionShard> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  assertAccountExists(maybeAccount, 'InscriptionShard');
  return deserializeInscriptionShard(maybeAccount);
}

export async function safeFetchInscriptionShard(
  context: Pick<Context, 'rpc'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<InscriptionShard | null> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  return maybeAccount.exists ? deserializeInscriptionShard(maybeAccount) : null;
}

export async function fetchAllInscriptionShard(
  context: Pick<Context, 'rpc'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<InscriptionShard[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, 'InscriptionShard');
    return deserializeInscriptionShard(maybeAccount);
  });
}

export async function safeFetchAllInscriptionShard(
  context: Pick<Context, 'rpc'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<InscriptionShard[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) =>
      deserializeInscriptionShard(maybeAccount as RpcAccount)
    );
}

export function getInscriptionShardGpaBuilder(
  context: Pick<Context, 'rpc' | 'programs'>
) {
  const programId = context.programs.getPublicKey(
    'mplInscription',
    '1NSA9E2dwbXfhmvP3VnnjpT8G5R89qnyw7AkXCjhzoB'
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      key: KeyArgs;
      bump: number;
      shardNumber: number;
      count: number | bigint;
    }>({
      key: [0, getKeySerializer()],
      bump: [1, u8()],
      shardNumber: [2, u8()],
      count: [3, u64()],
    })
    .deserializeUsing<InscriptionShard>((account) =>
      deserializeInscriptionShard(account)
    );
}

export function getInscriptionShardSize(): number {
  return 11;
}

export function findInscriptionShardPda(
  context: Pick<Context, 'eddsa' | 'programs'>,
  seeds: {
    shardNumber: number;
  }
): Pda {
  const programId = context.programs.getPublicKey(
    'mplInscription',
    '1NSA9E2dwbXfhmvP3VnnjpT8G5R89qnyw7AkXCjhzoB'
  );
  return context.eddsa.findPda(programId, [
    string({ size: 'variable' }).serialize('Inscription'),
    string({ size: 'variable' }).serialize('Shard'),
    publicKeySerializer().serialize(programId),
    u8().serialize(seeds.shardNumber),
  ]);
}

export async function fetchInscriptionShardFromSeeds(
  context: Pick<Context, 'eddsa' | 'programs' | 'rpc'>,
  seeds: Parameters<typeof findInscriptionShardPda>[1],
  options?: RpcGetAccountOptions
): Promise<InscriptionShard> {
  return fetchInscriptionShard(
    context,
    findInscriptionShardPda(context, seeds),
    options
  );
}

export async function safeFetchInscriptionShardFromSeeds(
  context: Pick<Context, 'eddsa' | 'programs' | 'rpc'>,
  seeds: Parameters<typeof findInscriptionShardPda>[1],
  options?: RpcGetAccountOptions
): Promise<InscriptionShard | null> {
  return safeFetchInscriptionShard(
    context,
    findInscriptionShardPda(context, seeds),
    options
  );
}
