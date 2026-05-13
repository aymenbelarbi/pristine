import { useMutation, useQuery } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';
import { ingestRepository, getArtifact, getArtifactStatus } from '@/api/client';
import { useArtifactStore } from '@/stores/artifactStore';
import type { IngestOptions } from '@/types';

/**
 * Hook for ingesting a repository and starting artifact processing.
 *
 * This hook provides a mutation function that:
 * - Sends the ingestion request to the API
 * - Adds the new artifact to the store
 * - Navigates to the result page on success
 * - Handles loading and error states
 *
 * @returns A React Query mutation object for ingestion
 *
 * @example
 * ```typescript
 * const ingest = useIngest();
 *
 * const handleSubmit = async () => {
 *   await ingest.mutateAsync({
 *     url: 'https://github.com/user/repo',
 *     tokenBudget: 50000
 *   });
 * };
 *
 * return (
 *   <button onClick={handleSubmit} disabled={ingest.isPending}>
 *     {ingest.isPending ? 'Processing...' : 'Ingest Repository'}
 * </button>
 * );
 * ```
 */
export function useIngest() {
  const navigate = useNavigate();
  const { addArtifact, setLoading, setError } = useArtifactStore();

  return useMutation({
    mutationFn: (options: IngestOptions) => ingestRepository(options),
    onMutate: () => {
      setLoading(true);
      setError(null);
    },
    onSuccess: (data) => {
      addArtifact({
        id: data.jobId,
        jobId: data.jobId,
        status: data.status,
        url: '',
        files: [],
        totalTokens: 0,
        totalFiles: 0,
        totalSize: 0,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      });
      setLoading(false);
      navigate(`/result/${data.jobId}`);
    },
    onError: (error: Error) => {
      setError(error.message);
      setLoading(false);
    },
  });
}

/**
 * Hook for fetching and caching an artifact by job ID.
 *
 * This hook provides a query that:
 * - Fetches the artifact from the API
 * - Updates the store with the latest data
 * - Only runs when a valid job ID is provided
 *
 * @param jobId - The unique job identifier
 * @returns A React Query query object for the artifact
 *
 * @example
 * ```typescript
 * const { data, isLoading, error } = useArtifact('job-123');
 *
 * if (isLoading) return <LoadingSpinner />;
 * if (error) return <ErrorAlert message={error.message} />;
 * if (data) return <ArtifactView artifact={data} />;
 * ```
 */
export function useArtifact(jobId: string) {
  const { updateArtifact } = useArtifactStore();

  return useQuery({
    queryKey: ['artifact', jobId],
    queryFn: async () => {
      const data = await getArtifact(jobId);
      updateArtifact(jobId, data);
      return data;
    },
    enabled: !!jobId,
  });
}

/**
 * Hook for polling the status of an in-progress artifact job.
 *
 * This hook provides a query that:
 * - Polls the job status every 2 seconds
 * - Automatically stops polling when job completes or fails
 * - Only runs when a valid job ID is provided
 *
 * @param jobId - The unique job identifier
 * @returns A React Query query object for the job status
 *
 * @example
 * ```typescript
 * const { data } = useArtifactStatus('job-123');
 *
 * return (
 *   <div>
 *     <p>Status: {data?.status}</p>
 *     {data?.progress && <ProgressBar value={data.progress} />}
 *   </div>
 * );
 * ```
 */
export function useArtifactStatus(jobId: string) {
  return useQuery({
    queryKey: ['artifact-status', jobId],
    queryFn: () => getArtifactStatus(jobId),
    enabled: !!jobId,
    refetchInterval: (query) => {
      const data = query.state.data;
      if (data?.status === 'completed' || data?.status === 'failed') {
        return false;
      }
      return 2000;
    },
  });
}
