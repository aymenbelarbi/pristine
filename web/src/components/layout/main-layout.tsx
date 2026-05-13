import { ReactNode } from 'react';
import { Header } from './header';
import { Footer } from './footer';
import { Sidebar } from './sidebar';
import { useUIStore } from '@/stores/uiStore';
import { cn } from '@/lib/utils';

interface MainLayoutProps {
  children: ReactNode;
}

export function MainLayout({ children }: MainLayoutProps) {
  const { sidebarOpen } = useUIStore();

  return (
    <div className="min-h-screen flex flex-col">
      <Header />
      <Sidebar />
      <div
        className={cn(
          'flex-1 transition-all duration-200',
          sidebarOpen ? 'md:ml-64' : 'md:ml-0'
        )}
      >
        <main className="container mx-auto px-4 py-8 min-h-[calc(100vh-8rem)]">
          {children}
        </main>
        <Footer />
      </div>
    </div>
  );
}
