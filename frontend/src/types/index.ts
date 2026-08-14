/// Type definitions for StellarNexus

export interface Beneficiary {
  address: string;
  basisPoints: number;
  percentage: number;
}

export interface VaultState {
  owner: string;
  balance: number;
  beneficiaries: Beneficiary[];
  timeRemaining: number;
  isPaused: boolean;
  lastHeartbeat: number;
  tokenAddress: string | null;
}

export interface ContractError {
  code: number;
  message: string;
}

export type VaultErrorType =
  | 'AlreadyInitialized'
  | 'InvalidAllocations'
  | 'Unauthorized'
  | 'NotInitialized'
  | 'InvalidAmount'
  | 'InsufficientBalance'
  | 'InvalidBeneficiary';

export interface TransactionResult {
  success: boolean;
  txHash?: string;
  error?: string;
}

export interface WalletState {
  connected: boolean;
  publicKey: string | null;
  network: 'testnet' | 'mainnet' | null;
}
