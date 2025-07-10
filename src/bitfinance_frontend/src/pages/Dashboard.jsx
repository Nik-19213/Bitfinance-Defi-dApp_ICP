import React from "react";
import { Wallet, TrendingUp, TrendingDown, Clock } from "lucide-react";

const Dashboard = () => {
  return (
    <div className="min-h-screen bg-gradient-to-br from-black via-gray-900 to-black px-6 py-10">
      <h1 className="text-4xl font-extrabold text-yellow-400 mb-8 text-center tracking-wide drop-shadow-lg">
        📊 Dashboard
      </h1>

      {/* Cards Section */}
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-6 mb-12">
        <div className="bg-gray-800 p-6 rounded-3xl shadow-2xl flex items-center space-x-4 hover:scale-105 transition">
          <Wallet className="text-blue-400 w-10 h-10" />
          <div>
            <p className="text-sm text-gray-400">Total Balance</p>
            <h2 className="text-2xl font-bold text-white">0.245 BTC</h2>
          </div>
        </div>

        <div className="bg-gray-800 p-6 rounded-3xl shadow-2xl flex items-center space-x-4 hover:scale-105 transition">
          <TrendingUp className="text-green-400 w-10 h-10" />
          <div>
            <p className="text-sm text-gray-400">BTC Lent</p>
            <h2 className="text-2xl font-bold text-white">0.1 BTC</h2>
          </div>
        </div>

        <div className="bg-gray-800 p-6 rounded-3xl shadow-2xl flex items-center space-x-4 hover:scale-105 transition">
          <TrendingDown className="text-red-400 w-10 h-10" />
          <div>
            <p className="text-sm text-gray-400">BTC Borrowed</p>
            <h2 className="text-2xl font-bold text-white">0.05 BTC</h2>
          </div>
        </div>
      </div>

      {/* Transaction History */}
      <div className="bg-gray-800 p-6 rounded-3xl shadow-2xl">
        <div className="flex items-center mb-5">
          <Clock className="text-yellow-400 w-6 h-6 mr-2" />
          <h3 className="text-2xl font-semibold text-white">Recent Transactions</h3>
        </div>

        <ul className="space-y-3 text-base text-gray-300">
          <li>🔁 Lent <span className="text-yellow-400 font-semibold">0.05 BTC</span> to Vault A <span className="text-gray-400">(2 days ago)</span></li>
          <li>📥 Borrowed <span className="text-yellow-400 font-semibold">0.01 BTC</span> from Pool X <span className="text-gray-400">(3 days ago)</span></li>
          <li>📤 Lent <span className="text-yellow-400 font-semibold">0.05 BTC</span> to Vault B <span className="text-gray-400">(5 days ago)</span></li>
        </ul>
      </div>
    </div>
  );
};

export default Dashboard;
