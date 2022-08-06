import {
  mainStateKeypair,
  masterKeypair,
  masterPubkeyToUserId,
  masterTokenAccount,
  masterUserIdToUserState,
  program,
  programMint,
  programTokenAccount,
  SYSVAR_RENT_PUBKEY,
  TOKEN_PROGRAM_ID,
} from "../constants";

export async function faucet() {
  console.log("faucet...");
  const res = await program.methods
    .faucet()
    .accounts({
      rent: SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      payer: masterKeypair.publicKey,
      mainState: mainStateKeypair.publicKey,
      programMint,
      programTokenAccount,
      payerTokenAccount: masterTokenAccount,
      pubkeyToUserId: masterPubkeyToUserId,
      userIdToUserState: masterUserIdToUserState,
    })
    .rpc();
  console.log("succeed!");
  return res;
}
