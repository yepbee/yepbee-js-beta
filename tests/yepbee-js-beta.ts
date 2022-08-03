import { expect } from "chai";
import {
  masterKeypair,
  masterPubkey,
  program,
  SYSVAR_RENT_PUBKEY,
  TOKEN_PROGRAM_ID,
  PublicKey,
  LAMPORTS_PER_SOL,
  mainStateKeypair,
} from "./constants";
import {
  createProgramMint,
  findUserIdByPubkey,
  findUserStateByUserId,
  findUserTokenAccount,
  getTokenBalance,
  ProgramMintInfo,
  uint,
} from "./utils";

describe("yepbee-js-beta", () => {
  let initialAmount: number;
  let minted: ProgramMintInfo;
  let masterTokenAccount: PublicKey;
  let masterPubkeyToUserId: PublicKey;
  let masterUserIdToUserState: PublicKey;

  let tx: string;

  before(async () => {
    initialAmount = 1000 * LAMPORTS_PER_SOL;
    minted = await createProgramMint(6);
    masterTokenAccount = findUserTokenAccount(masterPubkey)[0];
  });

  afterEach(() => {
    console.log("Your transaction signature", tx);
  });

  it("Is initialized!", async () => {
    console.log(mainStateKeypair.publicKey);
    tx = await program.methods
      .initialize(uint(initialAmount))
      .accounts({
        rent: SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        payer: masterKeypair.publicKey,
        mainState: mainStateKeypair.publicKey,
        programMint: minted.programMint,
        programTokenAccount: minted.programTokenAccount,
      })
      .signers([mainStateKeypair])
      .rpc();

    const programTokenBalance = await getTokenBalance(
      minted.programTokenAccount
    );
    console.log("program token balance: ", programTokenBalance);
    expect(programTokenBalance).is.equal(initialAmount);

    const mintAddress = (
      await program.account.mainState.fetch(mainStateKeypair.publicKey)
    ).programTokenAccountInfo.mintAddress.toString();
    console.log("main state mint address: ", mintAddress);
    expect(mintAddress).is.equal(minted.programMint.toString());
  });

  it("Creates masterTokenAccount", async () => {
    const userId = (
      await program.account.mainState.fetch(mainStateKeypair.publicKey)
    ).totalUserSupply.add(uint(1));

    masterPubkeyToUserId = findUserIdByPubkey(masterPubkey)[0];
    masterUserIdToUserState = findUserStateByUserId(userId)[0];

    tx = await program.methods
      .createUser()
      .accounts({
        rent: SYSVAR_RENT_PUBKEY,
        payer: masterKeypair.publicKey,
        mainState: mainStateKeypair.publicKey,
        userPubkey: masterPubkey,
        pubkeyToUserId: masterPubkeyToUserId,
        userIdToUserState: masterUserIdToUserState,
        programMint: minted.programMint,
        programTokenAccount: minted.programTokenAccount,
        userTokenAccount: masterTokenAccount,
      })
      .rpc();

    const totalUserSupply = (
      await program.account.mainState.fetch(mainStateKeypair.publicKey)
    ).totalUserSupply.toString();
    console.log("main state total user supply: ", totalUserSupply);
    expect(totalUserSupply).is.equal("1");
  });

  it("Gives a faucet", async () => {
    tx = await program.methods
      .faucet()
      .accounts({
        rent: SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        payer: masterKeypair.publicKey,
        mainState: mainStateKeypair.publicKey,
        programMint: minted.programMint,
        programTokenAccount: minted.programTokenAccount,
        payerTokenAccount: masterTokenAccount,
        pubkeyToUserId: masterPubkeyToUserId,
        userIdToUserState: masterUserIdToUserState,
      })
      .rpc();

    const userTokenBalance = await getTokenBalance(minted.programTokenAccount);
    console.log("master token balance: ", userTokenBalance);
    expect(userTokenBalance).is.gt(0);
  });
});
