import { BN } from "@coral-xyz/anchor";

// 2% slippage by default
export function calcOutTokenAmount(amountIn: BN, slippagePercentage: number = 200): BN {
  const defaultTokenReserves = new BN(1_073_000_000 * 10 ** 6);
  const defaultSolReserves = new BN(30 * 10 ** 9);
  const base = defaultTokenReserves.mul(amountIn).div(defaultSolReserves.add(amountIn));
  const slippage = base.mul(new BN(slippagePercentage)).div(new BN(10000));
  return base.sub(slippage);
}
