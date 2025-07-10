import React, { useState } from "react";
import { ArrowRight, Shield } from "lucide-react";

const Stake = () => {
  const [stakeAmount, setStakeAmount] = useState("");

  const handleStake = (e) => {
    e.preventDefault();
    console.log("Staked Amount:", stakeAmount);
    alert("Stake submitted ✅");
  };

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-black via-gray-900 to-black px-4">
      <h1 className="text-4xl font-extrabold mb-8 bg-gradient-to-r from-yellow-400 to-pink-500 text-transparent bg-clip-text flex items-center gap-2">
        <Shield className="w-10 h-10" /> Bitcoin Staking
      </h1>

      <form
        onSubmit={handleStake}
        className="bg-black bg-opacity-80 p-8 rounded-3xl shadow-2xl w-full max-w-md border border-gray-700 backdrop-blur-lg"
      >
        {/* Stake Amount */}
        <div className="mb-6">
          <label className="block text-gray-300 mb-2 text-left text-sm">
            Amount to Stake (BTC)
          </label>
          <input
            type="number"
            step="0.0001"
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

      {/* Rewards Section */}
      <div className="mt-10 bg-black bg-opacity-70 p-6 rounded-2xl w-full max-w-md border border-gray-700">
        <h2 className="text-xl font-bold text-white mb-4">Your Rewards</h2>
        <p className="text-lg text-yellow-400">0.0000 BTC</p>
        <button className="mt-4 w-full bg-green-600 text-white py-2 rounded-xl hover:bg-green-700 transition duration-200">
          Claim Rewards
        </button>
      </div>
    </div>
  );
};

export default Stake;
