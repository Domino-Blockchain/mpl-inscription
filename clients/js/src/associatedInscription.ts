import { Context, Pda, PublicKey } from '@metaplex-foundation/umi';
import {
  string,
  publicKey as publicKeySerializer,
} from '@metaplex-foundation/umi/serializers';

export function findAssociatedInscriptionPda(
  context: Pick<Context, 'eddsa' | 'programs'>,
  seeds: {
    associated_tag: string;
    inscriptionMetadataAccount: PublicKey | Pda;
  }
): Pda {
  const programId = context.programs.getPublicKey(
    'mplInscription',
    '1NSA9E2dwbXfhmvP3VnnjpT8G5R89qnyw7AkXCjhzoB'
  );
  return context.eddsa.findPda(programId, [
    string({ size: 'variable' }).serialize('Inscription'),
    string({ size: 'variable' }).serialize('Association'),
    string({ size: 'variable' }).serialize(seeds.associated_tag),
    publicKeySerializer().serialize(seeds.inscriptionMetadataAccount),
  ]);
}
