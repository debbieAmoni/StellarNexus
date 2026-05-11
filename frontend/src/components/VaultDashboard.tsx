import { useStellarNexus } from "../hooks/useStellarNexus";
import { HeartbeatTimer } from "./HeartbeatTimer";
import { BeneficiaryManager } from "./BeneficiaryManager";

export function VaultDashboard() {
  const { publicKey, loading, error, connect, heartbeat, deposit } = useStellarNexus();

  if (!publicKey) {
    return (
      <div className="vault-dashboard">
        <h1>🏺 StellarNexus</h1>
        <button onClick={connect}>Connect Freighter</button>
      </div>
    );
  }

  return (
    <div className="vault-dashboard">
      <h1>🏺 StellarNexus</h1>
      <p>Owner: {publicKey}</p>
      {error && <p className="error">{error}</p>}

      <HeartbeatTimer onHeartbeat={heartbeat} loading={loading} />
      <BeneficiaryManager />

      <section>
        <h2>Deposit</h2>
        <button
          onClick={() => deposit(BigInt(1_000_000))}
          disabled={loading}
        >
          Deposit 1 XLM
        </button>
      </section>
    </div>
  );
}
