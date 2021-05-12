import React, { useEffect, useState } from 'react';
import { Redirect } from 'react-router-dom';

async function auth() {
  const response = await fetch("/api/auth/auth");

  return response.ok;
}

const withAuth = (Component) => {

  const AuthRoute = () => {
    const [loggedIn, setLoggedIn] = useState(undefined);

    useEffect(() => {
      (async function () {
        const res = await auth()

        setLoggedIn(res);
      })();
    }, [setLoggedIn])

    if (loggedIn === true) {
      return <Component />
    } else if (loggedIn === false) {
      return <Redirect to="/login" />
    } else {
      return null;
    }
  }
  

  return AuthRoute;
}

export default withAuth;
