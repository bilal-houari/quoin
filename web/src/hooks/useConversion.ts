import { useState, useCallback, useEffect } from 'react';

export interface Config {
  density: string;
  two_cols: boolean;
  latex_font: boolean;
  alt_table: boolean;
  pretty_code: boolean;
  section_numbering: boolean;
  outline: boolean;
}

export function useConversion(markdown: string, config: Config, liveMode: boolean) {
  const [pdfUrl, setPdfUrl] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const convert = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const response = await fetch('/api/convert', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ markdown, ...config }),
      });

      if (!response.ok) {
        throw new Error(`Conversion failed: ${await response.text()}`);
      }

      const blob = await response.blob();
      if (pdfUrl) URL.revokeObjectURL(pdfUrl);
      const url = URL.createObjectURL(blob);
      setPdfUrl(url);
      return url;
    } catch (err: any) {
      setError(err.message);
      return null;
    } finally {
      setIsLoading(false);
    }
  }, [markdown, config, pdfUrl]);

  const downloadTyp = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const response = await fetch('/api/convert/typ', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ markdown, ...config }),
      });

      if (!response.ok) {
        throw new Error(`Typst export failed: ${await response.text()}`);
      }

      const blob = await response.blob();
      const url = URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = 'document.typ';
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      URL.revokeObjectURL(url);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setIsLoading(false);
    }
  }, [markdown, config]);

  useEffect(() => {
    if (!liveMode) return;
    const timer = setTimeout(() => {
      convert();
    }, 1000);
    return () => clearTimeout(timer);
  }, [markdown, config, liveMode]);

  return { pdfUrl, isLoading, error, convert, downloadTyp };
}
