"use client";

import RefineResults from "@/components/RefineResults";
import ResultsTable from "@/components/ResultsTable";
import SearchForm from "@/components/SearchForm";
import { searchAPI, type SearchResult } from "@/lib/api";
import { useState, useCallback } from "react";

interface SearchFields {
  master_id: string;
  name: string;
  fname: string;
  alt: string;
  email: string;
  address: string;
  mobile: string;
}

export default function Home() {
  const [searchFields, setSearchFields] = useState<SearchFields>({
    master_id: "",
    name: "",
    fname: "",
    alt: "",
    email: "",
    address: "",
    mobile: "",
  });
  const [searchMode, setSearchMode] = useState<"AND" | "OR">("AND");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [searchStats, setSearchStats] = useState<{
    totalMatches: number;
    resultsReturned: number;
    totalTime: number;
    queryParseTime: number;
    searchExecutionTime: number;
    documentRetrievalTime: number;
  }>({
    totalMatches: 0,
    resultsReturned: 0,
    totalTime: 0,
    queryParseTime: 0,
    searchExecutionTime: 0,
    documentRetrievalTime: 0,
  });
  const [filterText, setFilterText] = useState("");

  const handleSearch = useCallback(async () => {
    if (loading) return;

    setLoading(true);
    setError(null);

    try {
      const payload: Record<string, string> = {
        filter: searchMode,
      };

      for (const [key, value] of Object.entries(searchFields)) {
        if (value.trim()) {
          payload[key] = value.trim();
        }
      }

      const response = await searchAPI(
        payload as Parameters<typeof searchAPI>[0]
      );

      setResults(response.results);
      setSearchStats({
        totalMatches: response.total_matches,
        resultsReturned: response.results_returned,
        totalTime: response.total_time_ms,
        queryParseTime: response.query_parse_time_ms,
        searchExecutionTime: response.search_execution_time_ms,
        documentRetrievalTime: response.document_retrieval_time_ms,
      });
    } catch (err) {
      console.error("Search error:", err);
      setError(
        err instanceof Error ? err.message : "Search failed. Please try again."
      );
    } finally {
      setLoading(false);
    }
  }, [searchFields, searchMode, loading]);

  const handleReset = useCallback(() => {
    setSearchFields({
      master_id: "",
      name: "",
      fname: "",
      alt: "",
      email: "",
      address: "",
      mobile: "",
    });
    setResults([]);
    setFilterText("");
    setError(null);
    setSearchStats({
      totalMatches: 0,
      resultsReturned: 0,
      totalTime: 0,
      queryParseTime: 0,
      searchExecutionTime: 0,
      documentRetrievalTime: 0,
    });
  }, []);

  return (
    <div className="min-h-screen bg-slate-900">
      <div className="w-full px-4 sm:px-6 lg:px-8 py-6 sm:py-8">
        <SearchForm
          searchFields={searchFields}
          searchMode={searchMode}
          loading={loading}
          onFieldChange={setSearchFields}
          onModeChange={setSearchMode}
          onSearch={handleSearch}
          onReset={handleReset}
        />

        {error && (
          <div className="mb-6 p-4 bg-red-900/20 border border-red-500/30 rounded-lg text-red-300">
            {error}
          </div>
        )}

        {loading && (
          <div className="text-center py-12">
            <div className="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
            <p className="mt-4 text-slate-400">Searching...</p>
          </div>
        )}

        {!loading && results.length > 0 && (
          <>
            <RefineResults
              resultCount={results.length}
              filterText={filterText}
              onFilterChange={setFilterText}
            />

            <ResultsTable
              results={results}
              totalMatches={searchStats.totalMatches}
              resultsReturned={searchStats.resultsReturned}
              searchTime={searchStats.totalTime}
              queryParseTime={searchStats.queryParseTime}
              searchExecutionTime={searchStats.searchExecutionTime}
              documentRetrievalTime={searchStats.documentRetrievalTime}
              filterText={filterText}
              onFilterChange={setFilterText}
            />
          </>
        )}

        {results.length === 0 && !loading && !error && (
          <div className="text-center py-12 text-slate-400">
            Enter search criteria and click Search to find results
          </div>
        )}
      </div>
    </div>
  );
}
