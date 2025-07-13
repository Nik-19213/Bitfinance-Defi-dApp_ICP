import React, { useState, useContext, useEffect } from "react";
import { ArrowRight, Shield } from "lucide-react";
import { bitfinance_backend } from "../../../declarations/bitfinance_backend";
import { AuthContext } from "../context/AuthContext";

const Stake = () => {
  const [stakeAmount, setStakeAmount] = useState("");
  const [unstakeAmount, setUnstakeAmount] = useState("");
  const { principal } = useContext(AuthContext);

  // Display staked amount and timestamp
  const [userData, setUserData] = useState(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const data = await bitfinance_backend.get_my_data();
        if (data && data.length > 0) setUserData(data[0]);
        else setUserData({ staked: 0, stake_timestamp: null });
      } catch (err) {
        setUserData({ staked: 0, stake_timestamp: null });
      }
    };
    fetchData();
  }, [principal]);

  const handleStake = async (e) => {
    e.preventDefault();
    try {
      const result = await bitfinance_backend.stake_ckbtc(Number(stakeAmount));
      alert(result);
    } catch (err) {
      alert("Stake failed: " + err);
    }
  };

  const handleUnstake = async (e) => {
    e.preventDefault();
    try {
      const result = await bitfinance_backend.unstake_ckbtc(Number(unstakeAmount));
      alert(result);
    } catch (err) {
      alert("Unstake failed: " + err);
    }
  };

  const handleClaim = async () => {
    try {
      const result = await bitfinance_backend.claim_staking_rewards();
      alert(result);
    } catch (err) {
      alert("Claim failed: " + err);
    }
  };

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-black via-gray-900 to-black px-4 relative">
      <div className="w-full flex flex-col md:flex-row md:justify-between md:items-start">
        <h1 className="text-4xl font-extrabold mb-8 bg-gradient-to-r from-yellow-400 to-pink-500 text-transparent bg-clip-text flex items-center gap-2">
          ckBTC Staking
        </h1>
        {/* Staked Amount Display - right upper corner */}
        <div className="mb-6 md:mb-0 md:ml-auto bg-gray-900 rounded-xl px-6 py-4 border border-yellow-500 text-center min-w-[260px]">
          <div className="text-lg text-yellow-400 font-bold">
            Staked: {(Number(userData?.staked ?? 0) / 1e8).toFixed(8)} ckBTC
          </div>
          <div className="text-gray-400 text-sm">
            {userData && userData.stake_timestamp && Number(userData.stake_timestamp) > 0
              ? "Since: " +
              new Date(Number(userData.stake_timestamp) * 1000).toLocaleString()
              : "No active stake"}
          </div>
        </div>
      </div>

      <form
        onSubmit={handleStake}
        className="bg-black bg-opacity-80 p-8 rounded-3xl shadow-2xl w-full max-w-md border border-gray-700 backdrop-blur-lg"
      >
        {/* Stake Amount */}
        <h2 className="text-xl font-bold text-white mb-4">Stake</h2>
        <div className="mb-6">
          <label className="block text-gray-300 mb-2 text-left text-sm">
            Amount (ckBTC)
          </label>
          <input
            type="number"
            step="0.00000001"
            min="0"
            value={stakeAmount}
            onChange={(e) => setStakeAmount(e.target.value)}
            required
            className="w-full px-4 py-3 border border-gray-600 rounded-xl bg-gray-800 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500"
          />
        </div>

        {/* Stake Button */}
        <button
          type="submit"
          className="w-full flex items-center justify-center gap-2 bg-yellow-400 text-black py-3 rounded-xl font-semibold hover:bg-yellow-300 active:scale-95 transition duration-200 shadow-lg"
        >
          Stake Now <ArrowRight size={18} />
        </button>
      </form>

      {/* Unstake Section */}
      <form
        onSubmit={handleUnstake}
        className="bg-black bg-opacity-70 p-6 rounded-2xl w-full max-w-md border border-gray-700 mt-8"
      >
        <h2 className="text-xl font-bold text-white mb-4">Unstake</h2>
        <div className="mb-4">
          <label className="block text-gray-300 mb-2 text-left text-sm">
            Amount (ckBTC)
          </label>
          <input
            type="number"
            step="0.00000001"
            min="0"
            value={unstakeAmount}
            onChange={(e) => setUnstakeAmount(e.target.value)}
            required
            className="w-full px-4 py-3 border border-gray-600 rounded-xl bg-gray-800 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500"
          />
        </div>
        <button
          type="submit"
          className="w-full bg-yellow-500 text-black py-2 rounded-xl font-semibold hover:bg-yellow-600 transition duration-200 shadow-lg"
        >
          Unstake
        </button>
      </form>
    </div>
  );
};

export default Stake;
