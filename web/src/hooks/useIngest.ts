import { useMutation, useQuery } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';
import { ingestRepository, getArtifact, getArtifactStatus } from '@/api/client';
import { useArtifactStore } from '@/stores/artifactStore';
import type { IngestOptions } from '@/types';

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
