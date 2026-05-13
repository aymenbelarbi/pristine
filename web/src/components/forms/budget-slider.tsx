import { Label } from '@/components/ui/label';
import { Slider } from '@/components/ui/slider';
import { formatTokens } from '@/lib/utils';

interface BudgetSliderProps {
  label: string;
  value: number;
  onChange: (value: number) => void;
  min?: number;
  max?: number;
  step?: number;
}

export function BudgetSlider({
  label,
  value,
  onChange,
  min = 1000,
  max = 200000,
  step = 1000,
}: BudgetSliderProps) {
  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between">
        <Label>{label}</Label>
        <span className="text-sm text-muted-foreground">
          {formatTokens(value)} tokens
        </span>
      </div>
      <Slider
        value={[value]}
        onValueChange={(values) => onChange(values[0])}
        min={min}
        max={max}
        step={step}
        className="w-full"
      />
      <div className="flex justify-between text-xs text-muted-foreground">
        <span>{formatTokens(min)}</span>
        <span>{formatTokens(max)}</span>
      </div>
    </div>
  );
}
