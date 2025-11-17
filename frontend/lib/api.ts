import axios from "axios";

const API_URL = "http://localhost:8080";

export interface SearchResult {
  master_id: string;
  mobile: string;
  alt: string;
  name: string;
  fname: string;
  address: string;
  email: string;
}

export interface SearchResponse {
  results: SearchResult[];
  total_matches: number;
  results_returned: number;
  query_parse_time_ms: number;
  search_execution_time_ms: number;
  document_retrieval_time_ms: number;
  total_time_ms: number;
}

export interface SearchRequest {
  master_id?: string;
  name?: string;
  fname?: string;
  alt?: string;
  email?: string;
  address?: string;
  mobile?: string;
  filter?: "AND" | "OR";
}

export async function searchAPI(
  payload: SearchRequest
): Promise<SearchResponse> {
  const response = await axios.post<SearchResponse>(
    `${API_URL}/search`,
    payload
  );
  return response.data;
}
