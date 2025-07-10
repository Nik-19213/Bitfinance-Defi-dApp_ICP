import React from "react";

function TransactionHistory() {
  // Filhal koi transaction nahi
  const transactions = [];

  return (
    <div className="min-h-screen p-6 bg-gray-900 text-white">
      <h2 className="text-3xl font-bold mb-6 text-yellow-400">Transaction History</h2>

      <div className="overflow-x-auto">
        {transactions.length === 0 ? (
          <div className="text-center py-20 text-gray-400 text-lg border border-gray-700 rounded-xl bg-gray-800">
            📭 No transactions yet.
          </div>
        ) : (
          <table className="min-w-full bg-gray-800 rounded-lg shadow-lg">
            <thead>
              <tr className="bg-gray-700 text-left">
                <th className="px-4 py-3">Date</th>
                <th className="px-4 py-3">Action</th>
                <th className="px-4 py-3">Amount</th>
                <th className="px-4 py-3">Status</th>
              </tr>
            </thead>
            <tbody>
              {transactions.map((tx, index) => (
                <tr key={index} className="border-t border-gray-600 hover:bg-gray-700">
                  <td className="px-4 py-3">{tx.date}</td>
                  <td className="px-4 py-3">{tx.action}</td>
                  <td className="px-4 py-3">{tx.amount}</td>
                  <td
                    className={`px-4 py-3 font-semibold ${
                      tx.status === "Success"
                        ? "text-green-400"
                        : tx.status === "Pending"
                        ? "text-yellow-400"
                        : "text-red-400"
                    }`}
                  >
                    {tx.status}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
}

export default TransactionHistory;
