import { useState } from 'react';
import { X, Plus } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { Label } from '@/components/ui/label';

interface PatternInputProps {
  label: string;
  placeholder?: string;
  patterns: string[];
  onChange: (patterns: string[]) => void;
}

export function PatternInput({
  label,
  placeholder = 'Add pattern...',
  patterns,
  onChange,
}: PatternInputProps) {
  const [inputValue, setInputValue] = useState('');

  const addPattern = () => {
    if (inputValue.trim() && !patterns.includes(inputValue.trim())) {
      onChange([...patterns, inputValue.trim()]);
      setInputValue('');
    }
  };

  const removePattern = (pattern: string) => {
    onChange(patterns.filter((p) => p !== pattern));
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      addPattern();
    }
  };

  return (
    <div className="space-y-2">
      <Label>{label}</Label>
      <div className="flex gap-2">
        <Input
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder={placeholder}
          className="flex-1"
        />
        <Button type="button" variant="outline" size="icon" onClick={addPattern}>
          <Plus className="h-4 w-4" />
        </Button>
      </div>
      {patterns.length > 0 && (
        <div className="flex flex-wrap gap-2 mt-2">
          {patterns.map((pattern) => (
            <Badge key={pattern} variant="secondary" className="gap-1">
              {pattern}
              <button
                type="button"
                onClick={() => removePattern(pattern)}
                className="ml-1 hover:text-destructive"
              >
                <X className="h-3 w-3" />
              </button>
            </Badge>
          ))}
        </div>
      )}
    </div>
  );
}
