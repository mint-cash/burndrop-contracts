import { Decimal, Uint53 } from '@cosmjs/math';
import { GasPrice, coin } from '@cosmjs/stargate';
import axios from 'axios';

export const DEFAULT_GAS = '300000'; // 300K
export const DEFAULT_GAS_ADJUSTMENT = 1.4;

export const DEFAULT_GAS_PRICE = GasPrice.fromString('0.15uusd');

export const getGasPrice = async (_url?: string) => {
  try {
    const url =
      _url || 'https://terra-classic-fcd.publicnode.com/v1/txs/gas_prices';
    const { data } = await axios.get<Record<string, string>>(url);
    return new GasPrice(Decimal.fromUserInput(data['uusd'], 18), 'uusd');
  } catch (err) {
    console.error(err);
    return DEFAULT_GAS_PRICE;
  }
};

export const getMinStabilitySpread = async (_url?: string) => {
  try {
    const url =
      _url ||
      'https://terra-classic-fcd.publicnode.com/terra/market/v1beta1/params';
    const { data } = await axios.get<{
      params: {
        base_pool: string;
        pool_recovery_period: string;
        min_stability_spread: string;
      };
    }>(url);
    return data.params.min_stability_spread;
  } catch (err) {
    console.error(err);
    return '0.05';
  }
};

const multiplyBigIntAndFloat = (a: bigint, b: number) => {
  // MAX_SAFE_INTEGER is about 9e15, so we can't use 1e18 here
  const floatScale = 10n ** 6n;
  const bAsBigInt = BigInt(Math.ceil(b * Number(floatScale)));
  return (a * bAsBigInt) / floatScale;
};

export const calculateFee = async (
  estimatedGasUsed: bigint | undefined,
  gasAdjustment: number = DEFAULT_GAS_ADJUSTMENT,
) => {
  const gasUsed = Uint53.fromString(
    estimatedGasUsed?.toString() || DEFAULT_GAS,
  ).toNumber();
  const gasLimit = Math.round(gasUsed * gasAdjustment);
  const gasPrice = await getGasPrice();

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

export const calculateBurnFee = async (
  estimatedGasUsed: bigint | undefined,
  burnAmount: string,
  gasAdjustment: number = DEFAULT_GAS_ADJUSTMENT,
) => {
  const gasUsed = Uint53.fromString(
    estimatedGasUsed?.toString() || DEFAULT_GAS,
  ).toNumber();
  const gasLimit = Math.ceil(gasUsed * gasAdjustment);
  const gasPrice = await getGasPrice();

  const gasFee = BigInt(
    gasPrice.amount.multiply(new Uint53(gasLimit)).ceil().toString(),
  );
  const stabilityFee = multiplyBigIntAndFloat(
    BigInt(burnAmount),
    Number(await getMinStabilitySpread()),
  );
  const fee = gasFee + stabilityFee + 1n;

  return {
    amount: [coin(fee.toString(), gasPrice.denom)],
    gas: Math.round(
      Number(gasFee) / Number(gasPrice.amount.toString()),
    ).toString(),
  };
};
