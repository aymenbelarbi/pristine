import axios from 'axios';
import type { IngestOptions, IngestResponse, Artifact, HealthStatus } from '@/types';

export const apiClient = axios.create({
  baseURL: import.meta.env.VITE_API_URL || '/api/v1',
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
  const response = await apiClient.get<Artifact>(`/artifacts/${jobId}`);
  return response.data;
}

export async function getArtifactStatus(jobId: string): Promise<{ status: string; progress?: number }> {
  const response = await apiClient.get<{ status: string; progress?: number }>(`/artifacts/${jobId}/status`);
  return response.data;
}

export async function healthCheck(): Promise<HealthStatus> {
  const response = await apiClient.get<HealthStatus>('/health');
  return response.data;
}

export async function listArtifacts(): Promise<Artifact[]> {
  const response = await apiClient.get<Artifact[]>('/artifacts');
  return response.data;
}

export async function deleteArtifact(jobId: string): Promise<void> {
  await apiClient.delete(`/artifacts/${jobId}`);
}
