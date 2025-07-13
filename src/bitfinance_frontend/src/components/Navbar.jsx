import { Link } from "react-router-dom";
import { useContext, useState, useEffect } from "react";
import { AuthContext } from "../context/AuthContext";
import { Bitcoin } from "lucide-react";

function Navbar() {
  const { principal, login, logout } = useContext(AuthContext);
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(principal);
    setCopied(true);
  };

  useEffect(() => {
    if (copied) {
      const timer = setTimeout(() => setCopied(false), 2000);
      return () => clearTimeout(timer);
    }
  }, [copied]);

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
        <Link to="/" className="hover:text-yellow-400 transition duration-200 hover:scale-105">
          Home
        </Link>
        <Link to="/stake" className="hover:text-yellow-400 transition duration-200 hover:scale-105">
          Stake
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
        <Link to="/dashboard" className="hover:text-yellow-400 transition duration-200 hover:scale-105">
          Dashboard
        </Link>
        <Link to="/register" className="hover:text-yellow-400 transition duration-200 hover:scale-105">
          Register
        </Link>
      </div>

      {/* Right Wallet & Theme */}
      <div className="flex items-center gap-4 relative">
        {principal ? (
          <>
            <button
              onClick={handleCopy}
              title="Click to copy principal"
              className="relative text-sm bg-gray-600 px-2 py-1 rounded hover:bg-gray-600 transition cursor-pointer"
            >
              {principal.slice(0, 8)}...{principal.slice(-5)}
            </button>

            {copied && (
              <div className="absolute top-full left-0 mt-1 text-xs text-white bg-gray-800 px-2 py-0.5 rounded shadow-lg">
                Copied!
              </div>
            )}

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
            Connect Identity
          </button>
        )}
      </div>
    </nav>
  );
}

export default Navbar;
