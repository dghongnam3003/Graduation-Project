import { AnchorProvider, BN, Program, Wallet } from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { FinalProject } from "./idl/final_project";
import dotenv from "dotenv";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
dotenv.config();

export type GeckoResponse = {
  "data": {
    "id": "string",
    "type": "string",
    "attributes": {
      "name": "string",
      "address": "string",
      "symbol": "string",
      "decimals": 0,
      "total_supply": "string",
      "coingecko_coin_id": "string",
      "price_usd": "string",
      "fdv_usd": "string",
      "total_reserve_in_usd": "string",
      "volume_usd": {},
      "market_cap_usd": "string"
    },
    "relationships": {}
  }
}

async function updateCampaignClaimableAmount() {
  const devnet = true;
  const keyPair = Keypair.fromSecretKey(bs58.decode(process.env.OPERATOR_PRIV_KEY || ""));
  const connection = new Connection(clusterApiUrl(devnet ? "devnet" : "mainnet-beta"), { commitment: 'confirmed' });
  const wallet = new Wallet(keyPair);
  const provider = new AnchorProvider(connection, wallet);
  const IDL: FinalProject = require("./idl/final_project.json");
  const program = new Program(IDL, provider);

  const tx = new Transaction();
  const creator = new PublicKey("E7348VKp7Rxw32y2YAjpjQdtKgBb6H7gnetkk9fb8gPu"); // REPLACE WITH CREATOR ADDRESS
  const campaignIndex = new BN(2); // REPLACE WITH CAMPAIGN INDEX

  const [campaign,] = PublicKey.findProgramAddressSync(
    [Buffer.from("campaign"), creator.toBuffer(), Buffer.from(campaignIndex.toArray("le", 8))],
    program.programId
  );

  const campaignData = await program.account.campaign.fetch(campaign);

  // Fetch MarketCap data from GeckoTerminal API
  let claimAmount = new BN(0);
  try {
    // const response = await fetch(`https://api.geckoterminal.com/api/v2/networks/solana/tokens/${campaignData.mint}`);
    // const data = await response.json() as GeckoResponse;
    // const marketCap = data.data.attributes.market_cap_usd;
    // const marketCapNumber = parseFloat(marketCap);
    const marketCapNumber = 5_000_000;
    const totalBoughtAmount = campaignData.totalTokenBought;

    if (marketCapNumber >= 5_000_000) {
      // For $5M+ market cap, claim 20%
      claimAmount = totalBoughtAmount.muln(20).divn(100);
    } else if (marketCapNumber >= 2_000_000) {
      // For $2M+ market cap, claim 40%
      claimAmount = totalBoughtAmount.muln(40).divn(100);
    } else if (marketCapNumber >= 1_000_000) {
      // For $1M+ market cap, claim 30%
      claimAmount = totalBoughtAmount.muln(30).divn(100);
    } else if (marketCapNumber >= 500_000) {
      // For $500k+ market cap, claim 10%
      claimAmount = totalBoughtAmount.muln(10).divn(100);
    }

    // Ensure we don't claim more than what's available
    const remainingToClaim = totalBoughtAmount.sub(campaignData.totalClaimed);
    if (claimAmount.gt(remainingToClaim)) {
      claimAmount = remainingToClaim;
    }
  } catch (error) {
    console.error("Error fetching token data:", error);
    return null;
  }

  tx.add(await program.methods.updateClaimableToken(claimAmount).accounts({
    operator: keyPair.publicKey,
    campaignAccount: campaign,
  }).instruction());

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
  console.log("🚀 ~ updateCampaignClaimableAmount ~ txSignature:", txSignature)
  let latestBlockHash = await connection.getLatestBlockhash();
  await connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: txSignature,
  });
}

updateCampaignClaimableAmount();
