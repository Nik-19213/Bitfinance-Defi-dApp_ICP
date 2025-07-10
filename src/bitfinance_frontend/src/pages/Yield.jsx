import React, { useState } from "react";
import { ArrowRight, Leaf } from "lucide-react";

const YieldFarm = () => {
  const [farmAmount, setFarmAmount] = useState("");
  const [tokenType, setTokenType] = useState("ckBTC");

  const handleFarm = (e) => {
    e.preventDefault();
    console.log("Farming Amount:", farmAmount);
    console.log("Token Type:", tokenType);
    alert("Yield Farming request submitted ✅");
  };

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-black via-gray-900 to-black px-4">
      <h1 className="text-4xl font-extrabold mb-8 bg-gradient-to-r from-yellow-400 to-pink-500 text-transparent bg-clip-text flex items-center gap-2">
        <Leaf className="w-10 h-10" /> Yield Farming
      </h1>

      <form
        onSubmit={handleFarm}
        className="bg-black bg-opacity-80 p-8 rounded-3xl shadow-2xl w-full max-w-md border border-gray-700 backdrop-blur-lg"
      >
        {/* Token Type */}
        <div className="mb-6">
          <label className="block text-gray-300 mb-2 text-left text-sm">
            Select Token
          </label>
          <select
            value={tokenType}
            onChange={(e) => setTokenType(e.target.value)}
            className="w-full px-4 py-3 border border-gray-600 rounded-xl bg-gray-800 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500"
          >
            <option value="ckBTC">ckBTC</option>
            <option value="LP">LP Token</option>
          </select>
        </div>

        {/* Farming Amount */}
        <div className="mb-6">
          <label className="block text-gray-300 mb-2 text-left text-sm">
            Amount to Farm
          </label>
          <input
            type="number"
            step="0.0001"
            value={farmAmount}
            onChange={(e) => setFarmAmount(e.target.value)}
            required
            className="w-full px-4 py-3 border border-gray-600 rounded-xl bg-gray-800 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500"
          />
        </div>

        {/* Farm Button */}
        <button
          type="submit"
          className="w-full flex items-center justify-center gap-2 bg-yellow-500 text-black py-3 rounded-xl font-semibold hover:bg-yellow-600 active:scale-95 transition duration-200 shadow-lg"
        >
          Start Farming <ArrowRight size={18} />
        </button>
      </form>
    </div>
  );
};

export default YieldFarm;
