import { GasPrice, coin } from '@cosmjs/stargate';
import { Uint53 } from '@cosmjs/math';

export const DEFAULT_GAS = '300000'; // 300K
export const DEFAULT_GAS_ADJUSTMENT = 1.4;

// https://terra-classic-fcd.publicnode.com/v1/txs/gas_prices
export const DEFAULT_GAS_PRICES = [
  GasPrice.fromString('28.325uluna'),
  GasPrice.fromString('0.75uusd'),
];

export const calculateFee = (
  estimatedGasUsed: bigint | undefined,
  gasAdjustment: number = DEFAULT_GAS_ADJUSTMENT,
  gasPrices: GasPrice[] = DEFAULT_GAS_PRICES,
) => {
  const gasUsed = Uint53.fromString(
    estimatedGasUsed?.toString() || DEFAULT_GAS,
  ).toNumber();
  const gasLimit = Math.round(gasUsed * gasAdjustment);

  return {
    amount: gasPrices.map(({ amount: gasPriceAmount, denom }) => {
      const fee = gasPriceAmount
        .multiply(new Uint53(gasLimit))
        .ceil()
        .toString();
      return coin(fee, denom);
    }),
    gas: gasLimit.toString(),
  };
};

export const calculateBurnFee = (
  estimatedGasUsed: bigint | undefined,
  burnAmount: string,
  gasAdjustment: number = DEFAULT_GAS_ADJUSTMENT,
  gasPrices: GasPrice[] = DEFAULT_GAS_PRICES,
) => {
  const gasUsed = Uint53.fromString(
    estimatedGasUsed?.toString() || DEFAULT_GAS,
  ).toNumber();
  const gasLimit = Math.round(gasUsed * gasAdjustment);
  const gasLimitFromBurnAmount = Math.round(
    Uint53.fromString(burnAmount).toNumber() * 0.0233 * gasAdjustment,
  );

  // 0.0233 = 0.0035 / 0.15
  return {
    amount: gasPrices.map(({ amount, denom }) => {
      const fee = amount
        .multiply(new Uint53(Math.max(gasLimit, gasLimitFromBurnAmount)))
        .ceil()
        .toString();
      return coin(fee, denom);
    }),
    gas: gasLimit.toString(),
  };
};
