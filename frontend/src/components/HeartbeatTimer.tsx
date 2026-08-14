import React, { useEffect, useState } from 'react';
import './HeartbeatTimer.css';

interface HeartbeatTimerProps {
  timeRemaining: bigint | null;
}

const HeartbeatTimer: React.FC<HeartbeatTimerProps> = ({ timeRemaining }) => {
  const [displayTime, setDisplayTime] = useState({
    days: 0,
    hours: 0,
    minutes: 0,
    seconds: 0,
  });
  const [isExpired, setIsExpired] = useState(false);

  useEffect(() => {
    if (!timeRemaining) return;

    const seconds = Number(timeRemaining);
    
    if (seconds <= 0) {
      setIsExpired(true);
      setDisplayTime({ days: 0, hours: 0, minutes: 0, seconds: 0 });
      return;
    }

    setIsExpired(false);

    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;

    setDisplayTime({ days, hours, minutes, seconds: secs });

    // Update every second
    const interval = setInterval(() => {
      const newSeconds = Math.max(0, seconds - 1);
      const newDays = Math.floor(newSeconds / 86400);
      const newHours = Math.floor((newSeconds % 86400) / 3600);
      const newMinutes = Math.floor((newSeconds % 3600) / 60);
      const newSecs = newSeconds % 60;

      setDisplayTime({ days: newDays, hours: newHours, minutes: newMinutes, seconds: newSecs });

      if (newSeconds <= 0) {
        setIsExpired(true);
        clearInterval(interval);
      }
    }, 1000);

    return () => clearInterval(interval);
  }, [timeRemaining]);

  return (
    <div className={`heartbeat-timer ${isExpired ? 'expired' : 'active'}`}>
      <div className="timer-display">
        <div className="timer-unit">
          <span className="value">{String(displayTime.days).padStart(2, '0')}</span>
          <span className="label">Days</span>
        </div>
        <div className="separator">:</div>
        <div className="timer-unit">
          <span className="value">{String(displayTime.hours).padStart(2, '0')}</span>
          <span className="label">Hours</span>
        </div>
        <div className="separator">:</div>
        <div className="timer-unit">
          <span className="value">{String(displayTime.minutes).padStart(2, '0')}</span>
          <span className="label">Min</span>
        </div>
        <div className="separator">:</div>
        <div className="timer-unit">
          <span className="value">{String(displayTime.seconds).padStart(2, '0')}</span>
          <span className="label">Sec</span>
        </div>
      </div>
      {isExpired && (
        <div className="expiration-notice">
          ⚠️ Grace period elapsed - Drip release activated
        </div>
      )}
    </div>
  );
};

export default HeartbeatTimer;
