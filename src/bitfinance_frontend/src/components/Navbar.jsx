import { Link } from "react-router-dom";
import { useContext } from "react";
import { AuthContext } from "../context/AuthContext";

import { Bitcoin } from "lucide-react";

function Navbar() {
  const { principal, login, logout } = useContext(AuthContext);

  return (
    <nav className="bg-gray-900 text-white p-5 flex justify-between items-center shadow-lg border-b border-yellow-500">
      {/* Left Logo & Title */}
      <div className="flex items-center gap-3">
        <Bitcoin className="w-7 h-7 text-yellow-400" />
        <h2 className="text-2xl font-extrabold flex items-center gap-1">
          <span className="text-white">Bit</span>
          <span className="bg-yellow-400 text-black px-2 py-0.5 rounded-lg">Finance</span>
        </h2>
      </div>

      {/* Menu Links Stylish */}
      <div className="flex space-x-5 text-sm font-semibold uppercase tracking-wide">
         <Link to="/register" className="hover:text-yellow-400 transition duration-200 hover:scale-105">
          Register
        </Link>
        <Link to="/" className="hover:text-yellow-400 transition duration-200 hover:scale-105">
          Home
        </Link>
        <Link to="/lend" className="hover:text-yellow-400 transition duration-200 hover:scale-105">
          Lend
        </Link>
        <Link to="/borrow" className="hover:text-yellow-400 transition duration-200 hover:scale-105">
          Borrow
        </Link>
        <Link to="/yield" className="hover:text-yellow-400 transition duration-200 hover:scale-105">
          Yield
        </Link>
        <Link to="/stake" className="hover:text-yellow-400 transition duration-200 hover:scale-105">
          Stake
        </Link>
        <Link to="/transactions" className="hover:text-yellow-400 transition duration-200 hover:scale-105">
          Transactions
        </Link>
        <Link to="/dashboard" className="hover:text-yellow-400 transition duration-200 hover:scale-105">
          Dashboard
        </Link>
       
      </div>

      {/* Right Wallet & Theme */}
      <div className="flex items-center gap-4">
        
        {principal ? (
          <>
            <span className="text-sm bg-gray-700 px-2 py-1 rounded">
              {principal.slice(0, 5)}...{principal.slice(-5)}
            </span>
            <button
              onClick={logout}
              className="bg-red-500 text-white px-4 py-1 rounded hover:bg-red-600 transition"
            >
              Logout
            </button>
          </>
        ) : (
          <button
            onClick={login}
            className="bg-yellow-400 text-black px-4 py-1 rounded hover:bg-yellow-300 transition"
          >
            Connect Wallet
          </button>
        )}
      </div>
    </nav>
  );
}

export default Navbar;
