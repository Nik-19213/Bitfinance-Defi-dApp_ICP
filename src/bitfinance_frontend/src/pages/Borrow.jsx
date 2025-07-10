import React, { useState } from "react";
import { ArrowRight } from "lucide-react";

const Borrow = () => {
  const [amount, setAmount] = useState("");
  const [collateral, setCollateral] = useState("");

  const handleBorrow = (e) => {
    e.preventDefault();
    console.log("BTC to Borrow:", amount);
    console.log("Collateral (ICP):", collateral);
    alert("Borrow request submitted ✅");
  };

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-black via-gray-900 to-black px-4">

      {/* Heading */}
      <h1 className="text-4xl font-extrabold mb-8 bg-gradient-to-r from-green-400 to-blue-500 text-transparent bg-clip-text">
        Borrow Bitcoin
      </h1>

      {/* Form Card */}
      <form
        onSubmit={handleBorrow}
        className="bg-black bg-opacity-80 p-8 rounded-3xl shadow-2xl w-full max-w-md border border-gray-700 backdrop-blur-lg"
      >
        {/* Amount Input */}
        <div className="mb-6">
          <label className="block text-gray-300 mb-2 text-left text-sm">
            Amount to Borrow (BTC)
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

        {/* Collateral Input */}
        <div className="mb-6">
          <label className="block text-gray-300 mb-2 text-left text-sm">
            Collateral (ICP)
          </label>
          <input
            type="number"
            step="0.01"
            value={collateral}
            onChange={(e) => setCollateral(e.target.value)}
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
    </div>
  );
};

export default Borrow;
