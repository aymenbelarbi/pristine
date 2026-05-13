import { useState } from 'react';
import { ChevronRight, ChevronDown, File, Folder, FolderOpen } from 'lucide-react';
import { cn } from '@/lib/utils';
import { ScrollArea } from '@/components/ui/scroll-area';
import { formatBytes, formatTokens } from '@/lib/utils';
import type { FileTreeNode } from '@/types';

interface FileTreeProps {
  nodes: FileTreeNode[];
  onFileSelect?: (path: string) => void;
  selectedFile?: string;
}

export function FileTree({ nodes, onFileSelect, selectedFile }: FileTreeProps) {
  return (
    <ScrollArea className="h-full">
      <div className="p-2">
        {nodes.map((node) => (
          <TreeNode
            key={node.path}
            node={node}
            level={0}
            onFileSelect={onFileSelect}
            selectedFile={selectedFile}
          />
        ))}
      </div>
    </ScrollArea>
  );
}

interface TreeNodeProps {
  node: FileTreeNode;
  level: number;
  onFileSelect?: (path: string) => void;
  selectedFile?: string;
}

function TreeNode({ node, level, onFileSelect, selectedFile }: TreeNodeProps) {
  const [isExpanded, setIsExpanded] = useState(level < 2);
  const isDirectory = node.type === 'directory';
  const isSelected = node.path === selectedFile;

  const handleClick = () => {
    if (isDirectory) {
      setIsExpanded(!isExpanded);
    } else {
      onFileSelect?.(node.path);
    }
  };

  return (
    <div>
      <div
        className={cn(
          'flex items-center gap-2 px-2 py-1 rounded-md cursor-pointer hover:bg-accent',
          isSelected && 'bg-accent'
        )}
        style={{ paddingLeft: `${level * 16 + 8}px` }}
        onClick={handleClick}
      >
        {isDirectory ? (
          <>
            {isExpanded ? (
              <ChevronDown className="h-4 w-4 shrink-0" />
            ) : (
              <ChevronRight className="h-4 w-4 shrink-0" />
            )}
            {isExpanded ? (
              <FolderOpen className="h-4 w-4 shrink-0 text-blue-500" />
            ) : (
              <Folder className="h-4 w-4 shrink-0 text-blue-500" />
            )}
          </>
        ) : (
          <>
            <span className="w-4" />
            <File className="h-4 w-4 shrink-0 text-muted-foreground" />
          </>
        )}
        <span className="text-sm truncate flex-1">{node.name}</span>
        {node.size !== undefined && (
          <span className="text-xs text-muted-foreground">
            {formatBytes(node.size)}
          </span>
        )}
        {node.tokens !== undefined && (
          <span className="text-xs text-muted-foreground">
            {formatTokens(node.tokens)}
          </span>
        )}
      </div>
      {isDirectory && isExpanded && node.children && (
        <div>
          {node.children.map((child) => (
            <TreeNode
              key={child.path}
              node={child}
              level={level + 1}
              onFileSelect={onFileSelect}
              selectedFile={selectedFile}
            />
          ))}
        </div>
      )}
    </div>
  );
}
