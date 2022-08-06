import {
  mainStateKeypair,
  masterKeypair,
  masterPubkey,
  masterPubkeyToUserId,
  masterTokenAccount,
  masterUserIdToUserState,
  program,
  programMint,
  programTokenAccount,
  PublicKey,
  SYSVAR_RENT_PUBKEY,
} from "../constants";
import {
  findUserIdByPubkey,
  findUserStateByUserId,
  findUserTokenAccount,
  ProgramMintInfo,
  uint,
} from "../utils";

export async function createUser(userPubkey: PublicKey) {
  const newUserId = (
    await program.account.mainState.fetch(mainStateKeypair.publicKey)
  ).totalUserSupply.add(uint(1));

  const pubkeyToUserId = findUserIdByPubkey(userPubkey)[0];
  const userIdToUserState = findUserStateByUserId(newUserId)[0];
  const userTokenAccount = findUserTokenAccount(userPubkey)[0];
  console.log(`creating user state... ${userPubkey}`);
  const res = program.methods
    .createUser()
    .accounts({
      rent: SYSVAR_RENT_PUBKEY,
      payer: masterKeypair.publicKey,
      mainState: mainStateKeypair.publicKey,
      programMint,
      programTokenAccount,
      userPubkey,
      pubkeyToUserId,
      userIdToUserState,
      userTokenAccount,
    })
    .rpc();
  console.log("succeed!");
  return res;
}
