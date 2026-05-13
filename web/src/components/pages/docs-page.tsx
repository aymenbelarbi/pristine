import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Badge } from '@/components/ui/badge';
import { CodeBlock } from '@/components/display/code-block';

const apiEndpoints = [
  {
    method: 'POST',
    path: '/api/v1/ingest',
    description: 'Submit a repository for processing',
    request: `{
  "url": "https://github.com/user/repo",
  "branch": "main",
  "pack_type": "overview",
  "token_budget": 50000,
  "include_patterns": ["*.ts", "src/**"],
  "exclude_patterns": ["node_modules", ".git"]
}`,
    response: `{
  "job_id": "abc123",
  "status": "pending",
  "message": "Repository queued for processing"
}`,
  },
  {
    method: 'GET',
    path: '/api/v1/artifacts/:job_id',
    description: 'Get artifact by job ID',
    request: null,
    response: `{
  "id": "abc123",
  "job_id": "abc123",
  "status": "completed",
  "url": "https://github.com/user/repo",
  "files": [...],
  "total_tokens": 45000,
  "total_files": 25,
  "created_at": "2024-01-01T00:00:00Z"
}`,
  },
  {
    method: 'GET',
    path: '/api/v1/artifacts/:job_id/status',
    description: 'Get processing status',
    request: null,
    response: `{
  "status": "processing",
  "progress": 75
}`,
  },
  {
    method: 'GET',
    path: '/api/v1/health',
    description: 'Health check endpoint',
    request: null,
    response: `{
  "status": "healthy",
  "version": "0.1.0",
  "uptime": 3600
}`,
  },
];

const methodColors: Record<string, string> = {
  GET: 'bg-green-500/10 text-green-500',
  POST: 'bg-blue-500/10 text-blue-500',
  PUT: 'bg-yellow-500/10 text-yellow-500',
  DELETE: 'bg-red-500/10 text-red-500',
};

export function DocsPage() {
  return (
    <div className="max-w-4xl mx-auto space-y-8">
      <div>
        <h1 className="text-3xl font-bold">Documentation</h1>
        <p className="text-muted-foreground mt-2">
          Learn how to use the Pristine API to generate context packs for your codebases.
        </p>
      </div>

      <Tabs defaultValue="api" className="space-y-6">
        <TabsList>
          <TabsTrigger value="api">API Reference</TabsTrigger>
          <TabsTrigger value="cli">CLI Usage</TabsTrigger>
          <TabsTrigger value="config">Configuration</TabsTrigger>
        </TabsList>

        <TabsContent value="api" className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>Base URL</CardTitle>
              <CardDescription>
                All API requests should be made to the following base URL:
              </CardDescription>
            </CardHeader>
            <CardContent>
              <CodeBlock code="http://localhost:8080/api/v1" language="text" />
            </CardContent>
          </Card>

          <div className="space-y-4">
            <h2 className="text-xl font-semibold">Endpoints</h2>
            {apiEndpoints.map((endpoint) => (
              <Card key={endpoint.path}>
                <CardHeader>
                  <div className="flex items-center gap-2">
                    <Badge className={methodColors[endpoint.method]}>
                      {endpoint.method}
                    </Badge>
                    <code className="text-sm font-mono">{endpoint.path}</code>
                  </div>
                  <CardDescription>{endpoint.description}</CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                  {endpoint.request && (
                    <div>
                      <h4 className="text-sm font-medium mb-2">Request Body</h4>
                      <CodeBlock code={endpoint.request} language="json" />
                    </div>
                  )}
                  <div>
                    <h4 className="text-sm font-medium mb-2">Response</h4>
                    <CodeBlock code={endpoint.response} language="json" />
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        </TabsContent>

        <TabsContent value="cli" className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>Installation</CardTitle>
            </CardHeader>
            <CardContent>
              <CodeBlock
                code="cargo install pristine-cli"
                language="bash"
              />
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Basic Usage</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div>
                <h4 className="text-sm font-medium mb-2">Generate overview pack</h4>
                <CodeBlock
                  code="pristine ingest https://github.com/user/repo"
                  language="bash"
                />
              </div>
              <div>
                <h4 className="text-sm font-medium mb-2">With options</h4>
                <CodeBlock
                  code={`pristine ingest https://github.com/user/repo \\
  --branch main \\
  --pack-type overview \\
  --token-budget 50000 \\
  --include "*.ts" \\
  --exclude "node_modules"`}
                  language="bash"
                />
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="config" className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>Configuration File</CardTitle>
              <CardDescription>
                Create a <code>pristine.toml</code> file in your project root:
              </CardDescription>
            </CardHeader>
            <CardContent>
              <CodeBlock
                code={`[defaults]
pack_type = "overview"
token_budget = 50000
max_file_size = 1048576

[patterns]
include = ["*.ts", "*.js", "src/**"]
exclude = ["node_modules", ".git", "dist", "build"]

[output]
format = "json"
directory = "./.pristine"`}
                language="toml"
                filename="pristine.toml"
              />
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  );
}
