import { StrictMode, useEffect, useState } from 'react';
import { createRoot } from 'react-dom/client';
import './style.css';
import mqtt from 'mqtt'; 
import { state, StateModel } from './lib/store';

function App() {
  const [v, setV] = useState<StateModel | null>(null); 

  useEffect(() => {
    const client = mqtt.connect(import.meta.env.VITE_MQTT_URL || 'ws://127.0.0.1:9001');

    const topic = import.meta.env.VITE_MQTT_TOPIC || 'mav/1/state'; 
    client.on('connect', () => { 
      client.subscribe(topic);
    });
    
    client.on('message', (receivedTopic, message) => { 
      if (receivedTopic === topic) { 
        try { 
          const data = JSON.parse(message.toString()) as StateModel; 
          setV(data); 
        } catch (error) { 
          console.error('invalid: ', error); 
        } 
      } 
    });

    client.on('error', (error) => { 
      console.error('MQTT error: ', error); 
    });

    return () => { 
      client.end(); 
    };
  }, [])
return (
  <main>
    <h1>Vehicle telemetry</h1>

    {v === null ? (
      <p>Waiting for telemetry…</p>
    ) : (
      <dl>
        <dt>Vehicle ID</dt>
        <dd>{v.system_id}</dd>

        <dt>Connection</dt>
        <dd>{v.connected ? 'Connected' : 'Disconnected'}</dd>

        <dt>Latitude</dt>
        <dd>{v.latitude_deg?.toFixed(7) ?? 'Unknown'}</dd>

        <dt>Longitude</dt>
        <dd>{v.longitude_deg?.toFixed(7) ?? 'Unknown'}</dd>

        <dt>Altitude above sea level</dt>
        <dd>
          {v.altitude_m === null
            ? 'Unknown'
            : `${v.altitude_m.toFixed(1)} m`}
        </dd>
      </dl>
    )}
  </main>
);
}

createRoot(document.getElementById('root')!).render(
  <StrictMode><App /></StrictMode>,
);
