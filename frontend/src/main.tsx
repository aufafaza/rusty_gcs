import { StrictMode, useEffect, useState } from 'react';
import { createRoot } from 'react-dom/client';
import './style.css';
import mqtt from 'mqtt'; 

function App() {
  const [payload, setPayload] = useState('waiting for telemetry'); 

  useEffect(() => {
    const client = mqtt.connect(import.meta.env.VITE_MQTT_URL || 'ws://127.0.0.1:9001');

    const topic = import.meta.env.VITE_MQTT_TOPIC || 'mav/1/state'; 

    client.on('connect', () => { 
      client.subscribe(topic);
    });

    client.on('message', (receivedTopic, message) => { 
      if (receivedTopic === topic) { 
        setPayload(message.toString()); 
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
      <h1> telemetry </h1> 
      <pre>{payload}</pre> 
    </main>
  );
}

createRoot(document.getElementById('root')!).render(
  <StrictMode><App /></StrictMode>,
);
