import React from 'react';
import Form from 'react-bootstrap/Form';
import Button from 'react-bootstrap/Button';
import {useHistory} from 'react-router-dom';
import state from './state';
import './App.css';

async function login(ev, history) {
  ev.preventDefault();
  console.log(ev.target[0].value);
  console.log(ev.target[1].value);

  const response = await fetch("/api/auth/login", {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({
      email: ev.target[0].value,
      password: ev.target[1].value,
    }),
  });

  if (response.ok) {
    const responseBody = await response.json();
    state.token = responseBody.email;
    history.push('/')
  }
}


function App() {
  const history = useHistory();

  return (
    <div className="App">
      <header className="App-header">
        <div className="Login-Form">
          <Form onSubmit={(ev) => login(ev, history)}>
            <Form.Group controlId="formBasicEmail">
              <Form.Label>Email address</Form.Label>
              <Form.Control type="email" placeholder="Enter email" />
              <Form.Text className="text-muted">
                We'll never share your email with anyone else.
                </Form.Text>
            </Form.Group>

            <Form.Group controlId="formBasicPassword">
              <Form.Label>Password</Form.Label>
              <Form.Control type="password" placeholder="Password" />
            </Form.Group>
            <Form.Group controlId="formBasicCheckbox">
              <Form.Check type="checkbox" label="Check me out" />
            </Form.Group>
            <Button variant="primary" type="submit">
              Submit
              </Button>
          </Form>
          <p>
            Edit <code>src/App.js</code> and save to reload.
            </p>
        </div>
      </header>
    </div>
  );
}

export default App;
