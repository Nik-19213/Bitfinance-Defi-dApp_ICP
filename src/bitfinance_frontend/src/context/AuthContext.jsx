import React, { createContext, useState, useEffect } from "react";
import { AuthClient } from "@dfinity/auth-client";

export const AuthContext = createContext();

export const AuthProvider = ({ children }) => {
  const [authClient, setAuthClient] = useState(null);
  const [principal, setPrincipal] = useState(null);

  useEffect(() => {
    const setupAuth = async () => {
      const client = await AuthClient.create();
      setAuthClient(client);

      const identity = client.getIdentity();
      const principalText = identity.getPrincipal().toText();
      if (principalText !== "2vxsx-fae") {
        setPrincipal(principalText);
      }
    };

    setupAuth();
  }, []);

  const login = async () => {
    if (!authClient) return;
    await authClient.login({
      identityProvider: "https://identity.ic0.app",
      onSuccess: async () => {
        const identity = authClient.getIdentity();
        setPrincipal(identity.getPrincipal().toText());
      },
    });
  };

  const logout = async () => {
    if (!authClient) return;
    await authClient.logout();
    setPrincipal(null);
  };

  return (
    <AuthContext.Provider value={{ principal, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
};
