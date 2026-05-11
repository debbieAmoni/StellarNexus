interface Props {
  onHeartbeat: () => Promise<unknown>;
  loading: boolean;
}

export function HeartbeatTimer({ onHeartbeat, loading }: Props) {
  // TODO: fetch time_remaining from contract and display live countdown
  return (
    <section className="heartbeat-timer">
      <h2>💓 Heartbeat</h2>
      <p>Send a heartbeat every 180 days to keep your vault active.</p>
      <button onClick={onHeartbeat} disabled={loading}>
        {loading ? "Sending…" : "Send Heartbeat"}
      </button>
    </section>
  );
}
