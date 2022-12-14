import * as anchor from "@project-serum/anchor";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
export { LAMPORTS_PER_SOL };
import { AnchorProvider, Program, Wallet } from "@project-serum/anchor";
import { YepbeeJsBeta } from "../target/types/yepbee_js_beta";
import {
  findUserIdByPubkey,
  findUserStateByUserId,
  findUserTokenAccount,
} from "./utils";
import { BN } from "bn.js";

anchor.setProvider(anchor.AnchorProvider.env());

export const Keypair = anchor.web3.Keypair;
export type Keypair = anchor.web3.Keypair;
export const PublicKey = anchor.web3.PublicKey;
export type PublicKey = anchor.web3.PublicKey;

export const program = anchor.workspace.YepbeeJsBeta as Program<YepbeeJsBeta>;
export const connection = program.provider.connection;

export const PROGRAM_ID = program.programId;

export const TOKEN_PROGRAM_ID = anchor.utils.token.TOKEN_PROGRAM_ID;

export const SYSVAR_RENT_PUBKEY = anchor.web3.SYSVAR_RENT_PUBKEY;

export const masterWallet = (program.provider as AnchorProvider).wallet;
export const masterPubkey = masterWallet.publicKey;
export const masterPrivkey = (masterWallet as Wallet).payer.secretKey;
export const masterKeypair = Keypair.fromSecretKey(masterPrivkey);

export const mintKeypair = Keypair.fromSecretKey(
  new Uint8Array(require("./.keys/.mint.json"))
);
export const mainStateKeypair = Keypair.fromSecretKey(
  new Uint8Array(require("./.keys/.mainState.json"))
);

export const masterPubkeyToUserId = findUserIdByPubkey(masterPubkey)[0];
export const masterUserIdToUserState = findUserStateByUserId(new BN(1))[0];

export const programMint = mintKeypair.publicKey;
export const programTokenAccount = PublicKey.findProgramAddressSync(
  [mintKeypair.publicKey.toBuffer()],
  PROGRAM_ID
)[0];
export const masterTokenAccount = findUserTokenAccount(masterPubkey)[0];

export const LAMPORTS_PER_SOL_ZEROS = LAMPORTS_PER_SOL.toString().slice(1);
