import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { IcrosschainSwapSolana } from "../target/types/icrosschain_swap_solana";
import { web3 } from "@project-serum/anchor";
import fs from "fs";
import path from "path";
import dotenv from "dotenv";
import {
  Keypair,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  SystemProgram,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  MINT_SIZE,
  getOrCreateAssociatedTokenAccount,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddress,
  createInitializeMintInstruction,
  mintTo,
  createMint,
  createAssociatedTokenAccount,
} from "@solana/spl-token";

dotenv.config();

require('dotenv').config();

export const getTester = (file: String) => {
  const rawdata = fs.readFileSync(
    // replace with your key
    path.resolve("tests/" + file + ".json")
  );
  const keyData = JSON.parse(rawdata.toString());
  return web3.Keypair.fromSecretKey(new Uint8Array(keyData));
};

function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}
export const SOL_USDC_ADDRESS = process.env.SOL_USDC_ADDRESS as string;
export const SOL_RAY_ADDRESS = process.env.SOL_RAY_ADDRESS as string;
export const SOL_PDA_ADDRESS = process.env.SOL_PDA_ADDRESS as string;
export const SOL_PDA_USDC_ATA_ADDRESS = process.env
  .SOL_PDA_USDC_ATA_ADDRESS as string;
export const SOL_PDA_RAY_ATA_ADDRESS = process.env
  .SOL_PDA_RAY_ATA_ADDRESS as string;
export const SOL_TREASURY_USDC_ATA_ADDRESS = process.env
  .SOL_TREASURY_USDC_ATA_ADDRESS as string;
export const SOL_TREASURY_RAY_ATA_ADDRESS = process.env
  .SOL_TREASURY_RAY_ATA_ADDRESS as string;
export const SOL_DEPLOYER_USDC_ATA_ADDRESS = process.env
  .SOL_DEPLOYER_USDC_ATA_ADDRESS as string;
export const SOL_DEPLOYER_RAY_ATA_ADDRESS = process.env
  .SOL_DEPLOYER_RAY_ATA_ADDRESS as string;
export const SOL_USER_RAY_ATA_ADDRESS = process.env
  .SOL_USER_RAY_ATA_ADDRESS as string;
export const SOL_USER_USDC_ATA_ADDRESS = process.env
  .SOL_USER_USDC_ATA_ADDRESS as string;

export const POOL_PROGRAM_ID = process.env.POOL_PROGRAM_ID as string;
export const AMM_ID = process.env.AMM_ID as string;
export const AMM_AUTHORITY = process.env.AMM_AUTHORITY as string;
export const AMM_OPEN_ORDERS = process.env.AMM_OPEN_ORDERS as string;
export const AMM_TARGET_ORDERS = process.env.AMM_TARGET_ORDERS as string;
export const POOL_COIN_TOKEN_ACCOUNT = process.env
  .POOL_COIN_TOKEN_ACCOUNT as string;
export const POOL_PC_TOKEN_ACCOUNT = process.env
  .POOL_PC_TOKEN_ACCOUNT as string;
export const SERUM_PROGRAM_ID = process.env.SERUM_PROGRAM_ID as string;
export const SERUM_MARKET = process.env.SERUM_MARKET as string;
export const SERUM_BIDS = process.env.SERUM_BIDS as string;
export const SERUM_ASKS = process.env.SERUM_ASKS as string;
export const SERUM_EVENT_QUEUE = process.env.SERUM_EVENT_QUEUE as string;
export const SERUM_COIN_VAULT_ACCOUNT = process.env
  .SERUM_COIN_VAULT_ACCOUNT as string;
export const SERUM_PC_VAULT_ACCOUNT = process.env
  .SERUM_PC_VAULT_ACCOUNT as string;
export const SERUM_VAULT_SIGNER = process.env.SERUM_VAULT_SIGNER as string;

export const SOURCE_TX_ACCOUNT_ADDRESS = process.env
  .SOURCE_TX_ACCOUNT_ADDRESS as string;

