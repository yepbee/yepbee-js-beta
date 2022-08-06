import { expect } from "chai";
import {
  program,
  mainStateKeypair,
  LAMPORTS_PER_SOL_ZEROS,
  programTokenAccount,
  programMint,
  masterPubkey,
} from "./constants";
import { getTokenBalance } from "./utils";
import instructions from "./instructions";

describe("yepbee-js-beta", () => {
  let initialAmount: string = "1000" + LAMPORTS_PER_SOL_ZEROS;
  let tx: string;

  it("Is initialized!", async () => {
    // --------------------------------------------------
    tx = await instructions.initialize(6, initialAmount);

    const programTokenBalance = await getTokenBalance(programTokenAccount);

    console.log("program token balance: ", programTokenBalance);
    expect(programTokenBalance).is.equal(Number(initialAmount));

    const mintAddress = (
      await program.account.mainState.fetch(mainStateKeypair.publicKey)
    ).programTokenAccountInfo.mintAddress.toString();

    console.log("main state mint address: ", mintAddress);
    expect(mintAddress).is.equal(programMint.toString());
    // --------------------------------------------------
    tx = await instructions.createUser(masterPubkey);

    const totalUserSupply = (
      await program.account.mainState.fetch(mainStateKeypair.publicKey)
    ).totalUserSupply.toString();
    console.log("main state total user supply: ", totalUserSupply);
    expect(totalUserSupply).is.equal("1");
    // --------------------------------------------------
    tx = await instructions.faucet();

    const userTokenBalance = await getTokenBalance(programTokenAccount);
    console.log("master token balance: ", userTokenBalance);
    expect(userTokenBalance).is.gt(0);
  });
});
