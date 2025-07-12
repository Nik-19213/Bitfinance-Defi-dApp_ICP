import React, { useState, useContext, useEffect } from "react";
import { ArrowRight } from "lucide-react";
import { bitfinance_backend } from "../../../declarations/bitfinance_backend";
import { AuthContext } from "../context/AuthContext";

const Borrow = () => {
  const [amount, setAmount] = useState("");
  const [repayAmount, setRepayAmount] = useState("");
  const { principal } = useContext(AuthContext);

  // Display borrowed amount and timestamp
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

  const handleBorrow = async (e) => {
    e.preventDefault();
    try {
      const result = await bitfinance_backend.borrow_ckbtc(Number(amount));
      alert(result);
    } catch (err) {
      alert("Borrow failed: " + err);
    }
  };

  const handleRepay = async (e) => {
    e.preventDefault();
    try {
      const result = await bitfinance_backend.repay_loan_ckbtc(Number(repayAmount));
      alert(result);
    } catch (err) {
      alert("Repay failed: " + err);
    }
  };

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-black via-gray-900 to-black px-4 relative">
      <div className="w-full flex flex-col md:flex-row md:justify-between md:items-start">
        <h1 className="text-4xl font-extrabold mb-8 bg-gradient-to-r from-green-400 to-blue-500 text-transparent bg-clip-text flex items-center gap-2">
          Borrow ckBTC
        </h1>
        {/* Borrowed Amount Display - right upper corner */}
        <div className="mb-6 md:mb-0 md:ml-auto bg-gray-900 rounded-xl px-6 py-4 border border-yellow-500 text-center min-w-[260px]">
          <div className="text-lg text-yellow-400 font-bold">
            Borrowed: {userData ? (userData.loans / 1e8).toFixed(8) : "0.00000000"} ckBTC
          </div>
          <div className="text-gray-400 text-sm">
            {userData && userData.loan_timestamp
              ? "Since: " +
              new Date(Number(userData.loan_timestamp) * 1000).toLocaleString()
              : "No active loan"}
          </div>
        </div>
      </div>

      {/* Form Card */}
      <form
        onSubmit={handleBorrow}
        className="bg-black bg-opacity-80 p-8 rounded-3xl shadow-2xl w-full max-w-md border border-gray-700 backdrop-blur-lg"
      >
        {/* Amount Input */}
        <div className="mb-6">
          <label className="block text-gray-300 mb-2 text-left text-sm">
            Amount to Borrow (ckBTC)
          </label>
          <input
            type="number"
            step="0.0001"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            required
            className="w-full px-4 py-3 border border-gray-600 rounded-xl bg-gray-800 text-white focus:outline-none focus:ring-2 focus:ring-green-500"
          />
        </div>

        {/* Submit Button */}
        <button
          type="submit"
          className="w-full flex items-center justify-center gap-2 bg-green-500 text-black py-3 rounded-xl font-semibold hover:bg-green-600 active:scale-95 transition duration-200 shadow-lg"
        >
          Borrow Now <ArrowRight size={18} />
        </button>
      </form>

      {/* Repay Section */}
      <form
        onSubmit={handleRepay}
        className="bg-black bg-opacity-70 p-6 rounded-2xl w-full max-w-md border border-gray-700 mt-8"
      >
        <h2 className="text-xl font-bold text-white mb-4">Repay Loan</h2>
        <div className="mb-4">
          <label className="block text-gray-300 mb-2 text-left text-sm">
            Amount to Repay (ckBTC)
          </label>
          <input
            type="number"
            step="0.0001"
            value={repayAmount}
            onChange={(e) => setRepayAmount(e.target.value)}
            required
            className="w-full px-4 py-3 border border-gray-600 rounded-xl bg-gray-800 text-white focus:outline-none focus:ring-2 focus:ring-green-500"
          />
        </div>
        <button
          type="submit"
          className="w-full bg-green-500 text-black py-2 rounded-xl font-semibold hover:bg-green-600 transition duration-200 shadow-lg"
        >
          Repay
        </button>
      </form>
    </div>
  );
};

export default Borrow;
