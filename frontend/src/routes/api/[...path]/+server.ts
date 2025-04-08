import { error, json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

const API_URL = process.env.VITE_API_URL || 'http://api:8080/api';

// Proxy all API requests
export const GET: RequestHandler = async ({ params, url }) => {
  try {
    const path = params.path;
    const queryString = url.search;
    const apiUrl = `${API_URL}/${path}${queryString}`;
    
    console.log(`Proxying GET request to: ${apiUrl}`);
    
    const response = await fetch(apiUrl);
    
    if (!response.ok) {
      throw error(response.status, `API returned ${response.status}`);
    }
    
    const data = await response.json();
    return json(data);
  } catch (err) {
    console.error('API proxy error:', err);
    throw error(500, 'Failed to fetch data from API');
  }
};

// Add other methods as needed (POST, PUT, DELETE)
export const POST: RequestHandler = async ({ params, request, url }) => {
  try {
    const path = params.path;
    const queryString = url.search;
    const apiUrl = `${API_URL}/${path}${queryString}`;
    
    const body = await request.json();
    
    console.log(`Proxying POST request to: ${apiUrl}`);
    
    const response = await fetch(apiUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(body)
    });
    
    if (!response.ok) {
      throw error(response.status, `API returned ${response.status}`);
    }
    
    const data = await response.json();
    return json(data);
  } catch (err) {
    console.error('API proxy error:', err);
    throw error(500, 'Failed to send data to API');
  }
};
