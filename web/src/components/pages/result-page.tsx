import { useParams, Link } from 'react-router-dom';
import { useArtifact, useArtifactStatus } from '@/hooks/useIngest';
import { FileTree } from '@/components/display/file-tree';
import { CodeBlock } from '@/components/display/code-block';
import { TokenCounter } from '@/components/display/token-counter';
import { LoadingSpinner } from '@/components/feedback/loading-spinner';
import { ErrorAlert } from '@/components/feedback/error-alert';
import { ProgressBar } from '@/components/feedback/progress-bar';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { ArrowLeft, Download, CheckCircle2, Loader2 } from 'lucide-react';
import { useState, useMemo } from 'react';
import type { FileTreeNode } from '@/types';

export function ResultPage() {
  const { jobId } = useParams<{ jobId: string }>();
  const { data: artifact, isLoading, error } = useArtifact(jobId || '');
  const { data: status } = useArtifactStatus(jobId || '');
  const [selectedFile, setSelectedFile] = useState<string | null>(null);

  const fileTree = useMemo(() => {
    if (!artifact?.files) return [];
    return buildFileTree(artifact.files);
  }, [artifact?.files]);

  const selectedFileData = useMemo(() => {
    if (!selectedFile || !artifact?.files) return null;
    return artifact.files.find((f) => f.path === selectedFile);
  }, [selectedFile, artifact?.files]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-[400px]">
        <LoadingSpinner size="lg" text="Loading artifact..." />
      </div>
    );
  }

  if (error) {
    return (
      <ErrorAlert
        title="Failed to load artifact"
        message={error.message}
      />
    );
  }

  if (!artifact) {
    return (
      <ErrorAlert
        title="Not found"
        message="The requested artifact could not be found."
      />
    );
  }

  const isProcessing = artifact.status === 'pending' || artifact.status === 'processing';

  if (isProcessing) {
    return (
      <div className="max-w-2xl mx-auto space-y-6">
        <div className="flex items-center gap-4">
          <Button variant="ghost" size="icon" asChild>
            <Link to="/">
              <ArrowLeft className="h-4 w-4" />
            </Link>
          </Button>
          <div>
            <h1 className="text-2xl font-bold">Processing...</h1>
            <p className="text-muted-foreground">{artifact.url}</p>
          </div>
        </div>

        <Card>
          <CardContent className="pt-6 space-y-4">
            <div className="flex items-center gap-2">
              <Loader2 className="h-5 w-5 animate-spin text-primary" />
              <span className="font-medium">
                {artifact.status === 'pending' ? 'Queued for processing' : 'Analyzing repository...'}
              </span>
            </div>
            <ProgressBar value={status?.progress || 0} label="Progress" />
          </CardContent>
        </Card>
      </div>
    );
  }

  if (artifact.status === 'failed') {
    return (
      <div className="max-w-2xl mx-auto space-y-6">
        <div className="flex items-center gap-4">
          <Button variant="ghost" size="icon" asChild>
            <Link to="/">
              <ArrowLeft className="h-4 w-4" />
            </Link>
          </Button>
          <div>
            <h1 className="text-2xl font-bold">Processing Failed</h1>
            <p className="text-muted-foreground">{artifact.url}</p>
          </div>
        </div>

        <ErrorAlert
          title="Processing failed"
          message={artifact.error || 'An unknown error occurred'}
        />
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <Button variant="ghost" size="icon" asChild>
            <Link to="/">
              <ArrowLeft className="h-4 w-4" />
            </Link>
          </Button>
          <div>
            <h1 className="text-2xl font-bold">Context Pack</h1>
            <p className="text-muted-foreground truncate max-w-md">{artifact.url}</p>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <Badge variant="outline" className="gap-1">
            <CheckCircle2 className="h-3 w-3 text-green-500" />
            Completed
          </Badge>
          <Button variant="outline" size="sm">
            <Download className="h-4 w-4 mr-2" />
            Download
          </Button>
        </div>
      </div>

      <div className="grid gap-4 md:grid-cols-3">
        <TokenCounter
          current={artifact.totalTokens}
          label="Total Tokens"
          showProgress={false}
        />
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium">Files</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{artifact.totalFiles}</div>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium">Size</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {(artifact.totalSize / 1024).toFixed(1)} KB
            </div>
          </CardContent>
        </Card>
      </div>

      <div className="grid gap-6 lg:grid-cols-[300px_1fr]">
        <Card className="h-[600px]">
          <CardHeader>
            <CardTitle className="text-base">File Tree</CardTitle>
          </CardHeader>
          <CardContent className="p-0">
            <FileTree
              nodes={fileTree}
              onFileSelect={setSelectedFile}
              selectedFile={selectedFile || undefined}
            />
          </CardContent>
        </Card>

        <Card className="h-[600px]">
          <CardHeader>
            <CardTitle className="text-base">
              {selectedFileData?.path || 'Select a file'}
            </CardTitle>
          </CardHeader>
          <CardContent>
            {selectedFileData ? (
              <CodeBlock
                code={selectedFileData.content}
                filename={selectedFileData.path}
                language={selectedFileData.language}
                maxHeight="500px"
              />
            ) : (
              <div className="flex items-center justify-center h-[500px] text-muted-foreground">
                Select a file from the tree to view its contents
              </div>
            )}
          </CardContent>
        </Card>
      </div>

      {artifact.summary && (
        <Card>
          <CardHeader>
            <CardTitle>Summary</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground whitespace-pre-wrap">
              {artifact.summary}
            </p>
          </CardContent>
        </Card>
      )}
    </div>
  );
}

function buildFileTree(files: { path: string }[]): FileTreeNode[] {
  const root: FileTreeNode[] = [];

  files.forEach((file) => {
    const parts = file.path.split('/');
    let current = root;

    parts.forEach((part, index) => {
      const isFile = index === parts.length - 1;
      let node = current.find((n) => n.name === part);

      if (!node) {
        node = {
          name: part,
          path: parts.slice(0, index + 1).join('/'),
          type: isFile ? 'file' : 'directory',
          children: isFile ? undefined : [],
        };
        current.push(node);
      }

      if (!isFile && node.children) {
        current = node.children;
      }
    });
  });

  return root;
}
