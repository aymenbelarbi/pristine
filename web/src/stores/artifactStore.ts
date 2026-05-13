import { create } from 'zustand';
import type { Artifact } from '@/types';

/**
 * State interface for the artifact store.
 *
 * Manages the collection of artifacts and the currently selected artifact,
 * along with loading and error states.
 */
interface ArtifactState {
  /** The currently selected artifact for detailed view */
  currentArtifact: Artifact | null;
  /** List of all artifacts */
  artifacts: Artifact[];
  /** Whether an operation is in progress */
  isLoading: boolean;
  /** Error message if an operation failed */
  error: string | null;
  /** Set the current artifact */
  setCurrentArtifact: (artifact: Artifact | null) => void;
  /** Replace the entire artifacts list */
  setArtifacts: (artifacts: Artifact[]) => void;
  /** Add a new artifact to the beginning of the list */
  addArtifact: (artifact: Artifact) => void;
  /** Update an existing artifact by ID or job ID */
  updateArtifact: (id: string, updates: Partial<Artifact>) => void;
  /** Remove an artifact by ID or job ID */
  removeArtifact: (id: string) => void;
  /** Set the loading state */
  setLoading: (loading: boolean) => void;
  /** Set the error message */
  setError: (error: string | null) => void;
}

/**
 * Zustand store for managing artifacts.
 *
 * Provides a centralized state for artifact data with actions for
 * adding, updating, and removing artifacts.
 *
 * @example
 * ```typescript
 * // Access state
 * const { artifacts, currentArtifact } = useArtifactStore();
 *
 * // Use actions
 * const { addArtifact, updateArtifact, removeArtifact } = useArtifactStore();
 *
 * // Add new artifact
 * addArtifact({
 *   id: 'job-123',
 *   jobId: 'job-123',
 *   status: 'processing',
 *   // ... other fields
 * });
 *
 * // Update artifact
 * updateArtifact('job-123', { status: 'completed' });
 * ```
 */
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
