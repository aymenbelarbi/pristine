import { Progress } from '@/components/ui/progress';
import { cn } from '@/lib/utils';

interface ProgressBarProps {
  value: number;
  max?: number;
  label?: string;
  showPercentage?: boolean;
  className?: string;
}

export function ProgressBar({
  value,
  max = 100,
  label,
  showPercentage = true,
  className,
}: ProgressBarProps) {
  const percentage = Math.min(Math.round((value / max) * 100), 100);

  return (
    <div className={cn('space-y-2', className)}>
      {(label || showPercentage) && (
        <div className="flex items-center justify-between text-sm">
          {label && <span className="text-muted-foreground">{label}</span>}
          {showPercentage && <span className="font-medium">{percentage}%</span>}
        </div>
      )}
      <Progress value={percentage} className="h-2" />
    </div>
  );
}
