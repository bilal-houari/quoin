import { FileText, Play, Download, Moon, Sun, Settings, Loader2 } from 'lucide-react';

interface HeaderProps {
  isLoading: boolean;
  isDarkMode: boolean;
  toggleDarkMode: () => void;
  toggleSidebar: () => void;
  onCompile: () => void;
  onDownload: () => void;
}

export function Header({
  isLoading,
  isDarkMode,
  toggleDarkMode,
  toggleSidebar,
  onCompile,
  onDownload
}: HeaderProps) {
  return (
    <header className="h-14 border-b flex items-center justify-between px-4 bg-background z-20 shadow-sm">
      <div className="flex items-center gap-2">
        <div className="w-8 h-8 bg-black dark:bg-white rounded-md flex items-center justify-center">
          <FileText className="text-white dark:text-black w-5 h-5" />
        </div>
        <span className="font-bold text-xl tracking-tight uppercase">Quoin</span>
      </div>

      <div className="flex items-center gap-2">
        <button
          onClick={onCompile}
          disabled={isLoading}
          className="flex items-center gap-2 px-3 py-1.5 bg-primary text-primary-foreground rounded-md text-sm font-medium hover:opacity-90 disabled:opacity-50 transition-all shadow-sm"
        >
          {isLoading ? <Loader2 className="w-4 h-4 animate-spin" /> : <Play className="w-4 h-4" />}
          Compile
        </button>
        <button
          onClick={onDownload}
          title="Download PDF"
          className="p-2 hover:bg-muted rounded-md transition-colors text-muted-foreground"
        >
          <Download className="w-5 h-5" />
        </button>
        <div className="w-px h-6 bg-border mx-2" />
        <button
          onClick={toggleDarkMode}
          title={isDarkMode ? "Light Mode" : "Dark Mode"}
          className="p-2 hover:bg-muted rounded-md transition-colors text-muted-foreground"
        >
          {isDarkMode ? <Sun className="w-5 h-5" /> : <Moon className="w-5 h-5" />}
        </button>
        <button
          onClick={toggleSidebar}
          title="Toggle Sidebar"
          className="p-2 hover:bg-muted rounded-md transition-colors text-muted-foreground"
        >
          <Settings className="w-5 h-5" />
        </button>
      </div>
    </header>
  );
}
