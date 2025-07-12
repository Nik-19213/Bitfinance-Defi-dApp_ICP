import React, { useEffect, useState, useContext } from "react";
import { Wallet, TrendingUp, TrendingDown, Clock, RefreshCw, ArrowDownCircle, ArrowUpCircle, Gift, AlertTriangle } from "lucide-react";
import { bitfinance_backend } from "../../../declarations/bitfinance_backend";
import { AuthContext } from "../context/AuthContext";

const Dashboard = () => {
  const [userData, setUserData] = useState(null);
  const [depositAmount, setDepositAmount] = useState("");
  const [withdrawAmount, setWithdrawAmount] = useState("");
  const [loading, setLoading] = useState(false);
  const { principal } = useContext(AuthContext);

  // Rewards
  const [stakingRewards, setStakingRewards] = useState(0);
  const [lendingRewards, setLendingRewards] = useState(0);
  const [farmingRewards, setFarmingRewards] = useState(0);

  const fetchData = async () => {
    setLoading(true);
    try {
      const data = await bitfinance_backend.get_my_data();
      if (data && data.length > 0) setUserData(data[0]);
      // Rewards
      const sr = await bitfinance_backend.get_pending_staking_rewards();
      setStakingRewards(Number(sr));
      const lr = await bitfinance_backend.get_pending_lending_rewards();
      setLendingRewards(Number(lr));
      const fr = await bitfinance_backend.get_pending_yield_farming_rewards();
      setFarmingRewards(Number(fr));
    } catch (err) {
      // handle error
    }
    setLoading(false);
  };

  useEffect(() => {
    fetchData();
    // eslint-disable-next-line
  }, [principal]);

  const handleDeposit = async (e) => {
    e.preventDefault();
    setLoading(true);
    try {
      const result = await bitfinance_backend.deposit_ckbtc(Number(depositAmount));
      alert(result);
      setDepositAmount("");
      fetchData();
    } catch (err) {
      alert("Deposit failed: " + err);
    }
    setLoading(false);
  };

  const handleWithdraw = async (e) => {
    e.preventDefault();
    setLoading(true);
    try {
      const result = await bitfinance_backend.withdraw_ckbtc(Number(withdrawAmount));
      alert(result);
      setWithdrawAmount("");
      fetchData();
    } catch (err) {
      alert("Withdraw failed: " + err);
    }
    setLoading(false);
  };

  const handleClaimStaking = async () => {
    setLoading(true);
    try {
      const result = await bitfinance_backend.claim_staking_rewards();
      alert(result);
      fetchData();
    } catch (err) {
      alert("Claim staking rewards failed: " + err);
    }
    setLoading(false);
  };

  const handleClaimLending = async () => {
    setLoading(true);
    try {
      const result = await bitfinance_backend.claim_lending_rewards();
      alert(result);
      fetchData();
    } catch (err) {
      alert("Claim lending rewards failed: " + err);
    }
    setLoading(false);
  };

  const handleClaimFarming = async () => {
    setLoading(true);
    try {
      const result = await bitfinance_backend.claim_yield_farming_rewards();
      alert(result);
      fetchData();
    } catch (err) {
      alert("Claim farming rewards failed: " + err);
    }
    setLoading(false);
  };

  const handleEmergencyWithdraw = async () => {
    if (!window.confirm("Are you sure? This will withdraw all your assets!")) return;
    setLoading(true);
    try {
      const result = await bitfinance_backend.emergency_withdraw_all();
      alert(result);
      fetchData();
    } catch (err) {
      alert("Emergency withdraw failed: " + err);
    }
    setLoading(false);
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-black via-gray-900 to-black px-6 py-10">

      <div className="flex flex-col md:flex-row md:items-center md:justify-between mb-8">
        <div className="flex items-center gap-2">
          <Wallet className="w-8 h-8 text-yellow-400" />
          <h1 className="text-4xl font-extrabold text-yellow-400 tracking-wide drop-shadow-lg">
            Dashboard
          </h1>
          <button
            onClick={fetchData}
            disabled={loading}
            className="ml-4 bg-gray-800 hover:bg-gray-700 text-yellow-400 rounded-full p-2 transition"
            title="Refresh"
          >
            <RefreshCw className={loading ? "animate-spin" : ""} />
          </button>
        </div>
        {/* Total Balance Section - right side */}
        <div className="mt-6 md:mt-0 flex flex-col items-end">
          <div className="bg-gray-900 rounded-3xl shadow-2xl px-8 py-4 flex flex-col items-end border-2 border-yellow-400">
            <div className="flex items-center gap-2 mb-1">
              <Wallet className="w-7 h-7 text-yellow-400" />
              <span className="text-lg text-gray-300">Your Total ckBTC Balance</span>
            </div>
            <span className="text-3xl font-extrabold text-yellow-400">
              {userData ? (userData.ckbtc_balance / 1e8).toFixed(8) : "0.00000000"} ckBTC
            </span>
          </div>
        </div>
      </div>

      {/* Deposit & Withdraw */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-8 mb-12">
        <form
          onSubmit={handleDeposit}
          className="bg-gray-800 p-6 rounded-2xl shadow-lg flex flex-col gap-4"
        >
          <h2 className="text-xl font-bold text-yellow-400 flex items-center gap-2">
            <ArrowDownCircle className="w-6 h-6" /> Deposit ckBTC
          </h2>
          <input
            type="number"
            step="0.0001"
            min="0"
            value={depositAmount}
            onChange={(e) => setDepositAmount(e.target.value)}
            placeholder="Amount"
            required
            className="w-full px-4 py-3 border border-gray-600 rounded-xl bg-gray-900 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500"
          />
          <button
            type="submit"
            disabled={loading}
            className="bg-yellow-500 text-black py-2 rounded-xl font-semibold hover:bg-yellow-600 transition duration-200 shadow-lg"
          >
            Deposit
          </button>
        </form>
        <form
          onSubmit={handleWithdraw}
          className="bg-gray-800 p-6 rounded-2xl shadow-lg flex flex-col gap-4"
        >
          <h2 className="text-xl font-bold text-yellow-400 flex items-center gap-2">
            <ArrowUpCircle className="w-6 h-6" /> Withdraw ckBTC
          </h2>
          <input
            type="number"
            step="0.0001"
            min="0"
            value={withdrawAmount}
            onChange={(e) => setWithdrawAmount(e.target.value)}
            placeholder="Amount"
            required
            className="w-full px-4 py-3 border border-gray-600 rounded-xl bg-gray-900 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500"
          />
          <button
            type="submit"
            disabled={loading}
            className="bg-yellow-500 text-black py-2 rounded-xl font-semibold hover:bg-yellow-600 transition duration-200 shadow-lg"
          >
            Withdraw
          </button>
        </form>
      </div>

      {/* Rewards Section */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-8 mb-12">
        <div className="bg-gray-800 p-6 rounded-2xl shadow-lg flex flex-col items-center">
          <Gift className="w-8 h-8 text-yellow-400 mb-2" />
          <h3 className="text-lg font-bold text-white mb-2">Staking Rewards</h3>
          <p className="text-yellow-400 text-xl mb-2">{(stakingRewards / 1e8).toFixed(8)} BTC</p>
          <button
            onClick={handleClaimStaking}
            disabled={loading}
            className="bg-green-600 text-white py-2 px-4 rounded-xl hover:bg-green-700 transition duration-200"
          >
            Claim
          </button>
        </div>
        <div className="bg-gray-800 p-6 rounded-2xl shadow-lg flex flex-col items-center">
          <Gift className="w-8 h-8 text-yellow-400 mb-2" />
          <h3 className="text-lg font-bold text-white mb-2">Lending Rewards</h3>
          <p className="text-yellow-400 text-xl mb-2">{(lendingRewards / 1e8).toFixed(8)} BTC</p>
          <button
            onClick={handleClaimLending}
            disabled={loading}
            className="bg-green-600 text-white py-2 px-4 rounded-xl hover:bg-green-700 transition duration-200"
          >
            Claim
          </button>
        </div>
        <div className="bg-gray-800 p-6 rounded-2xl shadow-lg flex flex-col items-center">
          <Gift className="w-8 h-8 text-yellow-400 mb-2" />
          <h3 className="text-lg font-bold text-white mb-2">Farming Rewards</h3>
          <p className="text-yellow-400 text-xl mb-2">{(farmingRewards / 1e8).toFixed(8)} BTC</p>
          <button
            onClick={handleClaimFarming}
            disabled={loading}
            className="bg-green-600 text-white py-2 px-4 rounded-xl hover:bg-green-700 transition duration-200"
          >
            Claim
          </button>
        </div>
      </div>

      {/* Emergency Withdraw */}
      <div className="flex justify-center">
        <button
          onClick={handleEmergencyWithdraw}
          disabled={loading}
          className="flex items-center gap-2 bg-red-600 text-white px-6 py-3 rounded-xl font-semibold hover:bg-red-700 transition duration-200 shadow-lg"
        >
          <AlertTriangle className="w-5 h-5" /> Emergency Withdraw All
        </button>
      </div>
    </div>
  );
};

export default Dashboard;
