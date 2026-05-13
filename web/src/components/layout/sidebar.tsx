import { Link, useLocation } from 'react-router-dom';
import { Home, FileText, Settings, BookOpen, History } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useUIStore } from '@/stores/uiStore';
import { Button } from '@/components/ui/button';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Separator } from '@/components/ui/separator';

const navItems = [
  { href: '/', label: 'Home', icon: Home },
  { href: '/history', label: 'History', icon: History },
  { href: '/docs', label: 'Documentation', icon: BookOpen },
  { href: '/settings', label: 'Settings', icon: Settings },
];

export function Sidebar() {
  const location = useLocation();
  const { sidebarOpen, setSidebarOpen } = useUIStore();

  return (
    <>
      {sidebarOpen && (
        <div
          className="fixed inset-0 z-30 bg-black/50 md:hidden"
          onClick={() => setSidebarOpen(false)}
        />
      )}
      <aside
        className={cn(
          'fixed left-0 top-16 z-30 h-[calc(100vh-4rem)] w-64 border-r bg-background transition-transform duration-200 md:translate-x-0',
          sidebarOpen ? 'translate-x-0' : '-translate-x-full'
        )}
      >
        <ScrollArea className="h-full py-4">
          <nav className="space-y-1 px-3">
            {navItems.map((item) => {
              const Icon = item.icon;
              const isActive = location.pathname === item.href;
              return (
                <Button
                  key={item.href}
                  variant={isActive ? 'secondary' : 'ghost'}
                  className="w-full justify-start"
                  asChild
                  onClick={() => setSidebarOpen(false)}
                >
                  <Link to={item.href}>
                    <Icon className="mr-2 h-4 w-4" />
                    {item.label}
                  </Link>
                </Button>
              );
            })}
          </nav>
          <Separator className="my-4" />
          <div className="px-3">
            <h4 className="mb-2 px-4 text-sm font-semibold">Recent</h4>
            <div className="space-y-1">
              <Button variant="ghost" className="w-full justify-start text-muted-foreground" disabled>
                <FileText className="mr-2 h-4 w-4" />
                No recent items
              </Button>
            </div>
          </div>
        </ScrollArea>
      </aside>
    </>
  );
}
