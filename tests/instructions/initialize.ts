import {
  mainStateKeypair,
  masterKeypair,
  program,
  programMint,
  programTokenAccount,
  SYSVAR_RENT_PUBKEY,
  TOKEN_PROGRAM_ID,
} from "../constants";
import { createProgramMint, uint } from "../utils";

export async function initialize(
  decimals: number,
  amount: string
): Promise<string> {
  await createProgramMint(decimals);
  console.log("initializing...");
  const res = await program.methods
    .initialize(uint(amount))
    .accounts({
      rent: SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      payer: masterKeypair.publicKey,
      mainState: mainStateKeypair.publicKey,
      programMint,
      programTokenAccount,
    })
    .signers([mainStateKeypair])
    .rpc();
  console.log("succeed!");
  return res;
}
