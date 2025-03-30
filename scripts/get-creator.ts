import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, Keypair, PublicKey } from "@solana/web3.js";
import { FinalProject } from "./idl/final_project";
import dotenv from "dotenv";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
dotenv.config();

async function getCreator() {
  const devnet = true;
  const keyPair = Keypair.fromSecretKey(bs58.decode(process.env.PRIV_KEY || ""));
  const connection = new Connection(
    devnet ? clusterApiUrl("devnet") : clusterApiUrl("mainnet-beta"), { commitment: 'confirmed' });
  const wallet = new Wallet(keyPair);
  const provider = new AnchorProvider(connection, wallet);
  const IDL: FinalProject = require("./idl/final_project.json");
  const program = new Program(IDL, provider);

  const creatorAddress = new PublicKey(keyPair.publicKey);
  const [creator, _] = PublicKey.findProgramAddressSync(
    [Buffer.from("creator"), creatorAddress.toBuffer()],
    program.programId
  );
  console.log("🚀 ~ creator:", creator)

  console.log(await program.account.creator.fetch(creator));
}

getCreator();
