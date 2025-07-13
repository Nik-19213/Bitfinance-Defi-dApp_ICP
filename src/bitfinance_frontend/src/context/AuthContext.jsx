import React, { createContext, useState, useEffect } from "react";
import { AuthClient } from "@dfinity/auth-client";

export const AuthContext = createContext();

export const AuthProvider = ({ children }) => {
  const [authClient, setAuthClient] = useState(null);
  const [principal, setPrincipal] = useState(null);

  useEffect(() => {
    const setupAuth = async () => {
      try {
        const client = await AuthClient.create();
        setAuthClient(client);

        const identity = client.getIdentity();
        const principalObj = identity.getPrincipal();
        const principalText = principalObj ? principalObj.toText() : null;
        if (principalText && principalText !== "2vxsx-fae") {
          setPrincipal(principalText);
        } else {
          setPrincipal(null);
        }
      } catch (err) {
        console.error("AuthContext setup error:", err);
        setPrincipal(null);
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
        const principalObj = identity.getPrincipal();
        setPrincipal(principalObj ? principalObj.toText() : null);
      },
    });
  };

  const logout = async () => {
    if (!authClient) return;
    await authClient.logout();
    setPrincipal(null);
  };

  // Add a debug log for principal changes
  useEffect(() => {
    console.log("AuthContext principal changed:", principal);
  }, [principal]);

  return (
    <AuthContext.Provider value={{ principal, login, logout, authClient }}>
      {children}
    </AuthContext.Provider>
  );
};
