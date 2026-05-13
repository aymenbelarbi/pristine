import { create } from 'zustand';
import type { Artifact } from '@/types';

interface ArtifactState {
  currentArtifact: Artifact | null;
  artifacts: Artifact[];
  isLoading: boolean;
  error: string | null;
  setCurrentArtifact: (artifact: Artifact | null) => void;
  setArtifacts: (artifacts: Artifact[]) => void;
  addArtifact: (artifact: Artifact) => void;
  updateArtifact: (id: string, updates: Partial<Artifact>) => void;
  removeArtifact: (id: string) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}

export const useArtifactStore = create<ArtifactState>((set) => ({
  currentArtifact: null,
  artifacts: [],
  isLoading: false,
  error: null,
  setCurrentArtifact: (artifact) => set({ currentArtifact: artifact }),
  setArtifacts: (artifacts) => set({ artifacts }),
  addArtifact: (artifact) =>
    set((state) => ({ artifacts: [artifact, ...state.artifacts] })),
  updateArtifact: (id, updates) =>
    set((state) => ({
      artifacts: state.artifacts.map((a) =>
        a.id === id || a.jobId === id ? { ...a, ...updates } : a
      ),
      currentArtifact:
        state.currentArtifact?.id === id || state.currentArtifact?.jobId === id
          ? { ...state.currentArtifact, ...updates }
          : state.currentArtifact,
    })),
  removeArtifact: (id) =>
    set((state) => ({
      artifacts: state.artifacts.filter((a) => a.id !== id && a.jobId !== id),
    })),
  setLoading: (loading) => set({ isLoading: loading }),
  setError: (error) => set({ error }),
}));
