import { AlertCircle } from 'lucide-react';

interface EditorProps {
  value: string;
  onChange: (value: string) => void;
  error: string | null;
}

export function Editor({ value, onChange, error }: EditorProps) {
  return (
    <div className="flex-1 flex flex-col bg-background">
      <div className="h-9 border-b flex items-center px-4 bg-muted/20 justify-between shrink-0">
        <span className="text-[10px] font-bold uppercase tracking-widest text-muted-foreground">Markdown Editor</span>
        {error && (
          <div className="flex items-center gap-1.5 text-destructive text-[10px] font-medium animate-in fade-in slide-in-from-top-1">
            <AlertCircle className="w-3 h-3" />
            {error}
          </div>
        )}
      </div>
      <textarea
        value={value}
        onChange={(e) => onChange(e.target.value)}
        className="flex-1 p-6 resize-none focus:outline-none font-mono text-sm leading-relaxed bg-transparent"
        placeholder="Type your markdown here..."
      />
    </div>
  );
}
