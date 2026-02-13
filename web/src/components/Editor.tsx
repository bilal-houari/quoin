import { useRef, useEffect, useState, useCallback } from 'react';
import { AlertCircle } from 'lucide-react';

interface EditorProps {
  value: string;
  onChange: (value: string) => void;
  error: string | null;
}

export function Editor({ value, onChange, error }: EditorProps) {
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const gutterRef = useRef<HTMLDivElement>(null);
  const [lineCount, setLineCount] = useState(1);

  useEffect(() => {
    const lines = value.split('\n').length;
    setLineCount(lines);
  }, [value]);

  const handleScroll = useCallback(() => {
    if (textareaRef.current && gutterRef.current) {
      gutterRef.current.scrollTop = textareaRef.current.scrollTop;
    }
  }, []);

  return (
    <div className="flex-1 flex flex-col bg-background relative border-r overflow-hidden">
      <div className="h-9 border-b flex items-center px-4 bg-muted/20 justify-between shrink-0">
        <span className="text-[10px] font-bold uppercase tracking-widest text-muted-foreground">Markdown Editor</span>
        {error && (
          <div className="flex items-center gap-1.5 text-destructive text-[10px] font-medium animate-in fade-in slide-in-from-top-1">
            <AlertCircle className="w-3 h-3" />
            {error}
          </div>
        )}
      </div>

      <div className="flex-1 flex overflow-hidden group">
        {/* Line Numbers Gutter */}
        <div
          ref={gutterRef}
          className="w-12 bg-muted/10 border-r flex flex-col items-end py-6 pr-3 select-none overflow-hidden shrink-0 text-muted-foreground/30 font-mono text-[12px] leading-[22.4px]"
        >
          {Array.from({ length: lineCount }).map((_, i) => (
            <div key={i} className="h-[22.4px] flex items-center">
              {i + 1}
            </div>
          ))}
        </div>

        {/* Textarea */}
        <textarea
          ref={textareaRef}
          value={value}
          onChange={(e) => onChange(e.target.value)}
          onScroll={handleScroll}
          className="flex-1 p-6 resize-none focus:outline-none font-mono text-[14px] leading-[1.6] bg-transparent whitespace-pre overflow-y-auto scrollbar-thin scrollbar-thumb-muted-foreground/20 hover:scrollbar-thumb-muted-foreground/30 transition-shadow"
          placeholder="Type your markdown here..."
          spellCheck={false}
        />
      </div>
    </div>
  );
}
