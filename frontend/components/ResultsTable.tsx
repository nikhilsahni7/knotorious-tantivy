"use client";

import { SearchResult } from "@/lib/api";
import { formatNumber } from "@/lib/utils";
import { ChevronLeft, ChevronRight, Copy } from "lucide-react";
import { useCallback, useMemo, useState } from "react";

interface ResultsTableProps {
  results: SearchResult[];
  totalMatches: number;
  resultsReturned: number;
  searchTime: number;
  queryParseTime: number;
  searchExecutionTime: number;
  documentRetrievalTime: number;
  filterText: string;
  onFilterChange: (text: string) => void;
}

const RESULTS_PER_PAGE = 100;

export default function ResultsTable({
  results,
  totalMatches,
  resultsReturned,
  searchTime,
  queryParseTime,
  searchExecutionTime,
  documentRetrievalTime,
  filterText,
  onFilterChange,
}: ResultsTableProps) {
  const [copiedId, setCopiedId] = useState<string | null>(null);
  const [currentPage, setCurrentPage] = useState(1);

  const copyToClipboard = useCallback(async (text: string, id: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopiedId(id);
      setTimeout(() => setCopiedId(null), 2000);
    } catch (err) {
      console.error("Failed to copy:", err);
    }
  }, []);

  const copyAllResults = useCallback(async () => {
    try {
      const text = results
        .map(
          (r) =>
            `${r.master_id}\t${r.name}\t${r.fname}\t${r.mobile}\t${r.alt}\t${r.email}\t${r.address}`
        )
        .join("\n");
      await navigator.clipboard.writeText(text);
    } catch (err) {
      console.error("Failed to copy all:", err);
    }
  }, [results]);

  const filteredResults = useMemo(() => {
    if (!filterText) return results;
    const searchText = filterText.toLowerCase();
    return results.filter((result) => {
      return (
        result.master_id.toLowerCase().includes(searchText) ||
        result.name.toLowerCase().includes(searchText) ||
        result.fname.toLowerCase().includes(searchText) ||
        result.mobile.toLowerCase().includes(searchText) ||
        result.alt.toLowerCase().includes(searchText) ||
        result.email.toLowerCase().includes(searchText) ||
        result.address.toLowerCase().includes(searchText)
      );
    });
  }, [results, filterText]);

  // Pagination logic
  const totalPages = Math.ceil(filteredResults.length / RESULTS_PER_PAGE);
  const startIndex = (currentPage - 1) * RESULTS_PER_PAGE;
  const endIndex = startIndex + RESULTS_PER_PAGE;
  const paginatedResults = useMemo(() => {
    return filteredResults.slice(startIndex, endIndex);
  }, [filteredResults, startIndex, endIndex]);

  // Reset to page 1 when filter changes
  useMemo(() => {
    setCurrentPage(1);
  }, [filterText]);

  const goToPage = useCallback(
    (page: number) => {
      if (page >= 1 && page <= totalPages) {
        setCurrentPage(page);
        window.scrollTo({ top: 0, behavior: "smooth" });
      }
    },
    [totalPages]
  );

  return (
    <div className="bg-slate-800/50 rounded-lg border border-slate-700 p-6">
      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-6">
        <div className="space-y-1">
          <div className="text-sm text-slate-300">
            Showing{" "}
            <span className="font-semibold text-slate-100">
              {formatNumber(startIndex + 1)}-
              {formatNumber(Math.min(endIndex, filteredResults.length))}
            </span>{" "}
            of{" "}
            <span className="font-semibold text-slate-100">
              {formatNumber(filteredResults.length)}
            </span>{" "}
            filtered results
            {resultsReturned !== filteredResults.length && (
              <span className="text-slate-400">
                {" "}
                (from {formatNumber(resultsReturned)} returned)
              </span>
            )}
          </div>
          <div className="text-xs text-slate-400">
            Total matches in database:{" "}
            <span className="font-semibold text-blue-400">
              {formatNumber(totalMatches)}
            </span>
          </div>
          <div className="text-xs text-slate-400 space-x-4">
            <span>Query: {queryParseTime.toFixed(2)}ms</span>
            <span>Execution: {searchExecutionTime.toFixed(2)}ms</span>
            <span>Retrieval: {documentRetrievalTime.toFixed(2)}ms</span>
            <span className="font-semibold text-green-400">
              Total: {searchTime.toFixed(2)}ms
            </span>
          </div>
        </div>
        <button
          type="button"
          onClick={copyAllResults}
          className="flex items-center gap-2 px-4 py-2 bg-green-600/20 text-green-400 rounded-lg hover:bg-green-600/30 transition-colors border border-green-500/30 cursor-pointer"
        >
          <Copy className="w-4 h-4" />
          Copy All Results
        </button>
      </div>

      <div className="mb-4">
        <input
          type="text"
          value={filterText}
          onChange={(e) => onFilterChange(e.target.value)}
          className="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors"
          placeholder="Filter results..."
        />
      </div>

      <div className="overflow-x-auto rounded-lg border border-slate-700 mb-4">
        <table className="w-full table-auto">
          <thead className="bg-slate-900/50 border-b border-slate-600">
            <tr>
              <th className="text-left py-3 px-4 text-sm font-semibold text-slate-300 whitespace-nowrap">
                Master ID
              </th>
              <th className="text-left py-3 px-4 text-sm font-semibold text-slate-300 whitespace-nowrap">
                Name
              </th>
              <th className="text-left py-3 px-4 text-sm font-semibold text-slate-300 whitespace-nowrap">
                Father Name
              </th>
              <th className="text-left py-3 px-4 text-sm font-semibold text-slate-300 whitespace-nowrap">
                Mobile
              </th>
              <th className="text-left py-3 px-4 text-sm font-semibold text-slate-300 whitespace-nowrap">
                Alt Phone
              </th>
              <th className="text-left py-3 px-4 text-sm font-semibold text-slate-300 whitespace-nowrap">
                Email
              </th>
              <th className="text-left py-3 px-4 text-sm font-semibold text-slate-300 whitespace-nowrap">
                Address
              </th>
              <th className="text-left py-3 px-4 text-sm font-semibold text-slate-300 whitespace-nowrap">
                Action
              </th>
            </tr>
          </thead>
          <tbody>
            {paginatedResults.map((result, index) => {
              const rowId = `${result.master_id}-${result.mobile}-${
                startIndex + index
              }`;
              return (
                <tr
                  key={rowId}
                  className="border-b border-slate-700/50 hover:bg-slate-700/30 transition-colors"
                >
                  <td className="py-3 px-4 align-top">
                    <div className="text-sm text-slate-300 font-mono whitespace-nowrap">
                      {result.master_id || "-"}
                    </div>
                  </td>
                  <td className="py-3 px-4 align-top">
                    <div className="text-sm text-slate-100 font-medium break-words">
                      {result.name || "-"}
                    </div>
                  </td>
                  <td className="py-3 px-4 align-top">
                    <div className="text-sm text-slate-300 break-words">
                      {result.fname || "-"}
                    </div>
                  </td>
                  <td className="py-3 px-4 align-top">
                    <div className="text-sm text-slate-100 font-mono whitespace-nowrap">
                      {result.mobile || "-"}
                    </div>
                  </td>
                  <td className="py-3 px-4 align-top">
                    <div className="text-sm text-slate-300 font-mono whitespace-nowrap">
                      {result.alt || "-"}
                    </div>
                  </td>
                  <td className="py-3 px-4 align-top">
                    <div className="text-sm text-slate-300 break-all min-w-[200px]">
                      {result.email || "-"}
                    </div>
                  </td>
                  <td className="py-3 px-4 align-top">
                    <div className="text-sm text-slate-300 break-words min-w-[300px] max-w-[600px]">
                      {result.address || "-"}
                    </div>
                  </td>
                  <td className="py-3 px-4 align-top">
                    <button
                      type="button"
                      onClick={(e) => {
                        e.stopPropagation();
                        copyToClipboard(JSON.stringify(result, null, 2), rowId);
                      }}
                      className="p-2 hover:bg-slate-700/50 rounded-lg transition-colors cursor-pointer"
                      aria-label="Copy row"
                      disabled={copiedId === rowId}
                    >
                      <Copy
                        className={`w-4 h-4 ${
                          copiedId === rowId
                            ? "text-green-400"
                            : "text-blue-400"
                        }`}
                      />
                    </button>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>

      {/* Pagination Controls */}
      {totalPages > 1 && (
        <div className="flex items-center justify-between mt-4 pt-4 border-t border-slate-700">
          <div className="text-sm text-slate-400">
            Page{" "}
            <span className="font-semibold text-slate-200">{currentPage}</span>{" "}
            of{" "}
            <span className="font-semibold text-slate-200">{totalPages}</span>
          </div>
          <div className="flex items-center gap-2">
            <button
              type="button"
              onClick={() => goToPage(currentPage - 1)}
              disabled={currentPage === 1}
              className="flex items-center gap-1 px-3 py-2 bg-slate-700/50 text-slate-200 rounded-lg hover:bg-slate-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed border border-slate-600"
            >
              <ChevronLeft className="w-4 h-4" />
              Previous
            </button>

            {/* Page numbers */}
            <div className="flex items-center gap-1">
              {Array.from({ length: Math.min(5, totalPages) }, (_, i) => {
                let pageNum;
                if (totalPages <= 5) {
                  pageNum = i + 1;
                } else if (currentPage <= 3) {
                  pageNum = i + 1;
                } else if (currentPage >= totalPages - 2) {
                  pageNum = totalPages - 4 + i;
                } else {
                  pageNum = currentPage - 2 + i;
                }

                return (
                  <button
                    key={pageNum}
                    type="button"
                    onClick={() => goToPage(pageNum)}
                    className={`px-3 py-2 rounded-lg transition-colors border ${
                      currentPage === pageNum
                        ? "bg-blue-600 text-white border-blue-500"
                        : "bg-slate-700/50 text-slate-200 border-slate-600 hover:bg-slate-700"
                    }`}
                  >
                    {pageNum}
                  </button>
                );
              })}
            </div>

            <button
              type="button"
              onClick={() => goToPage(currentPage + 1)}
              disabled={currentPage === totalPages}
              className="flex items-center gap-1 px-3 py-2 bg-slate-700/50 text-slate-200 rounded-lg hover:bg-slate-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed border border-slate-600"
            >
              Next
              <ChevronRight className="w-4 h-4" />
            </button>
          </div>
        </div>
      )}

      {filteredResults.length === 0 && results.length > 0 && (
        <div className="text-center py-8 text-slate-400">
          No results match your filter criteria
        </div>
      )}
    </div>
  );
}
