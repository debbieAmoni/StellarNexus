import { useState, useCallback, useRef, useEffect } from "react";
import { 
  Contract, 
  SorobanRpc, 
  TransactionBuilder, 
  Networks, 
  BASE_FEE,
  Address,
  nativeToScVal
} from "@stellar/stellar-sdk";
import { getPublicKey, signTransaction, isConnected } from "@stellar/freighter-api";

const RPC_URL = process.env.VITE_RPC_URL || "https://soroban-testnet.stellar.org";
const CONTRACT_ID = process.env.VITE_CONTRACT_ID as string;
const NETWORK = process.env.VITE_NETWORK || "testnet";
const NETWORK_PASSPHRASE = NETWORK === "mainnet" ? Networks.PUBLIC : Networks.TESTNET;

export interface Beneficiary {
  address: string;
  basisPoints: number;
}

export interface VaultState {
  owner: string;
  balance: bigint;
  beneficiaries: Beneficiary[];
  timeRemaining: bigint;
  isPaused: boolean;
}

export function useStellarNexus() {
  const [publicKey, setPublicKey] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [vaultState, setVaultState] = useState<VaultState | null>(null);
  const serverRef = useRef<SorobanRpc.Server | null>(null);

  // Initialize server connection
  useEffect(() => {
    if (!serverRef.current) {
      serverRef.current = new SorobanRpc.Server(RPC_URL);
    }
  }, []);

  const connect = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      
      const connected = await isConnected();
      if (!connected) {
        throw new Error("Freighter wallet not installed or not connected");
      }
      
      const key = await getPublicKey();
      setPublicKey(key);
      return key;
    } catch (e) {
      const message = e instanceof Error ? e.message : "Failed to connect wallet";
      setError(message);
      throw e;
    } finally {
      setLoading(false);
    }
  }, []);

  const disconnect = useCallback(() => {
    setPublicKey(null);
    setVaultState(null);
    setError(null);
  }, []);

  const invokeContract = useCallback(
    async (method: string, args: unknown[]) => {
      if (!publicKey) throw new Error("Wallet not connected");
      if (!CONTRACT_ID) throw new Error("Contract ID not configured");
      if (!serverRef.current) throw new Error("Server not initialized");

      setLoading(true);
      setError(null);
      
      try {
        const server = serverRef.current;
        const account = await server.getAccount(publicKey);
        const contract = new Contract(CONTRACT_ID);

        const tx = new TransactionBuilder(account, {
          fee: BASE_FEE,
          networkPassphrase: NETWORK_PASSPHRASE,
        })
          .addOperation(contract.call(method, ...(args as any[])))
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
        const message = e instanceof Error ? e.message : "Contract invocation failed";
        setError(message);
        throw e;
      } finally {
        setLoading(false);
      }
    },
    [publicKey]
  );

  const initialize = useCallback(
    async (beneficiaries: Beneficiary[]) => {
      if (!publicKey) throw new Error("Wallet not connected");
      
      // Validate basis points sum to 10000
      const total = beneficiaries.reduce((sum, b) => sum + b.basisPoints, 0);
      if (total !== 10000) {
        throw new Error(`Beneficiary allocations must sum to 10000 basis points (got ${total})`);
      }

      try {
        const beneArgs = beneficiaries.map(b => [
          new Address(b.address),
          b.basisPoints
        ]);
        
        return await invokeContract("initialize", [
          new Address(publicKey),
          beneArgs
        ]);
      } catch (e) {
        const message = e instanceof Error ? e.message : "Initialization failed";
        setError(message);
        throw e;
      }
    },
    [publicKey, invokeContract]
  );

  const deposit = useCallback(
    async (amount: bigint) => {
      if (!publicKey) throw new Error("Wallet not connected");
      if (amount <= 0) throw new Error("Amount must be positive");

      return await invokeContract("deposit", [
        new Address(publicKey),
        amount
      ]);
    },
    [publicKey, invokeContract]
  );

  const heartbeat = useCallback(
    async () => {
      if (!publicKey) throw new Error("Wallet not connected");
      return await invokeContract("heartbeat", [new Address(publicKey)]);
    },
    [publicKey, invokeContract]
  );

  const checkAndRelease = useCallback(
    async () => {
      return await invokeContract("check_and_release", []);
    },
    [invokeContract]
  );

  const getBalance = useCallback(
    async () => {
      return await invokeContract("get_balance", []);
    },
    [invokeContract]
  );

  const getTimeRemaining = useCallback(
    async () => {
      return await invokeContract("time_remaining", []);
    },
    [invokeContract]
  );

  const pause = useCallback(
    async () => {
      if (!publicKey) throw new Error("Wallet not connected");
      return await invokeContract("pause", [new Address(publicKey)]);
    },
    [publicKey, invokeContract]
  );

  const resume = useCallback(
    async () => {
      if (!publicKey) throw new Error("Wallet not connected");
      return await invokeContract("resume", [new Address(publicKey)]);
    },
    [publicKey, invokeContract]
  );

  const updateBeneficiaries = useCallback(
    async (beneficiaries: Beneficiary[]) => {
      if (!publicKey) throw new Error("Wallet not connected");
      
      const total = beneficiaries.reduce((sum, b) => sum + b.basisPoints, 0);
      if (total !== 10000) {
        throw new Error(`Beneficiary allocations must sum to 10000 basis points (got ${total})`);
      }

      const beneArgs = beneficiaries.map(b => [
        new Address(b.address),
        b.basisPoints
      ]);

      return await invokeContract("update_beneficiaries", [
        new Address(publicKey),
        beneArgs
      ]);
    },
    [publicKey, invokeContract]
  );

  const setTokenAddress = useCallback(
    async (tokenAddress: string) => {
      if (!publicKey) throw new Error("Wallet not connected");
      return await invokeContract("set_token_address", [
        new Address(publicKey),
        new Address(tokenAddress)
      ]);
    },
    [publicKey, invokeContract]
  );

  return {
    // State
    publicKey,
    loading,
    error,
    vaultState,

    // Wallet methods
    connect,
    disconnect,

    // Vault methods
    initialize,
    deposit,
    heartbeat,
    checkAndRelease,
    getBalance,
    getTimeRemaining,
    pause,
    resume,
    updateBeneficiaries,
    setTokenAddress,
  };
}
