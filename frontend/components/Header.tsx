"use client";

import {
  Copy,
  Download,
  History,
  Lock,
  LogOut,
  Settings,
  User,
} from "lucide-react";

export default function Header() {
  return (
    <header className="bg-purple-900/50 backdrop-blur-sm border-b border-pink-500/20 px-4 sm:px-6 py-4">
      <div className="max-w-7xl mx-auto flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-xl sm:text-2xl font-bold text-white">
            Knotorious Search
          </h1>
          <p className="text-xs sm:text-sm text-purple-200">
            Administrator • admin@notorious.com
          </p>
        </div>
        <div className="flex items-center gap-2 sm:gap-4 flex-wrap">
          <div className="bg-green-500/20 px-3 sm:px-4 py-2 rounded-lg border border-green-400/30">
            <span className="text-green-300 text-xs sm:text-sm">
              Daily Limit: 0/999999 (999999 left)
            </span>
          </div>
          <button
            type="button"
            className="p-2 hover:bg-purple-700/50 rounded-lg transition-colors"
            aria-label="Profile"
          >
            <User className="w-4 h-4 sm:w-5 sm:h-5 text-pink-300" />
          </button>
          <button
            type="button"
            className="p-2 hover:bg-purple-700/50 rounded-lg transition-colors"
            aria-label="History"
          >
            <History className="w-4 h-4 sm:w-5 sm:h-5 text-yellow-300" />
          </button>
          <button
            type="button"
            className="p-2 hover:bg-purple-700/50 rounded-lg transition-colors"
            aria-label="Password"
          >
            <Lock className="w-4 h-4 sm:w-5 sm:h-5 text-blue-300" />
          </button>
          <button
            type="button"
            className="p-2 hover:bg-purple-700/50 rounded-lg transition-colors"
            aria-label="Export"
          >
            <Download className="w-4 h-4 sm:w-5 sm:h-5 text-green-300" />
          </button>
          <button
            type="button"
            className="p-2 hover:bg-purple-700/50 rounded-lg transition-colors"
            aria-label="Admin"
          >
            <Settings className="w-4 h-4 sm:w-5 sm:h-5 text-red-300" />
          </button>
          <button
            type="button"
            className="p-2 hover:bg-purple-700/50 rounded-lg transition-colors"
            aria-label="Logout"
          >
            <LogOut className="w-4 h-4 sm:w-5 sm:h-5 text-red-400" />
          </button>
        </div>
      </div>
    </header>
  );
}