describe("icrosschainSwapSolana", () => {
  // Configure the client to use the local cluster.
  //   anchor.setProvider(anchor.Provider.env());

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const payer = getTester("payer");
  const testUser = getTester("tester1");
  const treasuryWallet = getTester("tester2");
  const pda_address = new PublicKey(SOL_PDA_ADDRESS);
  const pda_ata_usdc = new anchor.web3.PublicKey(SOL_PDA_USDC_ATA_ADDRESS);
  const pda_ata_ray = new anchor.web3.PublicKey(SOL_PDA_RAY_ATA_ADDRESS);
  const ray_mint_address = new anchor.web3.PublicKey(SOL_RAY_ADDRESS);
  const usdc_mint_address = new PublicKey(SOL_USDC_ADDRESS);
  const deployer_ata_ray = new anchor.web3.PublicKey(
    SOL_DEPLOYER_RAY_ATA_ADDRESS
  );
  const deployer_ata_usdc = new anchor.web3.PublicKey(
    SOL_DEPLOYER_USDC_ATA_ADDRESS
  );
  const user_ata_ray = new PublicKey(SOL_USER_RAY_ATA_ADDRESS);
  const user_ata_usdc = new PublicKey(SOL_USER_USDC_ATA_ADDRESS);
  const treasure_ata_ray = new PublicKey(SOL_TREASURY_RAY_ATA_ADDRESS);
  const treasure_ata_usdc = new PublicKey(SOL_TREASURY_USDC_ATA_ADDRESS);
  const source_tx_account = new PublicKey(SOURCE_TX_ACCOUNT_ADDRESS);

  before(async () => { });

  const program = anchor.workspace
    .IcrosschainSwapSolana as Program<IcrosschainSwapSolana>;

  it("Create PDA for program", async () => {
    const programId = program.programId;

    let [pda, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("icrosschain")],
      programId
    );
    console.log(`bump: ${bump}, pubkey: ${pda.toBase58()}`);
  });

  it("Make swap only solana", async () => {
    let pda_amount_token_before = parseInt(
      (await provider.connection.getTokenAccountBalance(user_ata_ray)).value
        .amount
    );
    console.log(
      "user's ray amount token before swap: ",
      pda_amount_token_before
    );
    let receive_amount_token_before = parseInt(
      (await provider.connection.getTokenAccountBalance(user_ata_usdc)).value
        .amount
    );
    console.log(
      "user's usdc amount token before swap: ",
      receive_amount_token_before
    );
    let treasure_ray_before = parseInt(
      (await provider.connection.getTokenAccountBalance(treasure_ata_ray)).value
        .amount
    );
    console.log(
      "treasure's ray amount token before swap: ",
      treasure_ray_before
    );
    let tx = await program.rpc.swapSolana(
      new anchor.BN(100000),
      new anchor.BN(100),
      new anchor.BN(4),
      {
        accounts: {
          poolProgramId: new PublicKey(POOL_PROGRAM_ID), //raydium program
          tokenProgram: TOKEN_PROGRAM_ID, //1
          ammId: new PublicKey(AMM_ID), //2
          ammAuthority: new PublicKey(AMM_AUTHORITY), //3
          ammOpenOrders: new PublicKey(AMM_OPEN_ORDERS), //4
          atoOrMda: new PublicKey(AMM_TARGET_ORDERS), //5
          poolCoinTokenAccount: new PublicKey(POOL_COIN_TOKEN_ACCOUNT), //6 ray
          poolPcTokenAccount: new PublicKey(POOL_PC_TOKEN_ACCOUNT), //7 usdc
          serumProgramId: new PublicKey(SERUM_PROGRAM_ID), //8 Serum DEX V3
          serumMarket: new PublicKey(SERUM_MARKET),
          serumBids: new PublicKey(SERUM_BIDS),
          serumAsks: new PublicKey(SERUM_ASKS),
          serumEventQueue: new PublicKey(SERUM_EVENT_QUEUE),

          serumCoinVaultAccount: new PublicKey(SERUM_COIN_VAULT_ACCOUNT),
          serumPcVaultAccount: new PublicKey(SERUM_PC_VAULT_ACCOUNT),
          serumVaultSigner: new PublicKey(SERUM_VAULT_SIGNER),

          uerSourceTokenAccount: user_ata_ray, //Ray ata source
          uerDestinationTokenAccount: user_ata_usdc, //usdc ata destination
          userSourceOwner: payer.publicKey,
          treasureAta: treasure_ata_ray,
          tokenMint: ray_mint_address,
          pdaAddress: pda_address,
        },
        signers: [payer],
      }
    );
    let pda_amount_token = parseInt(
      (await provider.connection.getTokenAccountBalance(user_ata_ray)).value
        .amount
    );
    console.log("user's ray amount token after swap: ", pda_amount_token);
    let receive_amount_token = parseInt(
      (await provider.connection.getTokenAccountBalance(user_ata_usdc)).value
        .amount
    );
    console.log("user's usdc amount token after swap: ", receive_amount_token);
    let treasure_ray_after = parseInt(
      (await provider.connection.getTokenAccountBalance(treasure_ata_ray)).value
        .amount
    );
    console.log("treasure's ray amount token after swap: ", treasure_ray_after);
    console.log("swap solana tx: ", tx);
  });

  //   it("make swap to solana !", async () => {
  //     let pda_amount_token_before = parseInt(
  //       (await provider.connection.getTokenAccountBalance(pda_ata_usdc)).value
  //         .amount
  //     );
  //     console.log("pda usdc amount token before swap: ", pda_amount_token_before);
  //     let receive_amount_token_before = parseInt(
  //       (await provider.connection.getTokenAccountBalance(user_ata_ray)).value
  //         .amount
  //     );
  //     console.log(
  //       "user's ray amount token before swap: ",
  //       receive_amount_token_before
  //     );
  //     let tx = await program.rpc.swapToSolana(
  //       new anchor.BN(10000),
  //       new anchor.BN(100),
  //       new anchor.BN(4),
  //       "0x08a3a897783784dd5c727d78dd3fc938782fc7f99263c3adc087f1c7e34f1d54",
  //       {
  //         fromAddress: "0x",
  //         toAddress: "0xt",
  //         txHash: "0xxxxxx1",
  //       },
  //       {
  //         accounts: {
  //           poolProgramId: new PublicKey(POOL_PROGRAM_ID), //raydium program
  //           tokenProgram: TOKEN_PROGRAM_ID, //1
  //           ammId: new PublicKey(AMM_ID), //2
  //           ammAuthority: new PublicKey(AMM_AUTHORITY), //3
  //           ammOpenOrders: new PublicKey(AMM_OPEN_ORDERS), //4
  //           atoOrMda: new PublicKey(AMM_TARGET_ORDERS), //5
  //           poolCoinTokenAccount: new PublicKey(POOL_COIN_TOKEN_ACCOUNT), //6 ray
  //           poolPcTokenAccount: new PublicKey(POOL_PC_TOKEN_ACCOUNT), //7 usdc
  //           serumProgramId: new PublicKey(SERUM_PROGRAM_ID), //8 Serum DEX V3
  //           serumMarket: new PublicKey(SERUM_MARKET),
  //           serumBids: new PublicKey(SERUM_BIDS),
  //           serumAsks: new PublicKey(SERUM_ASKS),
  //           serumEventQueue: new PublicKey(SERUM_EVENT_QUEUE),

  //           serumCoinVaultAccount: new PublicKey(SERUM_COIN_VAULT_ACCOUNT),
  //           serumPcVaultAccount: new PublicKey(SERUM_PC_VAULT_ACCOUNT),
  //           serumVaultSigner: new PublicKey(SERUM_VAULT_SIGNER),

  //           uerSourceTokenAccount: pda_ata_usdc, //user ray
  //           uerDestinationTokenAccount: user_ata_ray, //pda usdc
  //           userSourceOwner: pda_address,
  //           author: wallet.publicKey,
  //           sourceTx: source_tx_account,
  //         },
  //         signers: [wallet],
  //       }
  //     );
  //     let pda_amount_token = parseInt(
  //       (await provider.connection.getTokenAccountBalance(pda_ata_usdc)).value
  //         .amount
  //     );
  //     console.log("pda's usdc amount token after swap: ", pda_amount_token);
  //     let receive_amount_token = parseInt(
  //       (await provider.connection.getTokenAccountBalance(user_ata_ray)).value
  //         .amount
  //     );
  //     console.log("user's ray amount token after swap: ", receive_amount_token);
  //     const updatedTweetAccount = await program.account.sourceTx.fetch(
  //       source_tx_account
  //     );
  //     console.log("account", updatedTweetAccount);
  //     console.log("swap to solana tx: ", tx);
  //   });
});
