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

    // Add handlers for all DeFi operations
    const stakeCkbtc = () => {
        if (!amount) return setResult("Enter amount");
        callBackend("stake_ckbtc", [BigInt(amount)]);
    };
    const unstakeCkbtc = () => {
        if (!amount) return setResult("Enter amount");
        callBackend("unstake_ckbtc", [BigInt(amount)]);
    };
    const lendCkbtc = () => {
        if (!amount) return setResult("Enter amount");
        callBackend("lend_ckbtc", [BigInt(amount)]);
    };
    const unlendCkbtc = () => {
        if (!amount) return setResult("Enter amount");
        callBackend("unlend_ckbtc", [BigInt(amount)]);
    };
    const yieldFarmCkbtc = () => {
        if (!amount) return setResult("Enter amount");
        callBackend("yield_farm_ckbtc", [BigInt(amount)]);
    };
    const unfarmCkbtc = () => {
        if (!amount) return setResult("Enter amount");
        callBackend("unfarm_ckbtc", [BigInt(amount)]);
    };
    const borrowCkbtc = () => {
        if (!amount) return setResult("Enter amount");
        callBackend("borrow_ckbtc", [BigInt(amount)]);
    };
    const repayLoanCkbtc = () => {
        if (!amount) return setResult("Enter amount");
        callBackend("repay_loan_ckbtc", [BigInt(amount)]);
    };
    const claimStakingRewards = () => callBackend("claim_staking_rewards");
    const claimLendingRewards = () => callBackend("claim_lending_rewards");
    const claimYieldFarmingRewards = () => callBackend("claim_yield_farming_rewards");

    // Add these constants for display
    const CKBTC_CANISTER_ID = "uxrrr-q7777-77774-qaaaq-cai"; // testnet, change if mainnet
    const BACKEND_CANISTER_ID_DISPLAY = BACKEND_CANISTER_ID || "your-backend-canister-id";

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
            <button onClick={stakeCkbtc}>Stake ckBTC</button>
            <button onClick={unstakeCkbtc}>Unstake ckBTC</button>
            <button onClick={lendCkbtc}>Lend ckBTC</button>
            <button onClick={unlendCkbtc}>Unlend ckBTC</button>
            <button onClick={yieldFarmCkbtc}>Yield Farm ckBTC</button>
            <button onClick={unfarmCkbtc}>Unfarm ckBTC</button>
            <button onClick={borrowCkbtc}>Borrow ckBTC</button>
            <button onClick={repayLoanCkbtc}>Repay Loan ckBTC</button>
            <button onClick={claimStakingRewards}>Claim Staking Rewards</button>
            <button onClick={claimLendingRewards}>Claim Lending Rewards</button>
            <button onClick={claimYieldFarmingRewards}>Claim Yield Farming Rewards</button>
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
            <div style={{ background: "#fffbe7", border: "1px solid #ffe58f", padding: "1em", marginTop: "1em" }}>
                <b>🔑 Before using DeFi functions:</b>
                <ol>
                    <li>Open your Plug wallet.</li>
                    <li>Go to the <b>ckBTC</b> token.</li>
                    <li>Find and use the <b>Approve</b> function.</li>
                    <li>
                        Approve <b>{BACKEND_CANISTER_ID_DISPLAY}</b> (your DeFi backend canister) to spend your ckBTC.<br />
                        <span style={{ fontSize: "0.9em" }}>
                            <b>ckBTC Canister ID:</b> {CKBTC_CANISTER_ID}
                        </span>
                    </li>
                    <li>Set the amount you want to allow for DeFi operations.</li>
                    <li>After approving, you can deposit, stake, lend, etc.</li>
                </ol>
                <div style={{ fontSize: "0.9em", color: "#888" }}>
                    <b>Note:</b> Approval is a one-time action per amount. If you want to increase your limit, approve again.
                </div>
            </div>
        </div>
    );
}

export default App;
