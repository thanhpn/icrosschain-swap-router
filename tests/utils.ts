import * as anchor from "@project-serum/anchor";
import fs from "fs";
import { web3, Provider } from "@project-serum/anchor";
import {
  mintTo,
  createAssociatedTokenAccount,
} from "@solana/spl-token";

export const handleAirdrop = async (
  provider: Provider,
  account: web3.PublicKey,
  amount: number = web3.LAMPORTS_PER_SOL
) => {
  const airdropSignature = await provider.connection.requestAirdrop(
    account,
    amount
  );
  await provider.connection.confirmTransaction(airdropSignature);
};

export const getKeypairFromFile = (file: string) => {
  const rawdata = fs.readFileSync(`tests/keys/${file}`);
  const keyData = JSON.parse(rawdata.toString());
  return anchor.web3.Keypair.fromSecretKey(new Uint8Array(keyData));
};

export const getTokenBalance = async (
  pubkey: web3.PublicKey,
  provider: Provider
) => {
  return parseInt(
    (await provider.connection.getTokenAccountBalance(pubkey)).value.amount
  );
};

export const createUserAndTokenAccount = async (
  provider: Provider,
  mint: web3.PublicKey,
  mintAuth: web3.Keypair,
  mintX: web3.PublicKey,
  mintUsdc: web3.PublicKey
) => {
  const user = web3.Keypair.generate();
  let userTokenPubkey;
  let userTokenXPubkey;
  let userTokenUsdcPubkey;

  await handleAirdrop(provider, user.publicKey);

  try {
    userTokenPubkey = await createAssociatedTokenAccount(
      provider.connection,
      user,
      mint,
      user.publicKey
    );

    userTokenXPubkey = await createAssociatedTokenAccount(
      provider.connection,
      user,
      mintX,
      user.publicKey
    );

    userTokenUsdcPubkey = await createAssociatedTokenAccount(
      provider.connection,
      user,
      mintUsdc,
      user.publicKey
    );

    await mintTo(
      provider.connection,
      user,
      mint,
      userTokenPubkey,
      mintAuth,
      1000000
    );

    await mintTo(
      provider.connection,
      user,
      mintUsdc,
      userTokenUsdcPubkey,
      mintAuth,
      1000000
    );
  } catch (error) {
    console.error(error);
  }

  return { user, userTokenPubkey, userTokenXPubkey, userTokenUsdcPubkey };
};
