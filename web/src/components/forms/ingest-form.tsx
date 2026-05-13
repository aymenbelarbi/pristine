import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { Loader2, GitBranch, FileCode, Search, Settings2 } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { useIngest } from '@/hooks/useIngest';
import { PatternInput } from './pattern-input';
import { BudgetSlider } from './budget-slider';

const ingestSchema = z.object({
  url: z.string().url('Please enter a valid URL'),
  branch: z.string().optional(),
  packType: z.enum(['overview', 'task', 'review']).default('overview'),
  query: z.string().optional(),
  maxFileSize: z.number().default(1048576),
  maxFiles: z.number().default(1000),
  tokenBudget: z.number().default(50000),
  includePatterns: z.array(z.string()).default([]),
  excludePatterns: z.array(z.string()).default(['node_modules', '.git', 'dist', 'build']),
});

type IngestFormData = z.infer<typeof ingestSchema>;

export function IngestForm() {
  const { mutate: ingest, isPending } = useIngest();
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [includePatterns, setIncludePatterns] = useState<string[]>([]);
  const [excludePatterns, setExcludePatterns] = useState<string[]>(['node_modules', '.git', 'dist', 'build']);
  const [tokenBudget, setTokenBudget] = useState(50000);

  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
  } = useForm<IngestFormData>({
    resolver: zodResolver(ingestSchema),
    defaultValues: {
      packType: 'overview',
      maxFileSize: 1048576,
      maxFiles: 1000,
      tokenBudget: 50000,
    },
  });

  const packType = watch('packType');

  const onSubmit = (data: IngestFormData) => {
    ingest({
      ...data,
      includePatterns,
      excludePatterns,
      tokenBudget,
    });
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Repository URL</CardTitle>
          <CardDescription>
            Enter the URL of the Git repository you want to analyze
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="url">URL</Label>
            <Input
              id="url"
              placeholder="https://github.com/user/repo"
              {...register('url')}
            />
            {errors.url && (
              <p className="text-sm text-destructive">{errors.url.message}</p>
            )}
          </div>

          <div className="space-y-2">
            <Label htmlFor="branch">Branch (optional)</Label>
            <Input
              id="branch"
              placeholder="main"
              {...register('branch')}
            />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Pack Type</CardTitle>
          <CardDescription>
            Choose the type of context pack to generate
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid gap-4 md:grid-cols-3">
            <Card
              className={`cursor-pointer transition-colors ${
                packType === 'overview' ? 'border-primary' : ''
              }`}
              onClick={() => {}}
            >
              <CardHeader>
                <FileCode className="h-8 w-8 mb-2" />
                <CardTitle className="text-base">Overview Pack</CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-sm text-muted-foreground">
                  Understand a codebase quickly with architecture, entrypoints, and key files.
                </p>
              </CardContent>
            </Card>

            <Card
              className={`cursor-pointer transition-colors ${
                packType === 'task' ? 'border-primary' : ''
              }`}
              onClick={() => {}}
            >
              <CardHeader>
                <Search className="h-8 w-8 mb-2" />
                <CardTitle className="text-base">Task Pack</CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-sm text-muted-foreground">
                  Get focused context for a specific task or query.
                </p>
              </CardContent>
            </Card>

            <Card
              className={`cursor-pointer transition-colors ${
                packType === 'review' ? 'border-primary' : ''
              }`}
              onClick={() => {}}
            >
              <CardHeader>
                <GitBranch className="h-8 w-8 mb-2" />
                <CardTitle className="text-base">Review Pack</CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-sm text-muted-foreground">
                  Review a PR or diff with impacted files and tests.
                </p>
              </CardContent>
            </Card>
          </div>

          {packType === 'task' && (
            <div className="mt-4 space-y-2">
              <Label htmlFor="query">Task Description</Label>
              <Textarea
                id="query"
                placeholder="Describe the task you want to accomplish..."
                {...register('query')}
              />
            </div>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <div>
            <CardTitle>Advanced Options</CardTitle>
            <CardDescription>
              Configure file patterns and token budget
            </CardDescription>
          </div>
          <Button
            type="button"
            variant="ghost"
            size="sm"
            onClick={() => setShowAdvanced(!showAdvanced)}
          >
            <Settings2 className="h-4 w-4 mr-2" />
            {showAdvanced ? 'Hide' : 'Show'}
          </Button>
        </CardHeader>
        {showAdvanced && (
          <CardContent className="space-y-6">
            <PatternInput
              label="Include Patterns"
              placeholder="e.g., *.ts, src/**"
              patterns={includePatterns}
              onChange={setIncludePatterns}
            />

            <PatternInput
              label="Exclude Patterns"
              placeholder="e.g., node_modules, .git"
              patterns={excludePatterns}
              onChange={setExcludePatterns}
            />

            <BudgetSlider
              label="Token Budget"
              value={tokenBudget}
              onChange={setTokenBudget}
              min={1000}
              max={200000}
              step={1000}
            />
          </CardContent>
        )}
      </Card>

      <Button type="submit" className="w-full" size="lg" disabled={isPending}>
        {isPending ? (
          <>
            <Loader2 className="mr-2 h-4 w-4 animate-spin" />
            Processing...
          </>
        ) : (
          'Generate Context Pack'
        )}
      </Button>
    </form>
  );
}
