import axios from 'axios';
import type { IngestOptions, IngestResponse, Artifact, HealthStatus } from '@/types';

/**
 * Axios instance for making API requests to the Pristine server.
 *
 * Configured with:
 * - Base URL from environment variable or default `/api/v1`
 * - 60 second timeout
 * - JSON content type header
 * - Error logging interceptor
 */
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

/**
 * Ingest a repository and start processing.
 *
 * @param options - The ingestion options including URL, branch, and filters
 * @returns Promise resolving to the ingestion response with job ID
 *
 * @example
 * ```typescript
 * const response = await ingestRepository({
 *   url: 'https://github.com/user/repo',
 *   branch: 'main',
 *   tokenBudget: 50000
 * });
 * console.log(response.jobId);
 * ```
 */
export async function ingestRepository(options: IngestOptions): Promise<IngestResponse> {
  const response = await apiClient.post<IngestResponse>('/ingest', options);
  return response.data;
}

/**
 * Get a processed artifact by job ID.
 *
 * @param jobId - The unique job identifier
 * @returns Promise resolving to the artifact with files and metadata
 *
 * @example
 * ```typescript
 * const artifact = await getArtifact('job-123');
 * console.log(artifact.files.length);
 * ```
 */
export async function getArtifact(jobId: string): Promise<Artifact> {
  const response = await apiClient.get<Artifact>(`/artifacts/${jobId}`);
  return response.data;
}

/**
 * Get the status of an in-progress artifact job.
 *
 * @param jobId - The unique job identifier
 * @returns Promise resolving to the job status and optional progress
 *
 * @example
 * ```typescript
 * const status = await getArtifactStatus('job-123');
 * if (status.status === 'completed') {
 *   // Process artifact
 * }
 * ```
 */
export async function getArtifactStatus(jobId: string): Promise<{ status: string; progress?: number }> {
  const response = await apiClient.get<{ status: string; progress?: number }>(`/artifacts/${jobId}/status`);
  return response.data;
}

/**
 * Check the health status of the Pristine API server.
 *
 * @returns Promise resolving to the health status and version
 *
 * @example
 * ```typescript
 * const health = await healthCheck();
 * if (health.status === 'healthy') {
 *   // Server is operational
 * }
 * ```
 */
export async function healthCheck(): Promise<HealthStatus> {
  const response = await apiClient.get<HealthStatus>('/health');
  return response.data;
}

/**
 * List all available artifacts.
 *
 * @returns Promise resolving to an array of artifacts
 *
 * @example
 * ```typescript
 * const artifacts = await listArtifacts();
 * artifacts.forEach(a => console.log(a.id, a.status));
 * ```
 */
export async function listArtifacts(): Promise<Artifact[]> {
  const response = await apiClient.get<Artifact[]>('/artifacts');
  return response.data;
}

/**
 * Delete an artifact by job ID.
 *
 * @param jobId - The unique job identifier to delete
 * @returns Promise that resolves when deletion is complete
 *
 * @example
 * ```typescript
 * await deleteArtifact('job-123');
 * ```
 */
export async function deleteArtifact(jobId: string): Promise<void> {
  await apiClient.delete(`/artifacts/${jobId}`);
}
