import { Uint53 } from '@cosmjs/math';
import { GasPrice, coin } from '@cosmjs/stargate';

export const DEFAULT_GAS = '300000'; // 300K
export const DEFAULT_GAS_ADJUSTMENT = 1.4;

// https://terra-classic-fcd.publicnode.com/v1/txs/gas_prices
export const DEFAULT_GAS_PRICE = GasPrice.fromString('0.15uusd');

export const calculateFee = (
  estimatedGasUsed: bigint | undefined,
  gasAdjustment: number = DEFAULT_GAS_ADJUSTMENT,
  gasPrice: GasPrice = DEFAULT_GAS_PRICE,
) => {
  const gasUsed = Uint53.fromString(
    estimatedGasUsed?.toString() || DEFAULT_GAS,
  ).toNumber();
  const gasLimit = Math.round(gasUsed * gasAdjustment);

  return {
    amount: [
      coin(
        gasPrice.amount.multiply(new Uint53(gasLimit)).ceil().toString(),
        gasPrice.denom,
      ),
    ],
    gas: gasLimit.toString(),
  };
};

export const calculateBurnFee = (
  estimatedGasUsed: bigint | undefined,
  burnAmount: string,
  gasAdjustment: number = DEFAULT_GAS_ADJUSTMENT,
  gasPrice: GasPrice = DEFAULT_GAS_PRICE,
) => {
  const gasUsed = Uint53.fromString(
    estimatedGasUsed?.toString() || DEFAULT_GAS,
  ).toNumber();
  const gasLimit = Math.round(gasUsed * gasAdjustment);

  const gas = BigInt(
    gasPrice.amount.multiply(new Uint53(gasLimit)).ceil().toString(),
  );
  const stability = BigInt(burnAmount) / 200n;
  const fee = gas + stability;

  return {
    amount: [coin(fee.toString(), gasPrice.denom)],
    gas: Math.round(
      Number(fee) / Number(gasPrice.amount.toString()),
    ).toString(),
  };
};
