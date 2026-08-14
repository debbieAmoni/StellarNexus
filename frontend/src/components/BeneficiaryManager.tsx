import React, { useState } from 'react';
import { useStellarNexus, Beneficiary } from '../hooks/useStellarNexus';
import './BeneficiaryManager.css';

const BeneficiaryManager: React.FC = () => {
  const { publicKey, loading, updateBeneficiaries } = useStellarNexus();
  const [beneficiaries, setBeneficiaries] = useState<Beneficiary[]>([
    { address: '', basisPoints: 5000 },
    { address: '', basisPoints: 5000 },
  ]);
  const [message, setMessage] = useState('');
  const [totalPercentage, setTotalPercentage] = useState(100);

  const handleAddBeneficiary = () => {
    setBeneficiaries([...beneficiaries, { address: '', basisPoints: 0 }]);
  };

  const handleRemoveBeneficiary = (index: number) => {
    const updated = beneficiaries.filter((_, i) => i !== index);
    setBeneficiaries(updated);
    updateTotalPercentage(updated);
  };

  const handleAddressChange = (index: number, value: string) => {
    const updated = [...beneficiaries];
    updated[index].address = value;
    setBeneficiaries(updated);
  };

  const handlePercentageChange = (index: number, value: string) => {
    const percentage = Math.min(100, Math.max(0, parseInt(value) || 0));
    const basisPoints = percentage * 100;
    
    const updated = [...beneficiaries];
    updated[index].basisPoints = basisPoints;
    setBeneficiaries(updated);
    updateTotalPercentage(updated);
  };

  const updateTotalPercentage = (benes: Beneficiary[]) => {
    const total = benes.reduce((sum, b) => sum + b.basisPoints, 0) / 100;
    setTotalPercentage(total);
  };

  const handleSave = async () => {
    if (totalPercentage !== 100) {
      setMessage(`Allocations must sum to 100% (currently ${totalPercentage}%)`);
      return;
    }

    if (beneficiaries.some(b => !b.address)) {
      setMessage('All beneficiary addresses must be filled');
      return;
    }

    try {
      setMessage('Updating beneficiaries...');
      await updateBeneficiaries(beneficiaries);
      setMessage('Beneficiaries updated successfully!');
    } catch (err) {
      setMessage(`Update failed: ${err instanceof Error ? err.message : 'Unknown error'}`);
    }
  };

  const percentageValid = totalPercentage === 100;

  return (
    <div className="beneficiary-manager">
      <div className="beneficiary-list">
        {beneficiaries.map((ben, index) => (
          <div key={index} className="beneficiary-item">
            <div className="beneficiary-inputs">
              <div className="form-group">
                <label>Address</label>
                <input
                  type="text"
                  placeholder="Stellar address (G...)"
                  value={ben.address}
                  onChange={(e) => handleAddressChange(index, e.target.value)}
                  disabled={loading}
                />
              </div>
              <div className="form-group">
                <label>Allocation %</label>
                <div className="percentage-input">
                  <input
                    type="number"
                    min="0"
                    max="100"
                    value={Math.round(ben.basisPoints / 100)}
                    onChange={(e) => handlePercentageChange(index, e.target.value)}
                    disabled={loading}
                  />
                  <span>%</span>
                </div>
              </div>
              {beneficiaries.length > 1 && (
                <button
                  className="btn btn-remove"
                  onClick={() => handleRemoveBeneficiary(index)}
                  disabled={loading}
                >
                  Remove
                </button>
              )}
            </div>
          </div>
        ))}
      </div>

      <div className="beneficiary-controls">
        <button
          className="btn btn-secondary"
          onClick={handleAddBeneficiary}
          disabled={loading}
        >
          + Add Beneficiary
        </button>
      </div>

      <div className={`allocation-summary ${percentageValid ? 'valid' : 'invalid'}`}>
        <div className="total-percentage">
          Total: <strong>{totalPercentage}%</strong>
        </div>
        <div className="percentage-bar">
          <div className="percentage-fill" style={{ width: `${Math.min(100, totalPercentage)}%` }} />
        </div>
        {!percentageValid && (
          <div className="percentage-warning">
            ⚠️ Allocations must sum to exactly 100%
          </div>
        )}
      </div>

      {message && (
        <div className={`message ${message.includes('Update failed') || message.includes('must') ? 'error' : 'info'}`}>
          {message}
        </div>
      )}

      <button
        className="btn btn-primary btn-save"
        onClick={handleSave}
        disabled={loading || !percentageValid || !publicKey}
      >
        {loading ? 'Saving...' : 'Save Beneficiaries'}
      </button>
    </div>
  );
};

export default BeneficiaryManager;
