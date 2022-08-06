import * as anchor from "@project-serum/anchor";
import { createMint } from "@solana/spl-token";
import {
  connection,
  masterKeypair,
  mintKeypair,
  program,
  PROGRAM_ID,
  PublicKey,
} from "./constants";

export type UINT = anchor.BN;

export type ProgramMintInfo = {
  programMint: PublicKey;
  programTokenAccount: PublicKey;
};

export async function createProgramMint(
  decimals: number,
  _mintKeypair: anchor.web3.Keypair = mintKeypair
): Promise<ProgramMintInfo> {
  console.log("creating program mint...");
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
  console.log("succeed!");
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

export async function getTokenBalance(tokenAccount: PublicKey): Promise<UINT> {
  return uint(
    (await connection.getTokenAccountBalance(tokenAccount)).value.amount
  );
}

export const uint = (
  amount: string | number | anchor.BN | Buffer | Uint8Array | number[]
): UINT => new anchor.BN(amount);

export async function findUserIdAndUserState(userPubkey: PublicKey) {
  const userIdPubkey = findUserIdByPubkey(userPubkey)[0];
  const userId = (await program.account.userId.fetch(userIdPubkey)).id;

  const userStatePubkey = findUserStateByUserId(userId)[0];
  const userState = await program.account.userState.fetch(userStatePubkey);

  console.log(`* findUserIdAndUserState:\nuserId(${userId})`);
  console.dir(userState, { depth: null });

  return {
    userIdPubkey,
    userStatePubkey,
    userId,
    userState,
  };
}
