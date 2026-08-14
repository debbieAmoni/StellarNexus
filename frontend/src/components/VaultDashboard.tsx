import React, { useEffect, useState } from 'react';
import { useStellarNexus, Beneficiary } from '../hooks/useStellarNexus';
import HeartbeatTimer from './HeartbeatTimer';
import BeneficiaryManager from './BeneficiaryManager';
import './VaultDashboard.css';

const VaultDashboard: React.FC = () => {
  const {
    publicKey,
    loading,
    error,
    vaultState,
    connect,
    disconnect,
    deposit,
    heartbeat,
    getBalance,
    getTimeRemaining,
    pause,
    resume,
  } = useStellarNexus();

  const [depositAmount, setDepositAmount] = useState('');
  const [balance, setBalance] = useState<bigint | null>(null);
  const [timeRemaining, setTimeRemaining] = useState<bigint | null>(null);
  const [isPaused, setIsPaused] = useState(false);
  const [txMessage, setTxMessage] = useState('');

  // Fetch vault state on mount and periodically
  useEffect(() => {
    if (publicKey) {
      const fetchState = async () => {
        try {
          const bal = await getBalance();
          setBalance(bal as bigint);
          
          const time = await getTimeRemaining();
          setTimeRemaining(time as bigint);
        } catch (err) {
          console.error('Failed to fetch state:', err);
        }
      };

      fetchState();
      const interval = setInterval(fetchState, 30000); // Refresh every 30 seconds
      return () => clearInterval(interval);
    }
  }, [publicKey, getBalance, getTimeRemaining]);

  const handleConnect = async () => {
    try {
      await connect();
      setTxMessage('Wallet connected!');
    } catch (err) {
      setTxMessage(`Connection failed: ${err instanceof Error ? err.message : 'Unknown error'}`);
    }
  };

  const handleDisconnect = () => {
    disconnect();
    setTxMessage('Wallet disconnected');
  };

  const handleDeposit = async () => {
    if (!depositAmount || parseFloat(depositAmount) <= 0) {
      setTxMessage('Please enter a valid amount');
      return;
    }

    try {
      setTxMessage('Processing deposit...');
      // Convert to stroops (1 XLM = 10^7 stroops)
      const stroops = BigInt(Math.floor(parseFloat(depositAmount) * 10000000));
      await deposit(stroops);
      setTxMessage('Deposit successful!');
      setDepositAmount('');
      
      // Refresh balance
      const bal = await getBalance();
      setBalance(bal as bigint);
    } catch (err) {
      setTxMessage(`Deposit failed: ${err instanceof Error ? err.message : 'Unknown error'}`);
    }
  };

  const handleHeartbeat = async () => {
    try {
      setTxMessage('Sending heartbeat...');
      await heartbeat();
      setTxMessage('Heartbeat sent! Timer reset.');
      
      // Refresh time remaining
      const time = await getTimeRemaining();
      setTimeRemaining(time as bigint);
    } catch (err) {
      setTxMessage(`Heartbeat failed: ${err instanceof Error ? err.message : 'Unknown error'}`);
    }
  };

  const handlePause = async () => {
    try {
      setTxMessage('Pausing vault...');
      await pause();
      setTxMessage('Vault paused! Drip release is suspended.');
      setIsPaused(true);
    } catch (err) {
      setTxMessage(`Pause failed: ${err instanceof Error ? err.message : 'Unknown error'}`);
    }
  };

  const handleResume = async () => {
    try {
      setTxMessage('Resuming vault...');
      await resume();
      setTxMessage('Vault resumed!');
      setIsPaused(false);
    } catch (err) {
      setTxMessage(`Resume failed: ${err instanceof Error ? err.message : 'Unknown error'}`);
    }
  };

  const formatBalance = (bal: bigint | null): string => {
    if (!bal) return '0';
    return (Number(bal) / 10000000).toFixed(2);
  };

  const formatTimeRemaining = (time: bigint | null): string => {
    if (!time || time === 0n) return 'Grace period elapsed';
    const seconds = Number(time);
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    return `${days} days, ${hours} hours`;
  };

  return (
    <div className="vault-dashboard">
      <header className="dashboard-header">
        <h1>⚡ StellarNexus Vault</h1>
        <div className="wallet-section">
          {!publicKey ? (
            <button className="btn btn-primary" onClick={handleConnect} disabled={loading}>
              {loading ? 'Connecting...' : 'Connect Wallet'}
            </button>
          ) : (
            <div className="wallet-info">
              <span className="wallet-address">
                {publicKey.substring(0, 8)}...{publicKey.substring(publicKey.length - 8)}
              </span>
              <button className="btn btn-secondary" onClick={handleDisconnect}>
                Disconnect
              </button>
            </div>
          )}
        </div>
      </header>

      {error && <div className="error-banner">{error}</div>}
      {txMessage && <div className="info-banner">{txMessage}</div>}

      {publicKey ? (
        <div className="dashboard-content">
          {/* Balance Section */}
          <section className="card">
            <h2>💰 Vault Balance</h2>
            <div className="balance-display">
              <span className="amount">{formatBalance(balance)} XLM</span>
              <button 
                className="btn btn-primary" 
                onClick={() => setTxMessage('Navigate to deposit form')}
              >
                View Details
              </button>
            </div>
          </section>

          {/* Deposit Section */}
          <section className="card">
            <h2>📥 Deposit Funds</h2>
            <div className="form-group">
              <input
                type="number"
                placeholder="Amount (XLM)"
                value={depositAmount}
                onChange={(e) => setDepositAmount(e.target.value)}
                disabled={loading}
                step="0.1"
                min="0"
              />
              <button 
                className="btn btn-primary" 
                onClick={handleDeposit}
                disabled={loading || !depositAmount}
              >
                {loading ? 'Processing...' : 'Deposit'}
              </button>
            </div>
          </section>

          {/* Heartbeat Section */}
          <section className="card">
            <h2>💓 Heartbeat Monitor</h2>
            <div className="heartbeat-section">
              <HeartbeatTimer timeRemaining={timeRemaining} />
              <button 
                className="btn btn-primary" 
                onClick={handleHeartbeat}
                disabled={loading}
              >
                {loading ? 'Sending...' : 'Send Heartbeat'}
              </button>
              <p className="time-remaining">
                Time until drip: <strong>{formatTimeRemaining(timeRemaining)}</strong>
              </p>
            </div>
          </section>

          {/* Vault Control Section */}
          <section className="card">
            <h2>⚙️ Vault Controls</h2>
            <div className="controls-grid">
              <button 
                className={`btn ${isPaused ? 'btn-success' : 'btn-warning'}`}
                onClick={isPaused ? handleResume : handlePause}
                disabled={loading}
              >
                {isPaused ? 'Resume Vault' : 'Pause Vault'}
              </button>
              <button 
                className="btn btn-secondary"
                onClick={() => setTxMessage('Beneficiary management coming soon')}
              >
                Manage Beneficiaries
              </button>
            </div>
            {isPaused && <p className="warning">⚠️ Vault is currently paused</p>}
          </section>

          {/* Beneficiaries Section */}
          <section className="card">
            <h2>👥 Beneficiaries</h2>
            <BeneficiaryManager />
          </section>
        </div>
      ) : (
        <div className="empty-state">
          <p>Connect your Freighter wallet to get started</p>
        </div>
      )}

      <footer className="dashboard-footer">
        <p>StellarNexus v1.0.0 | Secure Inheritance on Stellar</p>
      </footer>
    </div>
  );
};

export default VaultDashboard;
