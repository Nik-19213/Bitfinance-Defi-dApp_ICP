import React, { useState } from "react";
import { ArrowRight } from "lucide-react";

const Lend = () => {
  const [amount, setAmount] = useState("");

  const handleLend = (e) => {
    e.preventDefault();
    console.log("BTC Amount:", amount);
    alert("Lend request submitted ✅");
  };

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-black via-gray-900 to-black px-4 relative">

      {/* Background Bitcoin Image */}
      <img
        src="https://images.app.goo.gl/ZU9dZh7RGddm35Gh8"
        alt="Bitcoin"
        className="absolute opacity-5 w-[500px] h-[500px] object-contain"
        style={{ top: "50%", left: "50%", transform: "translate(-50%, -50%)" }}
      />

      <h1 className="text-4xl font-extrabold mb-8 bg-gradient-to-r from-yellow-400 to-orange-500 text-transparent bg-clip-text">
        Lend Your ckBTC
      </h1>

      <form
        onSubmit={handleLend}
        className="bg-black bg-opacity-80 p-8 rounded-3xl shadow-2xl w-full max-w-md border border-gray-700 backdrop-blur-lg relative z-10"
      >
        <div className="mb-6">
          <label className="block text-gray-300 mb-2 text-left text-sm">
            Amount (ckBTC)
          </label>
          <input
            type="number"
            step="0.0001"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            required
            className="w-full px-4 py-3 border border-gray-600 rounded-xl bg-gray-800 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500"
          />
        </div>

        <button
          type="submit"
          className="w-full flex items-center justify-center gap-2 bg-yellow-500 text-black py-3 rounded-xl font-semibold hover:bg-yellow-600 active:scale-95 transition duration-200 shadow-lg"
        >
          Lend Now <ArrowRight size={18} />
        </button>
      </form>
    </div>
  );
};

export default Lend;
