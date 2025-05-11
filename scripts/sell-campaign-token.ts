import { AnchorProvider, BN, Program, Wallet } from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { FinalProject } from "./idl/final_project";
import { PumpFun } from "./idl/pump-fun";
import dotenv from "dotenv";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import {
  getAssociatedTokenAddressSync
} from "@solana/spl-token";
dotenv.config();

async function sellCampaignToken() {
  const devnet = true;
  const keyPair = Keypair.fromSecretKey(bs58.decode(process.env.PRIV_KEY || ""));
  const connection = new Connection(clusterApiUrl(devnet ? "devnet" : "mainnet-beta"), { commitment: 'confirmed' });
  const wallet = new Wallet(keyPair);
  const provider = new AnchorProvider(connection, wallet);
  const IDL: FinalProject = require("./idl/final_project.json");
  const PumpFunIDL: PumpFun = require("./idl/pump-fun.json");
  const program = new Program(IDL, provider);
  const pumpFunProgram = new Program(PumpFunIDL, provider);

  const creatorAddress = new PublicKey(keyPair.publicKey);
  const campaignIndex = new BN(0); // REPLACE WITH CAMPAIGN INDEX
  const minSolOutput = new BN(0);
  const tx = new Transaction();

  const [campaign,] = PublicKey.findProgramAddressSync(
    [Buffer.from("campaign"), creatorAddress.toBuffer(), Buffer.from(campaignIndex.toArray("le", 8))],
    program.programId
  );
  const campaignData = await program.account.campaign.fetch(campaign);
  const mint = campaignData.mint;

  const [bondingCurve] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("bonding-curve"),
      mint.toBuffer()
    ],
    pumpFunProgram.programId
  );

  let associatedCampaign = getAssociatedTokenAddressSync(mint, campaign, true);
  let associatedBondingCurve = getAssociatedTokenAddressSync(mint, bondingCurve, true);

  const pumpFunGlobal = new PublicKey("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf");
  const pumpFunFeeRecipient = devnet
    ? new PublicKey("68yFSZxzLWJXkxxRGydZ63C6mHx1NLEDWmwN9Lb5yySg")
    : new PublicKey("CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM");
  const pumpFunEventAuthority = new PublicKey("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1");
  const sellTokenAccounts = {
    creator: keyPair.publicKey,
    campaignAccount: campaign,
    associatedCampaign,
    mint,
    pumpFunFeeRecipient,
    pumpFunBondingCurve: bondingCurve,
    pumpFunAssociatedBondingCurve: associatedBondingCurve,
    pumpFunGlobal,
    pumpFunEventAuthority,
    pumpFunProgram: pumpFunProgram.programId,
  }
  tx.add(await program.methods.sellToken(minSolOutput).accounts(sellTokenAccounts).instruction());

  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
  tx.feePayer = keyPair.publicKey;
  const recoverTx = Transaction.from(tx.serialize({ requireAllSignatures: false }));
  recoverTx.sign(keyPair);

  // const simulation = await connection.simulateTransaction(tx);
  // simulation.value.logs.forEach((log, index) => {
  //   console.log(`Log ${index}:`, log);
  // });
  // return;
  const txSignature = await connection.sendRawTransaction(recoverTx.serialize({ requireAllSignatures: true }));
  console.log("🚀 ~ sellCampaignToken ~ txSignature:", txSignature)
  let latestBlockHash = await connection.getLatestBlockhash();
  await connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: txSignature,
  });
}

sellCampaignToken();
