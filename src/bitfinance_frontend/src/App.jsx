import {
    BrowserRouter as Router,
    Routes,
    Route,
} from "react-router-dom";

import Navbar from "./components/Navbar"; // ✅ spelling fixed
import Home from "./pages/Home";
import Lend from "./pages/Lend";
import Borrow from "./pages/Borrow";
import Dashboard from "./pages/Dashboard";
import Yield from "./pages/Yield";
import TransactionHistory from "./pages/TransactionHistory";
import Register from "./pages/Register";
import Stake from "./pages/Stake";

function App() {
    return (
        <Router>
            <div className="min-h-screen bg-white dark:bg-gray-900 text-black dark:text-white transition-all">
                <Navbar />
                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/lend" element={<Lend />} />
                    <Route path="/borrow" element={<Borrow />} />
                    <Route path="/dashboard" element={<Dashboard />} />
                    <Route path="/yield" element={<Yield />} />
                    <Route path="/transactions" element={<TransactionHistory />} />
                    <Route path="/register" element={<Register />} />
                    <Route path="/stake" element={<Stake />} />

                </Routes>
            </div>
        </Router>
    );
}

export default App;


