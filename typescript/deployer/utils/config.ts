import { OfflineSigner } from '@cosmjs/proto-signing';
import { Secp256k1HdWallet, Secp256k1Wallet } from '@cosmjs/amino';
import { stringToPath } from '@cosmjs/crypto';

const chainId = 'localterra' as const;

type ConfigArgs = {
  mnemonic?: string;
  privateKey?: string;
  endpoint: string;
};

class Config {
  prefix = 'terra';

  constructor(public args: ConfigArgs) {
    if (!args.mnemonic && !args.privateKey) {
      console.error(
        'Error: Either mnemonic or privateKey must be provided via process.env',
      );
      return;
    }
  }

  async getSigner(): Promise<OfflineSigner> {
    let signer: Secp256k1HdWallet | Secp256k1Wallet | undefined = undefined;

    if (this.args.mnemonic) {
      signer = await Secp256k1HdWallet.fromMnemonic(this.args.mnemonic, {
        prefix: this.prefix,
        hdPaths: [stringToPath("m/44'/330'/0'/0/0")],
      });
    } else if (this.args.privateKey) {
      signer = await Secp256k1Wallet.fromKey(
        Buffer.from(this.args.privateKey, 'hex'),
        this.prefix,
      );
    }

    if (!signer) {
      throw Error('no mnemonic or privkey');
    }

    return signer;
  }

  get command(): string {
    return `terrad --node ${this.args.endpoint} --chain-id ${chainId}`;
  }
}

export const config = new Config({
  endpoint: process.env.ENDPOINT || 'http://localhost:26657',
  mnemonic: process.env.MNEMONIC,
  privateKey: process.env.PRIVATE_KEY,
});
