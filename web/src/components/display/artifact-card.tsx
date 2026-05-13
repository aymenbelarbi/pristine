import { FileText, Clock, HardDrive, Hash, GitBranch, AlertCircle, CheckCircle2, Loader2 } from 'lucide-react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';
import { formatBytes, formatTokens } from '@/lib/utils';
import type { Artifact } from '@/types';

interface ArtifactCardProps {
  artifact: Artifact;
  onView?: () => void;
  onDelete?: () => void;
}

export function ArtifactCard({ artifact, onView, onDelete }: ArtifactCardProps) {
  const statusConfig = {
    pending: { icon: Clock, color: 'text-yellow-500', bg: 'bg-yellow-500/10' },
    processing: { icon: Loader2, color: 'text-blue-500', bg: 'bg-blue-500/10' },
    completed: { icon: CheckCircle2, color: 'text-green-500', bg: 'bg-green-500/10' },
    failed: { icon: AlertCircle, color: 'text-red-500', bg: 'bg-red-500/10' },
  };

  const config = statusConfig[artifact.status];
  const StatusIcon = config.icon;

  return (
    <Card className="hover:shadow-md transition-shadow">
      <CardHeader>
        <div className="flex items-start justify-between">
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2 mb-1">
              <StatusIcon className={`h-4 w-4 ${config.color} ${artifact.status === 'processing' ? 'animate-spin' : ''}`} />
              <Badge variant={artifact.status === 'completed' ? 'default' : 'secondary'}>
                {artifact.status}
              </Badge>
            </div>
            <CardTitle className="text-lg truncate">{artifact.url}</CardTitle>
            <CardDescription className="truncate">
              {artifact.branch && (
                <span className="flex items-center gap-1">
                  <GitBranch className="h-3 w-3" />
                  {artifact.branch}
                </span>
              )}
            </CardDescription>
          </div>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        {artifact.status === 'processing' && (
          <Progress value={50} className="h-2" />
        )}

        <div className="grid grid-cols-3 gap-4 text-sm">
          <div className="flex items-center gap-2">
            <FileText className="h-4 w-4 text-muted-foreground" />
            <span>{artifact.totalFiles} files</span>
          </div>
          <div className="flex items-center gap-2">
            <Hash className="h-4 w-4 text-muted-foreground" />
            <span>{formatTokens(artifact.totalTokens)}</span>
          </div>
          <div className="flex items-center gap-2">
            <HardDrive className="h-4 w-4 text-muted-foreground" />
            <span>{formatBytes(artifact.totalSize)}</span>
          </div>
        </div>

        {artifact.error && (
          <p className="text-sm text-destructive">{artifact.error}</p>
        )}

        <div className="flex gap-2">
          <Button
            variant="default"
            size="sm"
            onClick={onView}
            disabled={artifact.status !== 'completed'}
          >
            View
          </Button>
          <Button variant="outline" size="sm" onClick={onDelete}>
            Delete
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}
