import { create } from 'zustand';

interface UIState {
  theme: 'light' | 'dark' | 'system';
  sidebarOpen: boolean;
  commandOpen: boolean;
  setTheme: (theme: 'light' | 'dark' | 'system') => void;
  toggleTheme: () => void;
  setSidebarOpen: (open: boolean) => void;
  toggleSidebar: () => void;
  setCommandOpen: (open: boolean) => void;
}

export const useUIStore = create<UIState>((set) => ({
  theme: (localStorage.getItem('theme') as 'light' | 'dark' | 'system') || 'system',
  sidebarOpen: true,
  commandOpen: false,
  setTheme: (theme) => {
    localStorage.setItem('theme', theme);
    set({ theme });
    applyTheme(theme);
  },
  toggleTheme: () =>
    set((state) => {
      const newTheme = state.theme === 'light' ? 'dark' : 'light';
      localStorage.setItem('theme', newTheme);
      applyTheme(newTheme);
      return { theme: newTheme };
    }),
  setSidebarOpen: (open) => set({ sidebarOpen: open }),
  toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),
  setCommandOpen: (open) => set({ commandOpen: open }),
}));

function applyTheme(theme: 'light' | 'dark' | 'system') {
  const root = document.documentElement;
  if (theme === 'system') {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    root.classList.toggle('dark', prefersDark);
  } else {
    root.classList.toggle('dark', theme === 'dark');
  }
}

// Initialize theme on load
if (typeof window !== 'undefined') {
  const stored = localStorage.getItem('theme') as 'light' | 'dark' | 'system' | null;
  applyTheme(stored || 'system');
}
