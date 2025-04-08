import axios from 'axios';
import type { ScenarioDetail, ScenarioFilters, ScenarioListItem, FilterOptions } from './types';

// Use our own server-side proxy to access the API
// This ensures browser requests go through our proxy, not directly to the API container
const API_URL = '/api';

const api = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json'
  }
});

export const scenariosApi = {
  // Get a list of scenarios with optional filters
  getScenarios: async (filters?: ScenarioFilters): Promise<ScenarioListItem[]> => {
    const params = filters || {};
    const response = await api.get('/scenarios', { params });
    return response.data;
  },

  // Get detailed information about a specific scenario
  getScenarioById: async (id: number): Promise<ScenarioDetail> => {
    const response = await api.get(`/scenarios/${id}`);
    return response.data;
  },

  // Get all available filter options
  getFilterOptions: async (): Promise<FilterOptions> => {
    const response = await api.get('/scenarios/filters/options');
    return response.data;
  }
};

// Health check API
export const healthApi = {
  check: async () => {
    const response = await api.get('/health');
    return response.data;
  }
};