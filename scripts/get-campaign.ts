import { AnchorProvider, BN, Program, Wallet } from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, Keypair, PublicKey } from "@solana/web3.js";
import { FinalProject } from "./idl/final_project";
import dotenv from "dotenv";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
dotenv.config();

async function getCampaign() {
  const devnet = true;
  const keyPair = Keypair.fromSecretKey(bs58.decode(process.env.PRIV_KEY || ""));
  const connection = new Connection(
    devnet ? clusterApiUrl("devnet") : clusterApiUrl("mainnet-beta"), { commitment: 'confirmed' });
  const wallet = new Wallet(keyPair);
  const provider = new AnchorProvider(connection, wallet);
  const IDL: FinalProject = require("./idl/final_project.json");
  const program = new Program(IDL, provider);

  const creatorAddress = new PublicKey(keyPair.publicKey);
  const campaignIndex = new BN(1);
  const [campaign, _] = PublicKey.findProgramAddressSync(
    [Buffer.from("campaign"), creatorAddress.toBuffer(), Buffer.from(campaignIndex.toArray("le", 8))],
    program.programId
  );
  console.log("🚀 ~ campaign:", campaign)

  const campaignInfo = await connection.getAccountInfo(campaign);
  const minimumRentExemption = await connection.getMinimumBalanceForRentExemption(campaignInfo.data.length);
  let campaignData = await program.account.campaign.fetch(campaign);
  console.log({
    totalFundRaised: campaignInfo.lamports - minimumRentExemption,
    ...campaignData,
  })
}

getCampaign();
