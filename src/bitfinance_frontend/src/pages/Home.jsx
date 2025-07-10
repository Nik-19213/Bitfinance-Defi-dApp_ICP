import React from "react";
import { Link } from "react-router-dom";
import { Bitcoin, ShieldCheck, ArrowRight } from "lucide-react";

const Home = () => {
  return (
    <div className="relative min-h-screen flex flex-col px-4 text-center overflow-hidden bg-black">

      {/* Background */}
      <div className="absolute inset-0 bg-gradient-to-br from-black via-gray-900 to-black"></div>

      {/* Main Content */}
      <div className="relative z-10 flex flex-col items-center justify-center flex-1 text-white">
        <div className="bg-black bg-opacity-80 p-10 rounded-3xl shadow-2xl max-w-xl w-full border border-gray-800 backdrop-blur-lg text-center">

          <div className="flex items-center justify-center mb-5">
            <Bitcoin className="w-16 h-16 text-yellow-400 animate-bounce" />
          </div>

          {/* Heading */}
          <h1 className="text-5xl font-extrabold mb-4 flex items-center justify-center gap-2 flex-wrap">
            <span className="text-white">Bit</span>
            <span className="bg-yellow-400 text-black px-4 py-1 rounded-xl">Finance</span>
          </h1>

          <p className="text-gray-300 text-lg mb-8 leading-relaxed">
            Decentralized finance redefined — lend, borrow, stake & farm ckBTC securely using the power of Internet Computer Protocol.
          </p>

          {/* 4 Buttons */}
          <div className="flex flex-wrap items-center justify-center gap-4">

            <Link to="/lend">
              <button className="flex items-center gap-2 bg-blue-600 text-white px-6 py-3 rounded-xl hover:bg-blue-700 active:scale-95 transition shadow-lg">
                Lend ckBTC <ArrowRight size={18} />
              </button>
            </Link>

            <Link to="/borrow">
              <button className="flex items-center gap-2 bg-green-600 text-white px-6 py-3 rounded-xl hover:bg-green-700 active:scale-95 transition shadow-lg">
                Borrow ckBTC <ArrowRight size={18} />
              </button>
            </Link>

            <Link to="/stake">
              <button className="flex items-center gap-2 bg-yellow-400 text-black px-6 py-3 rounded-xl hover:bg-yellow-300 active:scale-95 transition shadow-lg">
                Stake ckBTC <ArrowRight size={18} />
              </button>
            </Link>

            <Link to="/yield">
              <button className="flex items-center gap-2 bg-pink-500 text-white px-6 py-3 rounded-xl hover:bg-pink-600 active:scale-95 transition shadow-lg">
                Yield Farm <ArrowRight size={18} />
              </button>
            </Link>

          </div>

          <div className="flex items-center justify-center mt-8 text-gray-400 text-sm">
            <ShieldCheck className="w-4 h-4 mr-2 text-blue-400" />
            <span>Secured with ICP Smart Contracts</span>
          </div>

        </div>
      </div>

    </div>
  );
};

export default Home;
