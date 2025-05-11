import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, Keypair, PublicKey } from "@solana/web3.js";
import dotenv from "dotenv";
import { PumpFun } from "./idl/pump-fun";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
dotenv.config();

async function getBondingCurve() {
  const devnet = true;
  const keyPair = Keypair.fromSecretKey(bs58.decode(process.env.PRIV_KEY || ""));
  const connection = new Connection(
    devnet ? clusterApiUrl("devnet") : clusterApiUrl("mainnet-beta"), { commitment: 'confirmed' });
  const wallet = new Wallet(keyPair);
  const provider = new AnchorProvider(connection, wallet);
  const IDL: PumpFun = require("./idl/pump-fun.json");
  const program = new Program(IDL, provider);

  const PUMP_FUN_PROGRAM = new PublicKey("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
  const mint = new PublicKey("H8C2Ct7FhBVh7yvzEee75hdUC3838KmQaSjEKUQwMgrK");
  const [bondingCurve] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("bonding-curve"),
      mint.toBuffer()
    ],
    PUMP_FUN_PROGRAM
  );
  console.log("🚀 ~ getCampaign ~ bondingCurve:", bondingCurve)

  const bondingCurveData = await program.account.bondingCurve.fetch(bondingCurve);

  console.log("virtualTokenReserves: ", bondingCurveData.virtualTokenReserves.toString());
  console.log("virtualSolReserves: ", bondingCurveData.virtualSolReserves.toString());
  console.log("realTokenReserves: ", bondingCurveData.realTokenReserves.toString());
  console.log("tokenTotalSupply: ", bondingCurveData.tokenTotalSupply.toString());
}

getBondingCurve();
