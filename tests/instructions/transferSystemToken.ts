import {
  mainStateKeypair,
  masterKeypair,
  program,
  programMint,
  programTokenAccount,
  PublicKey,
  SYSVAR_RENT_PUBKEY,
} from "../constants";
import {
  findUserIdAndUserState,
  findUserTokenAccount,
  UINT,
  uint,
} from "../utils";

export async function transferSystemToken(
  from: PublicKey,
  to: PublicKey,
  amount: UINT,
  options?: { isToTokenAccount: boolean }
) {
  const { isToTokenAccount = false } = options || {};

  console.log(
    `transfering system token... from: ${from}, to: ${to}, amount: ${amount}`
  );

  const user = await findUserIdAndUserState(from);

  const res = await program.methods
    .transferSystemToken(amount)
    .accounts({
      rent: SYSVAR_RENT_PUBKEY,
      payer: masterKeypair.publicKey,
      mainState: mainStateKeypair.publicKey,
      programMint,
      programTokenAccount,
      fromPubkey: from,
      fromPubkeyToUserId: user.userIdPubkey,
      fromUserIdToUserState: user.userStatePubkey,
      fromTokenAccount: user.userState.tokenAccountInfo.tokenAccount,
      toTokenAccount: isToTokenAccount ? to : findUserTokenAccount(to)[0],
    })
    .rpc();
  console.log("succeed!");
  return res;
}
