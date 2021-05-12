import React, { useState, useEffect } from 'react';
import withAuth from './withAuth';
import state from './state';

async function priv() {
  const response = await fetch('/api/shipments', {
    headers: {
      Authorization: `Bearer ${state.token}`,
    },
  });

  if (response.ok) {
    return await response.text();
  } else {
    return await response.text();
  }
}

function Home() {
  const [msg, setMsg] = useState("")

  useEffect(() => {
    (async function() {
      const msg = await priv();
      setMsg(msg);
    })();
  }, [])

  return (
    <div className="App">
      <h1>{msg}</h1>
    </div>
  );
}

export default withAuth(Home);
