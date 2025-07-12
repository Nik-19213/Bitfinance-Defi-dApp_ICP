import React, { useState, useContext, useEffect } from "react";
import { ArrowRight, Leaf } from "lucide-react";
import { bitfinance_backend } from "../../../declarations/bitfinance_backend";
import { AuthContext } from "../context/AuthContext";

const YieldFarm = () => {
  const [farmAmount, setFarmAmount] = useState("");
  const [unfarmAmount, setUnfarmAmount] = useState("");
  const { principal } = useContext(AuthContext);

  // Display farmed amount and timestamp
  const [userData, setUserData] = useState(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const data = await bitfinance_backend.get_my_data();
        if (data && data.length > 0) setUserData(data[0]);
      } catch (err) { }
    };
    fetchData();
  }, [principal]);

  const handleFarm = async (e) => {
    e.preventDefault();
    try {
      const result = await bitfinance_backend.yield_farm_ckbtc(Number(farmAmount));
      alert(result);
    } catch (err) {
      alert("Yield farming failed: " + err);
    }
  };

  const handleUnfarm = async (e) => {
    e.preventDefault();
    try {
      const result = await bitfinance_backend.unfarm_ckbtc(Number(unfarmAmount));
      alert(result);
    } catch (err) {
      alert("Unfarm failed: " + err);
    }
  };

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-black via-gray-900 to-black px-4">
      <div className="w-full flex flex-col md:flex-row md:justify-between md:items-start">
        <h1 className="text-4xl font-extrabold mb-8 bg-gradient-to-r from-yellow-400 to-pink-500 text-transparent bg-clip-text flex items-center gap-2">
          Yield Farming
        </h1>

        {/* Farmed Amount Display - right upper corner */}
        <div className="mb-6 md:mb-0 md:ml-auto bg-gray-900 rounded-xl px-6 py-4 border border-yellow-500 text-center min-w-[260px]">
          <div className="text-lg text-yellow-400 font-bold">
            Farmed: {userData ? (userData.farmed / 1e8).toFixed(8) : "0.00000000"} ckBTC
          </div>
          <div className="text-gray-400 text-sm">
            {userData && userData.farm_timestamp
              ? "Since: " +
              new Date(Number(userData.farm_timestamp) * 1000).toLocaleString()
              : "No active farming"}
          </div>
        </div>
      </div>

      <form
        onSubmit={handleFarm}
        className="bg-black bg-opacity-80 p-8 rounded-3xl shadow-2xl w-full max-w-md border border-gray-700 backdrop-blur-lg"
      >
        {/* Farming Amount */}
        <div className="mb-6">
          <label className="block text-gray-300 mb-2 text-left text-sm">
            Amount to Farm (ckBTC)
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

      {/* Unfarm Section */}
      <form
        onSubmit={handleUnfarm}
        className="bg-black bg-opacity-70 p-6 rounded-2xl w-full max-w-md border border-gray-700 mt-8"
      >
        <h2 className="text-xl font-bold text-white mb-4">Unfarm</h2>
        <div className="mb-4">
          <label className="block text-gray-300 mb-2 text-left text-sm">
            Amount to Unfarm (ckBTC)
          </label>
          <input
            type="number"
            step="0.0001"
            value={unfarmAmount}
            onChange={(e) => setUnfarmAmount(e.target.value)}
            required
            className="w-full px-4 py-3 border border-gray-600 rounded-xl bg-gray-800 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500"
          />
        </div>
        <button
          type="submit"
          className="w-full bg-yellow-500 text-black py-2 rounded-xl font-semibold hover:bg-yellow-600 transition duration-200 shadow-lg"
        >
          Unfarm
        </button>
      </form>
    </div>
  );
};

export default YieldFarm;
