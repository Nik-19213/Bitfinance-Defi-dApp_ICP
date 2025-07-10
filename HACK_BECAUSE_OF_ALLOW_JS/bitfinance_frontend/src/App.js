import React, { useState } from "react";
import { bitfinance_backend } from "../../declarations/bitfinance_backend";
const BACKEND_CANISTER_ID = process.env.CANISTER_ID_BITFINANCE_BACKEND;
function App() {
    const [principal, setPrincipal] = useState(null);
    const [status, setStatus] = useState("Not connected");
    const [amount, setAmount] = useState("");
    const [result, setResult] = useState("");
    const [userData, setUserData] = useState(null);
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
            if (window.ic.plug.agent) {
                bitfinance_backend.actor = await window.ic.plug.createActor({
                    canisterId: BACKEND_CANISTER_ID,
                    interfaceFactory: bitfinance_backend.idlFactory,
                });
            }
        }
        else {
            setStatus("Connection failed");
        }
    };
    const callBackend = async (method, args = []) => {
        setResult("Loading...");
        try {
            const actor = bitfinance_backend.actor || bitfinance_backend;
            const res = args.length ? await actor[method](...args) : await actor[method]();
            setResult(JSON.stringify(res, null, 2));
            return res;
        }
        catch (e) {
            setResult("Error: " + e.message);
        }
    };
    const registerUser = () => callBackend("register_user");
    const depositCkbtc = () => {
        if (!amount)
            return setResult("Enter amount");
        callBackend("deposit_ckbtc", [BigInt(amount)]);
    };
    const withdrawCkbtc = () => {
        if (!amount)
            return setResult("Enter amount");
        callBackend("withdraw_ckbtc", [BigInt(amount)]);
    };
    const getMyData = async () => {
        const res = await callBackend("get_my_data");
        setUserData(res);
    };
    const stakeCkbtc = () => {
        if (!amount)
            return setResult("Enter amount");
        callBackend("stake_ckbtc", [BigInt(amount)]);
    };
    const unstakeCkbtc = () => {
        if (!amount)
            return setResult("Enter amount");
        callBackend("unstake_ckbtc", [BigInt(amount)]);
    };
    const lendCkbtc = () => {
        if (!amount)
            return setResult("Enter amount");
        callBackend("lend_ckbtc", [BigInt(amount)]);
    };
    const unlendCkbtc = () => {
        if (!amount)
            return setResult("Enter amount");
        callBackend("unlend_ckbtc", [BigInt(amount)]);
    };
    const yieldFarmCkbtc = () => {
        if (!amount)
            return setResult("Enter amount");
        callBackend("yield_farm_ckbtc", [BigInt(amount)]);
    };
    const unfarmCkbtc = () => {
        if (!amount)
            return setResult("Enter amount");
        callBackend("unfarm_ckbtc", [BigInt(amount)]);
    };
    const borrowCkbtc = () => {
        if (!amount)
            return setResult("Enter amount");
        callBackend("borrow_ckbtc", [BigInt(amount)]);
    };
    const repayLoanCkbtc = () => {
        if (!amount)
            return setResult("Enter amount");
        callBackend("repay_loan_ckbtc", [BigInt(amount)]);
    };
    const claimStakingRewards = () => callBackend("claim_staking_rewards");
    const claimLendingRewards = () => callBackend("claim_lending_rewards");
    const claimYieldFarmingRewards = () => callBackend("claim_yield_farming_rewards");
    const CKBTC_CANISTER_ID = "uxrrr-q7777-77774-qaaaq-cai";
    const BACKEND_CANISTER_ID_DISPLAY = BACKEND_CANISTER_ID || "your-backend-canister-id";
    return (<div className="max-w-3xl mx-auto p-6 font-sans text-gray-800">
            <h2 className="text-2xl font-bold mb-4 text-center">BitFinance DeFi Dapp (Plug Wallet Demo)</h2>

            <button onClick={connectPlug} className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700">
                {principal ? "Connected" : "Connect Plug Wallet"}
            </button>

            <div className="mt-2 text-sm text-gray-600">Status: {status}</div>
            <hr className="my-4"/>

            <button onClick={registerUser} className="bg-green-500 text-white px-4 py-2 rounded hover:bg-green-600">
                Register User
            </button>

            <div className="my-4 flex gap-2 items-center">
                <input type="number" placeholder="Amount (satoshis)" value={amount} onChange={e => setAmount(e.target.value)} className="border px-3 py-2 rounded w-full"/>
            </div>

            <div className="grid grid-cols-2 md:grid-cols-3 gap-2 my-2">
                <button onClick={depositCkbtc} className="btn">Deposit</button>
                <button onClick={withdrawCkbtc} className="btn">Withdraw</button>
                <button onClick={stakeCkbtc} className="btn">Stake</button>
                <button onClick={unstakeCkbtc} className="btn">Unstake</button>
                <button onClick={lendCkbtc} className="btn">Lend</button>
                <button onClick={unlendCkbtc} className="btn">Unlend</button>
                <button onClick={yieldFarmCkbtc} className="btn">Yield Farm</button>
                <button onClick={unfarmCkbtc} className="btn">Unfarm</button>
                <button onClick={borrowCkbtc} className="btn">Borrow</button>
                <button onClick={repayLoanCkbtc} className="btn">Repay Loan</button>
                <button onClick={claimStakingRewards} className="btn">Claim Staking</button>
                <button onClick={claimLendingRewards} className="btn">Claim Lending</button>
                <button onClick={claimYieldFarmingRewards} className="btn">Claim Farming</button>
            </div>

            <button onClick={getMyData} className="bg-purple-600 text-white px-4 py-2 rounded hover:bg-purple-700 my-4">
                Get My Data
            </button>

            {userData && (<pre className="bg-gray-100 p-4 rounded text-sm overflow-x-auto">
                    {JSON.stringify(userData, null, 2)}
                </pre>)}

            <hr className="my-4"/>
            <div>
                <h3 className="text-lg font-semibold mb-2">Result:</h3>
                <pre className="bg-gray-100 p-4 rounded text-sm overflow-x-auto">{result}</pre>
            </div>

            <div className="mt-8 text-sm text-gray-600">
                <b>Other operations:</b>
                <ul className="list-disc pl-5">
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
            </div>

            <div className="bg-yellow-100 border border-yellow-300 p-4 mt-6 rounded">
                <h4 className="font-semibold mb-2">🔑 Before using DeFi functions:</h4>
                <ol className="list-decimal pl-5 text-sm">
                    <li>Open your Plug wallet.</li>
                    <li>Go to the <b>ckBTC</b> token.</li>
                    <li>Find and use the <b>Approve</b> function.</li>
                    <li>
                        Approve <b>{BACKEND_CANISTER_ID_DISPLAY}</b> to spend your ckBTC.<br />
                        <span className="text-xs text-gray-600">
                            <b>ckBTC Canister ID:</b> {CKBTC_CANISTER_ID}
                        </span>
                    </li>
                    <li>Set the amount you want to allow for DeFi operations.</li>
                    <li>After approving, you can deposit, stake, lend, etc.</li>
                </ol>
                <div className="text-xs text-gray-600 mt-2">
                    <b>Note:</b> Approval is a one-time action per amount. If you want to increase your limit, approve again.
                </div>
            </div>
        </div>);
}
export default App;
