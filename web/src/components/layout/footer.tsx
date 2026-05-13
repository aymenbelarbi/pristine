import { Github } from 'lucide-react';

export function Footer() {
  return (
    <footer className="border-t py-6 md:py-0">
      <div className="container mx-auto px-4 flex flex-col items-center justify-between gap-4 md:h-24 md:flex-row">
        <div className="flex flex-col items-center gap-4 px-8 md:flex-row md:gap-2 md:px-0">
          <svg
            viewBox="0 0 24 24"
            className="h-6 w-6"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
          >
            <path d="M12 2L2 7l10 5 10-5-10-5z" />
            <path d="M2 17l10 5 10-5" />
            <path d="M2 12l10 5 10-5" />
          </svg>
          <p className="text-center text-sm leading-loose text-muted-foreground md:text-left">
            Built with Rust. Licensed under{' '}
            <a
              href="https://opensource.org/licenses/MIT"
              target="_blank"
              rel="noopener noreferrer"
              className="font-medium underline underline-offset-4"
            >
              MIT
            </a>
            .
          </p>
        </div>
        <div className="flex items-center gap-4">
          <a
            href="https://github.com/aymenbelarbi/pristine"
            target="_blank"
            rel="noopener noreferrer"
            className="text-muted-foreground hover:text-foreground"
          >
            <Github className="h-4 w-4" />
            <span className="sr-only">GitHub</span>
          </a>
        </div>
      </div>
    </footer>
  );
}
