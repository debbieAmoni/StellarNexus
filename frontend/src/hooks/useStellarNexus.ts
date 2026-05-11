import { useState, useCallback } from "react";
import { Contract, SorobanRpc, TransactionBuilder, Networks, BASE_FEE } from "@stellar/stellar-sdk";
import { getPublicKey, signTransaction } from "@stellar/freighter-api";

const RPC_URL = "https://soroban-testnet.stellar.org";
const CONTRACT_ID = import.meta.env.VITE_CONTRACT_ID as string;
const NETWORK_PASSPHRASE = Networks.TESTNET;

export function useStellarNexus() {
  const [publicKey, setPublicKey] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const connect = useCallback(async () => {
    try {
      const key = await getPublicKey();
      setPublicKey(key);
    } catch (e) {
      setError("Freighter not available");
    }
  }, []);

  const invokeContract = useCallback(
    async (method: string, args: unknown[]) => {
      if (!publicKey) throw new Error("Wallet not connected");
      setLoading(true);
      setError(null);
      try {
        const server = new SorobanRpc.Server(RPC_URL);
        const account = await server.getAccount(publicKey);
        const contract = new Contract(CONTRACT_ID);

        const tx = new TransactionBuilder(account, {
          fee: BASE_FEE,
          networkPassphrase: NETWORK_PASSPHRASE,
        })
          .addOperation(contract.call(method, ...(args as Parameters<typeof contract.call>).slice(1)))
          .setTimeout(30)
          .build();

        const prepared = await server.prepareTransaction(tx);
        const signed = await signTransaction(prepared.toXDR(), {
          networkPassphrase: NETWORK_PASSPHRASE,
        });
        const result = await server.sendTransaction(
          TransactionBuilder.fromXDR(signed, NETWORK_PASSPHRASE)
        );
        return result;
      } catch (e) {
        setError((e as Error).message);
        throw e;
      } finally {
        setLoading(false);
      }
    },
    [publicKey]
  );

  const heartbeat = useCallback(
    () => invokeContract("heartbeat", [publicKey]),
    [invokeContract, publicKey]
  );

  const deposit = useCallback(
    (amount: bigint) => invokeContract("deposit", [publicKey, amount]),
    [invokeContract, publicKey]
  );

  const checkAndRelease = useCallback(
    () => invokeContract("check_and_release", []),
    [invokeContract]
  );

  return { publicKey, loading, error, connect, heartbeat, deposit, checkAndRelease };
}
