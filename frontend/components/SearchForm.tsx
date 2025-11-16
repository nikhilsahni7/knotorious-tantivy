"use client";

import { Search } from "lucide-react";

interface SearchFields {
  master_id: string;
  name: string;
  fname: string;
  alt: string;
  email: string;
  address: string;
  mobile: string;
}

interface SearchFormProps {
  searchFields: SearchFields;
  searchMode: "AND" | "OR";
  loading: boolean;
  onFieldChange: (fields: SearchFields) => void;
  onModeChange: (mode: "AND" | "OR") => void;
  onSearch: () => void;
  onReset: () => void;
}

export default function SearchForm({
  searchFields,
  searchMode,
  loading,
  onFieldChange,
  onModeChange,
  onSearch,
  onReset,
}: SearchFormProps) {
  const updateField = (key: keyof SearchFields, value: string) => {
    onFieldChange({ ...searchFields, [key]: value });
  };

  return (
    <div className="bg-slate-800/50 rounded-lg border border-slate-700 p-6 mb-6">
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 mb-6">
        <div>
          <label htmlFor="master_id" className="block text-sm font-medium text-slate-300 mb-2">
            Master ID
          </label>
          <input
            id="master_id"
            type="text"
            value={searchFields.master_id}
            onChange={(e) => updateField("master_id", e.target.value)}
            className="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors"
            placeholder="Enter Master ID"
          />
        </div>
        <div>
          <label htmlFor="name" className="block text-sm font-medium text-slate-300 mb-2">
            Name
          </label>
          <input
            id="name"
            type="text"
            value={searchFields.name}
            onChange={(e) => updateField("name", e.target.value)}
            className="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors"
            placeholder="Enter Name"
          />
        </div>
        <div>
          <label htmlFor="fname" className="block text-sm font-medium text-slate-300 mb-2">
            Father&apos;s Name
          </label>
          <input
            id="fname"
            type="text"
            value={searchFields.fname}
            onChange={(e) => updateField("fname", e.target.value)}
            className="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors"
            placeholder="Enter Father&apos;s Name"
          />
        </div>
        <div>
          <label htmlFor="alt" className="block text-sm font-medium text-slate-300 mb-2">
            Alternate Number
          </label>
          <input
            id="alt"
            type="text"
            value={searchFields.alt}
            onChange={(e) => updateField("alt", e.target.value)}
            className="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors"
            placeholder="Enter Alternate Number"
          />
        </div>
        <div>
          <label htmlFor="email" className="block text-sm font-medium text-slate-300 mb-2">
            Email
          </label>
          <input
            id="email"
            type="email"
            value={searchFields.email}
            onChange={(e) => updateField("email", e.target.value)}
            className="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors"
            placeholder="Enter Email"
          />
        </div>
        <div>
          <label htmlFor="address" className="block text-sm font-medium text-slate-300 mb-2">
            Address
          </label>
          <input
            id="address"
            type="text"
            value={searchFields.address}
            onChange={(e) => updateField("address", e.target.value)}
            className="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors"
            placeholder="Enter Address"
          />
        </div>
        <div>
          <label htmlFor="mobile" className="block text-sm font-medium text-slate-300 mb-2">
            Mobile
          </label>
          <input
            id="mobile"
            type="text"
            value={searchFields.mobile}
            onChange={(e) => updateField("mobile", e.target.value)}
            className="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors"
            placeholder="Enter Mobile"
          />
        </div>
      </div>

      <div className="flex flex-col sm:flex-row items-start sm:items-center gap-4 mb-6">
        <div className="flex items-center gap-2">
          <span className="text-sm font-medium text-slate-300">Search Mode:</span>
          <div className="flex bg-slate-900/50 rounded-lg p-1 border border-slate-600">
            <button
              type="button"
              onClick={() => onModeChange("AND")}
              className={`px-4 py-1 rounded-md text-sm font-medium transition-colors ${
                searchMode === "AND"
                  ? "bg-blue-600 text-white"
                  : "text-slate-400 hover:text-slate-200"
              }`}
            >
              AND
            </button>
            <button
              type="button"
              onClick={() => onModeChange("OR")}
              className={`px-4 py-1 rounded-md text-sm font-medium transition-colors ${
                searchMode === "OR"
                  ? "bg-blue-600 text-white"
                  : "text-slate-400 hover:text-slate-200"
              }`}
            >
              OR
            </button>
          </div>
        </div>
      </div>

      <div className="flex flex-col sm:flex-row gap-4">
        <button
          type="button"
          onClick={onSearch}
          disabled={loading}
          className="flex items-center justify-center gap-2 px-8 py-3 bg-blue-600 text-white font-semibold rounded-lg hover:bg-blue-700 transition-colors shadow-lg hover:shadow-blue-500/50 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <Search className="w-5 h-5" />
          {loading ? "Searching..." : "Search"}
        </button>
        <button
          type="button"
          onClick={onReset}
          className="px-8 py-3 bg-slate-700/50 text-slate-200 font-semibold rounded-lg hover:bg-slate-700 transition-colors border border-slate-600"
        >
          Reset
        </button>
      </div>
    </div>
  );
}
