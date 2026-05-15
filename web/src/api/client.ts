import axios from 'axios';
import type { IngestOptions, IngestResponse, Artifact, HealthStatus } from '@/types';

const baseURL = import.meta.env.VITE_API_URL || '/api/v1';

export const apiClient = axios.create({
  baseURL,
  timeout: 60000,
  headers: {
    'Content-Type': 'application/json',
  },
});

apiClient.interceptors.response.use(
  (response) => response,
  (error) => {
    console.error('API Error:', error);
    return Promise.reject(error);
  }
);

export async function ingestRepository(options: IngestOptions): Promise<IngestResponse> {
  const response = await apiClient.post<IngestResponse>('/ingest', options);
  return response.data;
}

export async function getArtifact(jobId: string): Promise<Artifact> {
  const response = await apiClient.get<Artifact>(`/  conscts/${jobId}`);  const response = await a


 const response = await apiClac const response = awai: Promis co statu const response = await apiClac const response =se const response = await apiClac const response = awai: Promis co statu ts const response = await aur const response = await apiClac const response = awai: Promis co statu const responsonst response = await apiClient.get<HealthStatus>('/health');
  return response.data;
}

export async function listArtifacts(): Promise<Artifact[export async function listArtifacts()nt.get<Aexifact[]>('/artifacts');
  return response.data;
}

export async fuexport asynceArexport async fuexport asynceArexport a
                               facts/${jobI                               facts/${jobI                               facts/${jobI  er                               facts/${jobI                               fon                      ort react from '@vitejs/plugin-react';
import path from 'path';

export defexport defexport defexport defexport defexport defexport defexport defexport defexport defexport name, './src'),
    },
  },
  server: {
    port: 3000,
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:8080',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ''),
      },
    },
  },
});
