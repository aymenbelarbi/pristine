import { Link } from 'react-router-dom';
import { Github, Moon, Sun } from 'lucide-react';
import { Button } from '../ui/button';

export function Header() {
  return (
    <header className="border-b">
      <div className="container mx-auto px-4 h-16 flex items-center justify-between">
        <Link to="/" className="flex items-center gap-2">
          <span className="text-xl font-bold">Pristine</span>
        </Link>
        <nav className="flex items-center gap-4">
          <Link to="/docs" className="text-sm text-muted-foreground hover:text-foreground">
            Docs
          </Link>
          <Link to="/settings" className="text-sm text-muted-foreground hover:text-foreground">
            Settings
          </Link>
          <Button variant="ghost" size="icon">
            <Sun className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="icon" asChild>
            <a href="https://github.com/aymenbelarbi/pristine" target="_blank" rel="noopener noreferrer">
              <Github className="h-4 w-4" />
            </a>
          </Button>
        </nav>
      </div>
    </header>
  );
}
