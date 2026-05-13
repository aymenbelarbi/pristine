import { Hash, TrendingUp, TrendingDown, Minus } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Progress } from '@/components/ui/progress';
import { formatTokens } from '@/lib/utils';
import { cn } from '@/lib/utils';

interface TokenCounterProps {
  current: number;
  max?: number;
  label?: string;
  showProgress?: boolean;
  trend?: 'up' | 'down' | 'neutral';
  className?: string;
}

export function TokenCounter({
  current,
  max,
  label = 'Tokens',
  showProgress = true,
  trend,
  className,
}: TokenCounterProps) {
  const percentage = max ? (current / max) * 100 : 0;
  const isOverBudget = max ? current > max : false;

  const trendConfig = {
    up: { icon: TrendingUp, color: 'text-green-500' },
    down: { icon: TrendingDown, color: 'text-red-500' },
    neutral: { icon: Minus, color: 'text-muted-foreground' },
  };

  const TrendIcon = trend ? trendConfig[trend].icon : null;
  const trendColor = trend ? trendConfig[trend].color : '';

  return (
    <Card className={cn('overflow-hidden', className)}>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm font-medium flex items-center justify-between">
          <span className="flex items-center gap-2">
            <Hash className="h-4 w-4" />
            {label}
          </span>
          {TrendIcon && <TrendIcon className={cn('h-4 w-4', trendColor)} />}
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div className="text-2xl font-bold">{formatTokens(current)}</div>
        {max && (
          <p className="text-xs text-muted-foreground mt-1">
            of {formatTokens(max)} max
          </p>
        )}
        {showProgress && max && (
          <Progress
            value={Math.min(percentage, 100)}
            className={cn('h-2 mt-2', isOverBudget && 'bg-red-500')}
          />
        )}
        {isOverBudget && (
          <p className="text-xs text-destructive mt-1">
            Over budget by {formatTokens(current - max!)}
          </p>
        )}
      </CardContent>
    </Card>
  );
}
