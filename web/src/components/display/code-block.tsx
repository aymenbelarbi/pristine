import { useMemo, useState } from 'react';
import { Copy, Check } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { ScrollArea } from '@/components/ui/scroll-area';

interface CodeBlockProps {
  code: string;
  language?: string;
  filename?: string;
  showLineNumbers?: boolean;
  maxHeight?: string;
}

export function CodeBlock({
  code,
  filename,
  showLineNumbers = true,
  maxHeight = '500px',
}: CodeBlockProps) {
  const [copied, setCopied] = useState(false);

  const lines = useMemo(() => code.split('\n'), [code]);

  const handleCopy = async () => {
    await navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="rounded-lg border bg-muted/50 overflow-hidden">
      {filename && (
        <div className="flex items-center justify-between px-4 py-2 border-b bg-muted">
          <span className="text-sm font-medium">{filename}</span>
          <Button
            variant="ghost"
            size="sm"
            className="h-8 px-2"
            onClick={handleCopy}
          >
            {copied ? (
              <Check className="h-4 w-4" />
            ) : (
              <Copy className="h-4 w-4" />
            )}
          </Button>
        </div>
      )}
      <ScrollArea style={{ maxHeight }}>
        <div className="flex">
          {showLineNumbers && (
            <div className="py-4 px-2 text-right select-none bg-muted/50 border-r">
              {lines.map((_, i) => (
                <div
                  key={i}
                  className="text-xs text-muted-foreground leading-5 px-2"
                >
                  {i + 1}
                </div>
              ))}
            </div>
          )}
          <pre className="flex-1 p-4 overflow-x-auto">
            <code className="text-sm leading-5">{code}</code>
          </pre>
        </div>
      </ScrollArea>
    </div>
  );
}
