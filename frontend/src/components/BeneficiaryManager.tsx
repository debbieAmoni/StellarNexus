import { useState } from "react";

interface Beneficiary {
  address: string;
  basisPoints: number;
}

export function BeneficiaryManager() {
  const [beneficiaries, setBeneficiaries] = useState<Beneficiary[]>([]);
  const [address, setAddress] = useState("");
  const [bps, setBps] = useState("");

  const add = () => {
    const points = parseInt(bps, 10);
    if (!address || isNaN(points)) return;
    setBeneficiaries((prev) => [...prev, { address, basisPoints: points }]);
    setAddress("");
    setBps("");
  };

  const total = beneficiaries.reduce((sum, b) => sum + b.basisPoints, 0);

  return (
    <section className="beneficiary-manager">
      <h2>👥 Beneficiaries</h2>
      <ul>
        {beneficiaries.map((b) => (
          <li key={b.address}>
            {b.address} — {b.basisPoints / 100}%
          </li>
        ))}
      </ul>
      <p>Total allocated: {total / 100}% {total !== 10_000 && <span>(must reach 100%)</span>}</p>
      <input placeholder="Stellar address" value={address} onChange={(e) => setAddress(e.target.value)} />
      <input placeholder="Basis points (e.g. 5000 = 50%)" value={bps} onChange={(e) => setBps(e.target.value)} />
      <button onClick={add}>Add Beneficiary</button>
      {/* TODO: wire submit to contract initialize / update_beneficiaries */}
    </section>
  );
}
