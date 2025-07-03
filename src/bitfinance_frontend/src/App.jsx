import React, { useState } from "react";
import { bitfinance_backend } from "../../declarations/bitfinance_backend";

// Use the canister ID from the generated bindings
const BACKEND_CANISTER_ID = process.env.CANISTER_ID_BITFINANCE_BACKEND;

function App() {
    const [principal, setPrincipal] = useState(null);
    const [status, setStatus] = useState("Not connected");
    const [amount, setAmount] = useState("");
    const [result, setResult] = useState("");
    const [userData, setUserData] = useState(null);

    // Connect to Plug wallet and set as agent for backend
    const connectPlug = async () => {
        if (!window.ic?.plug) {
            setStatus("Plug wallet not found");
            return;
        }
        const connected = await window.ic.plug.requestConnect({
            whitelist: [BACKEND_CANISTER_ID],
        });
        if (connected) {
            const principalId = await window.ic.plug.getPrincipal();
            setPrincipal(principalId);
            setStatus("Connected: " + principalId);

            // Set Plug as the agent for the backend canister
            if (window.ic.plug.agent) {
                bitfinance_backend.actor = await window.ic.plug.createActor({
                    canisterId: BACKEND_CANISTER_ID,
                    interfaceFactory: bitfinance_backend.idlFactory,
                });
            }
        } else {
            setStatus("Connection failed");
        }
    };

    // Helper to call backend canister methods via JS bindings
    const callBackend = async (method, args = []) => {
        setResult("Loading...");
        try {
            // Use the generated actor for method calls
            const actor = bitfinance_backend.actor || bitfinance_backend;
            let res;
            if (args.length === 0) {
                res = await actor[method]();
            } else {
                res = await actor[method](...args);
            }
            setResult(JSON.stringify(res, null, 2));
            return res;
        } catch (e) {
            setResult("Error: " + e.message);
        }
    };

    // Example: Register user
    const registerUser = () => callBackend("register_user");

    // Example: Deposit ckBTC
    const depositCkbtc = () => {
        if (!amount) return setResult("Enter amount");
        callBackend("deposit_ckbtc", [BigInt(amount)]);
    };

    // Example: Withdraw ckBTC
    const withdrawCkbtc = () => {
        if (!amount) return setResult("Enter amount");
        callBackend("withdraw_ckbtc", [BigInt(amount)]);
    };

    // Example: Get user data
    const getMyData = async () => {
        const res = await callBackend("get_my_data");
        setUserData(res);
    };

    return (
        <div style={{ maxWidth: 600, margin: "2rem auto", fontFamily: "sans-serif" }}>
            <h2>BitFinance DeFi Dapp (Plug Wallet Demo)</h2>
            <button onClick={connectPlug}>
                {principal ? "Connected" : "Connect Plug Wallet"}
            </button>
            <div>Status: {status}</div>
            <hr />
            <button onClick={registerUser}>Register User</button>
            <br /><br />
            <input
                type="number"
                placeholder="Amount (satoshis)"
                value={amount}
                onChange={e => setAmount(e.target.value)}
            />
            <button onClick={depositCkbtc}>Deposit ckBTC</button>
            <button onClick={withdrawCkbtc}>Withdraw ckBTC</button>
            <br /><br />
            <button onClick={getMyData}>Get My Data</button>
            {userData && (
                <pre style={{ background: "#eee", padding: "1em" }}>
                    {JSON.stringify(userData, null, 2)}
                </pre>
            )}
            <hr />
            <div>
                <b>Result:</b>
                <pre style={{ background: "#f8f8f8", padding: "1em" }}>{result}</pre>
            </div>
            <hr />
            <div>
                <b>Other operations:</b>
                <ul>
                    <li>stake_ckbtc(amount)</li>
                    <li>unstake_ckbtc(amount)</li>
                    <li>lend_ckbtc(amount)</li>
                    <li>unlend_ckbtc(amount)</li>
                    <li>yield_farm_ckbtc(amount)</li>
                    <li>unfarm_ckbtc(amount)</li>
                    <li>borrow_ckbtc(amount)</li>
                    <li>repay_loan_ckbtc(amount)</li>
                    <li>claim_staking_rewards()</li>
                    <li>claim_lending_rewards()</li>
                    <li>claim_yield_farming_rewards()</li>
                </ul>
                {/* You can add more buttons/inputs for these as needed */}
            </div>
            <div style={{ marginTop: "2em", fontSize: "0.9em", color: "#888" }}>
                <b>Note:</b> This is a minimal demo. For production, use your generated JS bindings and handle Plug wallet errors and user flows more robustly.
            </div>
        </div>
    );
}

export default App;
