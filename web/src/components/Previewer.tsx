import { Loader2 } from 'lucide-react';

interface PreviewerProps {
  pdfUrl: string | null;
  isLoading: boolean;
}

export function Previewer({ pdfUrl, isLoading }: PreviewerProps) {
  return (
    <div className="flex-1 bg-muted/40 flex flex-col relative">
      <div className="h-9 border-b flex items-center px-4 bg-muted/20 shrink-0">
        <span className="text-[10px] font-bold uppercase tracking-widest text-muted-foreground">PDF Preview</span>
      </div>
      <div className="flex-1 flex items-center justify-center p-8 overflow-hidden">
        {pdfUrl ? (
          <iframe
            src={`${pdfUrl}#toolbar=0&navpanes=0`}
            className="w-full h-full border rounded-md shadow-2xl bg-white"
          />
        ) : (
          <div className="text-center space-y-4">
            <Loader2 className="w-10 h-10 animate-spin mx-auto text-muted-foreground/30" />
            <p className="text-sm text-muted-foreground font-medium">Preparing your workspace...</p>
          </div>
        )}
      </div>
      {isLoading && pdfUrl && (
        <div className="absolute top-11 right-4 bg-background/80 backdrop-blur-sm border rounded-full px-3 py-1 flex items-center gap-2 shadow-sm animate-in fade-in zoom-in-95">
          <Loader2 className="w-3 h-3 animate-spin text-primary" />
          <span className="text-[10px] font-medium">Updating...</span>
        </div>
      )}
    </div>
  );
}
