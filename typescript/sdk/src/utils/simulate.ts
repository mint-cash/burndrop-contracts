import { encodeSecp256k1Pubkey } from '@cosmjs/amino';
import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { EncodeObject, type AccountData } from '@cosmjs/proto-signing';

export type TrySimulateEncodedMsgProps = {
  sender: string;
  encodedMsg: EncodeObject;
  signingCosmwasmClient: SigningCosmWasmClient;
};
export const trySimulateEncodedMsg = async ({
  sender,
  encodedMsg,
  signingCosmwasmClient,
}: TrySimulateEncodedMsgProps) => {
  try {
    const anyMsgs = [encodedMsg].map((m) =>
      signingCosmwasmClient.registry.encodeAsAny(m),
    );

    const accountFromSigner = // @ts-ignore
      (await signingCosmwasmClient.signer.getAccounts()).find(
        (account: AccountData) => account.address === sender,
      )!;
    const pubkey = encodeSecp256k1Pubkey(accountFromSigner.pubkey);
    const { sequence } = await signingCosmwasmClient.getSequence(sender);

    // @ts-ignore
    const queryClient = signingCosmwasmClient.forceGetQueryClient();
    const { gasInfo } = await queryClient.tx.simulate(
      anyMsgs,
      '',
      pubkey,
      sequence,
    );
    return gasInfo || null;
  } catch (err) {
    console.error('[!] Simulation Error', err);
    return null;
  }
};
