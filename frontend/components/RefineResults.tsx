"use client";

interface RefineResultsProps {
  resultCount: number;
  filterText: string;
  onFilterChange: (text: string) => void;
}

export default function RefineResults({
  resultCount,
  filterText,
  onFilterChange,
}: RefineResultsProps) {
  return (
    <div className="bg-slate-800/50 rounded-lg border border-slate-700 p-6 mb-6">
      <div className="mb-4">
        <h2 className="text-lg font-semibold text-slate-100 mb-1">
          Refine Results (doesn&apos;t use search credits)
        </h2>
        <p className="text-sm text-slate-400">
          Found {resultCount} results. Add filters to narrow down your search.
        </p>
      </div>
      <input
        type="text"
        value={filterText}
        onChange={(e) => onFilterChange(e.target.value)}
        className="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors"
        placeholder="Filter results..."
      />
    </div>
  );
}
