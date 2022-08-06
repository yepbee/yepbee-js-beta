import { expect } from "chai";
import {
  program,
  mainStateKeypair,
  LAMPORTS_PER_SOL_ZEROS,
  programTokenAccount,
  programMint,
  masterPubkey,
  Keypair,
  masterTokenAccount,
} from "./constants";
import { findUserTokenAccount, getTokenBalance, uint, UINT } from "./utils";
import instructions from "./instructions";

describe("yepbee-js-beta", () => {
  let initialAmount: UINT = uint("1000" + LAMPORTS_PER_SOL_ZEROS);
  let tx: string;

  const testUserKeypair = Keypair.generate();

  afterEach(() => {
    console.log(`result: ${tx}`);
  });

  it("Is initialized!", async () => {
    try {
      // --------------------------------------------------
      tx = await instructions.initialize(6, initialAmount);

      const programTokenBalance = await getTokenBalance(programTokenAccount);

      console.log(`program token balance: ${programTokenBalance}`);
      expect(programTokenBalance.toString()).is.equal(initialAmount.toString());

      const mintAddress = (
        await program.account.mainState.fetch(mainStateKeypair.publicKey)
      ).programTokenAccountInfo.mintAddress.toString();

      console.log(`main state mint address: ${mintAddress}`);
      expect(mintAddress).is.equal(programMint.toString());
      // --------------------------------------------------
      tx = await instructions.createUser(masterPubkey);

      const totalUserSupply = (
        await program.account.mainState.fetch(mainStateKeypair.publicKey)
      ).totalUserSupply.toString();
      console.log(`main state total user supply: ${totalUserSupply}`);
      expect(totalUserSupply).is.equal("1");
      // --------------------------------------------------
      tx = await instructions.faucet();

      const userTokenBalance = await getTokenBalance(programTokenAccount);
      console.log(`master token balance: ${userTokenBalance}`);
      expect(userTokenBalance.toString()).is.not.equal("0");
    } catch (e) {
      console.error(e);
      throw e;
    }
  });

  it("Creates new user", async () => {
    try {
      const beforeTotalUserSupply = (
        await program.account.mainState.fetch(mainStateKeypair.publicKey)
      ).totalUserSupply;

      tx = await instructions.createUser(testUserKeypair.publicKey);

      const afterTotalUserSupply = (
        await program.account.mainState.fetch(mainStateKeypair.publicKey)
      ).totalUserSupply;

      console.log(`main state total user supply: ${afterTotalUserSupply}`);
      expect(afterTotalUserSupply.toString()).is.equal(
        beforeTotalUserSupply.add(uint(1)).toString()
      );
    } catch (e) {
      console.error(e);
      throw e;
    }
  });

  it("Transfers system token Master to UserA", async () => {
    try {
      const userATokenAccount = findUserTokenAccount(
        testUserKeypair.publicKey
      )[0];

      const beforeMasterTokenAmount = await getTokenBalance(masterTokenAccount);
      const beforeUserATokenAmount = await getTokenBalance(userATokenAccount);

      const amount = uint(30);

      tx = await instructions.transferSystemToken(
        masterPubkey,
        userATokenAccount,
        amount,
        { isToTokenAccount: true }
      );

      const afterMasterTokenAmount = await getTokenBalance(masterTokenAccount);
      const afterUserATokenAmount = await getTokenBalance(userATokenAccount);

      expect(afterMasterTokenAmount.toString()).is.equal(
        beforeMasterTokenAmount.sub(amount).toString()
      );
      expect(afterUserATokenAmount.toString()).is.equal(
        beforeUserATokenAmount.add(amount).toString()
      );
    } catch (e) {
      console.error(e);
      throw e;
    }
  });
});
