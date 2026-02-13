import { FileText, Play, Download, Moon, Sun, Settings, Loader2 } from 'lucide-react';

interface HeaderProps {
  title: string;
  onTitleChange: (title: string) => void;
  isLoading: boolean;
  isDarkMode: boolean;
  toggleDarkMode: () => void;
  toggleSidebar: () => void;
  onCompile: () => void;
  onDownload: () => void;
  onDownloadTyp: () => void;
  onDownloadMd: () => void;
}

export function Header({
  title,
  onTitleChange,
  isLoading,
  isDarkMode,
  toggleDarkMode,
  toggleSidebar,
  onCompile,
  onDownload,
  onDownloadTyp,
  onDownloadMd
}: HeaderProps) {
  return (
    <header className="h-14 border-b flex items-center justify-between px-4 bg-background z-20 shadow-sm">
      <div className="flex items-center gap-4">
        <div className="flex items-center gap-2">
          <div className="w-8 h-8 bg-black dark:bg-white rounded-md flex items-center justify-center shadow-inner">
            <FileText className="text-white dark:text-black w-5 h-5" />
          </div>
          <span className="font-bold text-xl tracking-tight uppercase hidden sm:inline">Quoin</span>
        </div>

        <div className="h-6 w-px bg-border mx-1" />

        <div className="relative group flex items-center">
          <input
            type="text"
            value={title}
            onChange={(e) => onTitleChange(e.target.value)}
            className="bg-transparent border-none focus:ring-0 text-sm font-medium px-2 py-1 rounded hover:bg-muted/50 transition-colors w-48 md:w-64 placeholder:italic"
            placeholder="Untitled Document"
          />
          <div className="absolute bottom-0 left-2 right-2 h-0.5 bg-primary scale-x-0 group-focus-within:scale-x-100 transition-transform origin-left rounded-full" />
        </div>
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
        <button
          onClick={onDownloadTyp}
          title="Export Typst Source"
          className="flex items-center gap-1.5 px-2 py-1.5 hover:bg-muted rounded-md transition-colors text-muted-foreground text-[10px] font-bold uppercase tracking-wider"
        >
          <FileText className="w-4 h-4" />
          .typ
        </button>
        <button
          onClick={onDownloadMd}
          title="Download Markdown"
          className="flex items-center gap-1.5 px-2 py-1.5 hover:bg-muted rounded-md transition-colors text-muted-foreground text-[10px] font-bold uppercase tracking-wider"
        >
          <FileText className="w-4 h-4" />
          .md
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
