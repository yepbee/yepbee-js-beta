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
      programMint,
      programTokenAccount,
      payer: masterKeypair.publicKey,
      payerPubkeyToUserId: masterPubkeyToUserId,
      payerUserIdToUserState: masterUserIdToUserState,
      payerTokenAccount: masterTokenAccount,
      mainState: mainStateKeypair.publicKey,
    })
    .rpc();
  console.log("succeed!");
  return res;
}
