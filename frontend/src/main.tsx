import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import './style.css';

function App() {
  return (
    <main>
      <p className="eyebrow">test</p>
      <h1>testtesttest</h1>
      <dl>
        <dt>Broker WebSocket URL</dt>
        <dd>{import.meta.env.VITE_MQTT_URL || 'ws://127.0.0.1:9001'}</dd>
        <dt>Vehicle state topic</dt>
        <dd>{import.meta.env.VITE_MQTT_TOPIC || 'mav/1/state'}</dd>
      </dl>
      <p>No telemetry connection.</p>
    </main>
  );
}

createRoot(document.getElementById('root')!).render(
  <StrictMode><App /></StrictMode>,
);
