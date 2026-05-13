export function Footer() {
  return (
    <footer className="border-t py-6">
      <div className="container mx-auto px-4 text-center text-sm text-muted-foreground">
        <p>Pristine - Code Context Compiler</p>
        <p className="mt-1">
          Built with Rust. Licensed under{' '}
          <a href="https://opensource.org/licenses/MIT" className="underline">
            MIT
          </a>
          .
        </p>
      </div>
    </footer>
  );
}
