import * as anchor from "@project-serum/anchor";
import { createMint } from "@solana/spl-token";
import {
  connection,
  masterKeypair,
  mintKeypair,
  PROGRAM_ID,
  PublicKey,
} from "./constants";

export type ProgramMintInfo = {
  programMint: PublicKey;
  programTokenAccount: PublicKey;
};

export async function createProgramMint(
  decimals: number,
  _mintKeypair: anchor.web3.Keypair = mintKeypair
): Promise<ProgramMintInfo> {
  const [programPDA, bump] = PublicKey.findProgramAddressSync(
    [_mintKeypair.publicKey.toBuffer()],
    PROGRAM_ID
  );
  await createMint(
    connection,
    masterKeypair,
    programPDA,
    programPDA,
    decimals,
    _mintKeypair
  );
  // console.log("Mint Address: ", confirm.toString());
  return {
    programMint: _mintKeypair.publicKey,
    programTokenAccount: programPDA,
  };
}

export function findUserTokenAccount(
  userPubkey: anchor.web3.PublicKey,
  _mintKeypair: anchor.web3.Keypair = mintKeypair
) {
  return PublicKey.findProgramAddressSync(
    [mintKeypair.publicKey.toBuffer(), userPubkey.toBuffer()],
    PROGRAM_ID
  );
}

export function findPDA(
  seeds: (Buffer | Uint8Array)[],
  _mintKeypair: anchor.web3.Keypair = mintKeypair
) {
  return PublicKey.findProgramAddressSync(seeds, PROGRAM_ID);
}

export function findUserIdByPubkey(pubkey: anchor.web3.PublicKey) {
  return findPDA([Buffer.from("user_state-pubkey"), pubkey.toBuffer()]);
}
export function findUserStateByUserId(id: anchor.BN) {
  return findPDA([Buffer.from("user_state-id"), Buffer.from(id.toString())]);
}

export async function getTokenBalance(tokenAccount: PublicKey) {
  return parseInt(
    (await connection.getTokenAccountBalance(tokenAccount)).value.amount
  );
}

export const uint = (
  amount: string | number | anchor.BN | Buffer | Uint8Array | number[]
) => new anchor.BN(amount);
