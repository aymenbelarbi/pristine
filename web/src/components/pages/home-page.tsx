export function HomePage() {
  return (
    <div className="max-w-4xl mx-auto">
      <div className="text-center mb-12">
        <h1 className="text-4xl font-bold mb-4">Code Context Compiler</h1>
        <p className="text-xl text-muted-foreground">
          Generate optimized context artifacts for any codebase.
        </p>
      </div>
      
      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        <div className="p-6 border rounded-lg">
          <h3 className="font-semibold mb-2">Overview Pack</h3>
          <p className="text-sm text-muted-foreground">
            Understand a codebase quickly with architecture, entrypoints, and key files.
          </p>
        </div>
        <div className="p-6 border rounded-lg">
          <h3 className="font-semibold mb-2">Task Pack</h3>
          <p className="text-sm text-muted-foreground">
            Get focused context for a specific task or query.
          </p>
        </div>
        <div className="p-6 border rounded-lg">
          <h3 className="font-semibold mb-2">Review Pack</h3>
          <p className="text-sm text-muted-foreground">
            Review a PR or diff with impacted files and tests.
          </p>
        </div>
      </div>
    </div>
  );
}
