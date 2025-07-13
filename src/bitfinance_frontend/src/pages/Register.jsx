import React, { useContext } from "react";
import { ArrowRight } from "lucide-react";
import { bitfinance_backend } from "../../../declarations/bitfinance_backend";
import { AuthContext } from "../context/AuthContext";

const Register = () => {
  const { principal } = useContext(AuthContext);

  const handleRegister = async (e) => {
    e.preventDefault();
    if (!principal) {
      alert("Please connect your wallet first.");
      return;
    }
    try {
      const result = await bitfinance_backend.register_user();
      alert(result);
      // Optionally, force a reload or redirect after registration
      // window.location.reload();
      // or use a router to navigate
    } catch (err) {
      console.error("Registration error:", err);
      alert("Registration failed: " + err);
    }
  };

  // Add a debug log for principal
  console.log("Register page principal:", principal);

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
        {/* Register Button */}
        <button
          type="submit"
          className="w-full flex items-center justify-center gap-2 bg-yellow-500 text-black py-3 rounded-xl font-semibold hover:bg-yellow-600 active:scale-95 transition duration-200 shadow-lg"
          disabled={!principal}
        >
          Register as User <ArrowRight size={18} />
        </button>
        {!principal && (
          <div className="text-red-400 text-center mt-4">
            Please connect with your Identity to register.
          </div>
        )}
      </form>
    </div>
  );
};

export default Register;
