import { useState, useEffect } from 'react';
import { useConversion, Config } from './hooks/useConversion';
import { Header } from './components/Header';
import { Editor } from './components/Editor';
import { Previewer } from './components/Previewer';
import { Sidebar } from './components/Sidebar';

function App() {
  const [markdown, setMarkdown] = useState<string>(() => {
    return localStorage.getItem('quoin-markdown') || '# Hello Quoin\n\nEdit this to see live preview!';
  });
  const [config, setConfig] = useState<Config>(() => {
    const saved = localStorage.getItem('quoin-config');
    const defaultConfig: Config = {
      density: 'standard',
      two_cols: false,
      latex_font: false,
      alt_table: true,
      pretty_code: true,
      section_numbering: false
    };
    return saved ? JSON.parse(saved) : defaultConfig;
  });
  const [liveMode, setLiveMode] = useState(true);
  const [isDarkMode, setIsDarkMode] = useState(() => {
    return localStorage.getItem('quoin-theme') === 'dark';
  });
  const [showSidebar, setShowSidebar] = useState(true);

  const { pdfUrl, isLoading, error, convert, downloadTyp } = useConversion(markdown, config, liveMode);

  // Sync theme with document root and localStorage
  useEffect(() => {
    if (isDarkMode) {
      document.documentElement.classList.add('dark');
      localStorage.setItem('quoin-theme', 'dark');
    } else {
      document.documentElement.classList.remove('dark');
      localStorage.setItem('quoin-theme', 'light');
    }
  }, [isDarkMode]);

  // Persist markdown and config
  useEffect(() => {
    localStorage.setItem('quoin-markdown', markdown);
  }, [markdown]);

  useEffect(() => {
    localStorage.setItem('quoin-config', JSON.stringify(config));
  }, [config]);

  const handleDownload = () => {
    if (!pdfUrl) return;
    const link = document.createElement('a');
    link.href = pdfUrl;
    link.download = 'quoin-document.pdf';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  const handleDownloadMd = () => {
    const blob = new Blob([markdown], { type: 'text/markdown' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = 'document.md';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  };

  return (
    <div className="h-screen flex flex-col bg-background text-foreground transition-colors duration-300">
      <Header
        isLoading={isLoading}
        isDarkMode={isDarkMode}
        toggleDarkMode={() => setIsDarkMode(!isDarkMode)}
        toggleSidebar={() => setShowSidebar(!showSidebar)}
        onCompile={convert}
        onDownload={handleDownload}
        onDownloadTyp={downloadTyp}
        onDownloadMd={handleDownloadMd}
      />

      <main className="flex-1 flex overflow-hidden">
        <Editor
          value={markdown}
          onChange={setMarkdown}
          error={error}
        />

        <Previewer
          pdfUrl={pdfUrl}
          isLoading={isLoading}
        />

        {showSidebar && (
          <Sidebar
            config={config}
            setConfig={setConfig}
            liveMode={liveMode}
            setLiveMode={setLiveMode}
          />
        )}
      </main>
    </div>
  );
}

export default App;
