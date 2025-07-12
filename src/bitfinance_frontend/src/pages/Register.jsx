import React, { useState, useContext } from "react";
import { UserPlus, ArrowRight } from "lucide-react";
import { bitfinance_backend } from "../../../declarations/bitfinance_backend";
import { AuthContext } from "../context/AuthContext";

const Register = () => {
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [confirm, setConfirm] = useState("");
  const { principal } = useContext(AuthContext);

  const handleRegister = async (e) => {
    e.preventDefault();

    if (password !== confirm) {
      alert("Passwords do not match ❌");
      return;
    }

    // Call backend register_user
    try {
      const result = await bitfinance_backend.register_user();
      alert(result);
    } catch (err) {
      alert("Registration failed: " + err);
    }
  };

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-black via-gray-900 to-black px-4 relative">
      {/* Page Heading */}
      <h1 className="text-4xl font-extrabold mb-8 bg-gradient-to-r from-yellow-400 to-orange-500 text-transparent bg-clip-text flex items-center gap-2">
        Register
      </h1>

      {/* Registration Form */}
      <form
        onSubmit={handleRegister}
        className="bg-black bg-opacity-80 p-8 rounded-3xl shadow-2xl w-full max-w-md border border-gray-700 backdrop-blur-lg"
      >
        {/* Name */}
        <div className="mb-5">
          <label className="block text-gray-300 mb-2 text-left text-sm">
            Full Name
          </label>
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            required
            className="w-full px-4 py-3 border border-gray-600 rounded-xl bg-gray-800 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500"
          />
        </div>

        {/* Email */}
        <div className="mb-5">
          <label className="block text-gray-300 mb-2 text-left text-sm">
            Email Address
          </label>
          <input
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            required
            className="w-full px-4 py-3 border border-gray-600 rounded-xl bg-gray-800 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500"
          />
        </div>

        {/* Password */}
        <div className="mb-5">
          <label className="block text-gray-300 mb-2 text-left text-sm">
            Password
          </label>
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
            className="w-full px-4 py-3 border border-gray-600 rounded-xl bg-gray-800 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500"
          />
        </div>

        {/* Confirm Password */}
        <div className="mb-8">
          <label className="block text-gray-300 mb-2 text-left text-sm">
            Confirm Password
          </label>
          <input
            type="password"
            value={confirm}
            onChange={(e) => setConfirm(e.target.value)}
            required
            className="w-full px-4 py-3 border border-gray-600 rounded-xl bg-gray-800 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500"
          />
        </div>

        {/* Register Button */}
        <button
          type="submit"
          className="w-full flex items-center justify-center gap-2 bg-yellow-500 text-black py-3 rounded-xl font-semibold hover:bg-yellow-600 active:scale-95 transition duration-200 shadow-lg"
        >
          Register <ArrowRight size={18} />
        </button>
      </form>
    </div>
  );
};

export default Register;
