/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  ClusterFilter,
  Context,
  Program,
  PublicKey,
} from '@metaplex-foundation/umi';
import {
  getMplInscriptionErrorFromCode,
  getMplInscriptionErrorFromName,
} from '../errors';

export const MPL_INSCRIPTION_PROGRAM_ID =
  '1NSA9E2dwbXfhmvP3VnnjpT8G5R89qnyw7AkXCjhzoB' as PublicKey<'1NSA9E2dwbXfhmvP3VnnjpT8G5R89qnyw7AkXCjhzoB'>;

export function createMplInscriptionProgram(): Program {
  return {
    name: 'mplInscription',
    publicKey: MPL_INSCRIPTION_PROGRAM_ID,
    getErrorFromCode(code: number, cause?: Error) {
      return getMplInscriptionErrorFromCode(code, this, cause);
    },
    getErrorFromName(name: string, cause?: Error) {
      return getMplInscriptionErrorFromName(name, this, cause);
    },
    isOnCluster() {
      return true;
    },
  };
}

export function getMplInscriptionProgram<T extends Program = Program>(
  context: Pick<Context, 'programs'>,
  clusterFilter?: ClusterFilter
): T {
  return context.programs.get<T>('mplInscription', clusterFilter);
}

export function getMplInscriptionProgramId(
  context: Pick<Context, 'programs'>,
  clusterFilter?: ClusterFilter
): PublicKey {
  return context.programs.getPublicKey(
    'mplInscription',
    MPL_INSCRIPTION_PROGRAM_ID,
    clusterFilter
  );
}
