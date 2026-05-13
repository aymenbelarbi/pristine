import { IngestForm } from '@/components/forms/ingest-form';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Code2, Zap, Shield, GitBranch } from 'lucide-react';

const features = [
  {
    icon: Code2,
    title: 'Smart Context Extraction',
    description: 'Automatically identifies and extracts the most relevant files from any codebase.',
  },
  {
    icon: Zap,
    title: 'Token Budget Control',
    description: 'Set token limits to optimize for your LLM context window.',
  },
  {
    icon: Shield,
    title: 'Privacy First',
    description: 'Process locally or self-host. Your code never leaves your infrastructure.',
  },
  {
    icon: GitBranch,
    title: 'Git Native',
    description: 'Works with any Git repository. Supports branches, commits, and PRs.',
  },
];

export function HomePage() {
  return (
    <div className="max-w-5xl mx-auto space-y-12">
      <div className="text-center space-y-4">
        <h1 className="text-4xl font-bold tracking-tight">
          Code Context Compiler
        </h1>
        <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
          Generate optimized context artifacts for any codebase. Perfect for AI-assisted development.
        </p>
      </div>

      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
        {features.map((feature) => {
          const Icon = feature.icon;
          return (
            <Card key={feature.title}>
              <CardHeader>
                <Icon className="h-8 w-8 text-primary mb-2" />
                <CardTitle className="text-base">{feature.title}</CardTitle>
              </CardHeader>
              <CardContent>
                <CardDescription>{feature.description}</CardDescription>
              </CardContent>
            </Card>
          );
        })}
      </div>

      <div className="space-y-4">
        <h2 className="text-2xl font-semibold text-center">Get Started</h2>
        <IngestForm />
      </div>
    </div>
  );
}
