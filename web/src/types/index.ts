export interface IngestOptions {
  url: string;
  branch?: string;
  commit?: string;
  includePatterns?: string[];
  excludePatterns?: string[];
  maxFileSize?: number;
  maxFiles?: number;
  tokenBudget?: number;
  packType?: 'overview' | 'task' | 'review';
  query?: string;
}

export interface IngestResponse {
  jobId: string;
  status: 'pending' | 'processing' | 'completed' | 'failed';
  message?: string;
}

export interface ArtifactFile {
  path: string;
  content: string;
  size: number;
  tokens: number;
  language?: string;
}

export interface Artifact {
  id: string;
  jobId: string;
  status: 'pending' | 'processing' | 'completed' | 'failed';
  url: string;
  branch?: string;
  commit?: string;
  summary?: string;
  files: ArtifactFile[];
  totalTokens: number;
  totalFiles: number;
  totalSize: number;
  createdAt: string;
  updatedAt: string;
  error?: string;
}

export interface HealthStatus {
  status: 'healthy' | 'degraded' | 'unhealthy';
  version: string;
  uptime: number;
}

export interface FileTreeNode {
  name: string;
  path: string;
  type: 'file' | 'directory';
  children?: FileTreeNode[];
  size?: number;
  tokens?: number;
}
