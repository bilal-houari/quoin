import { useState, useEffect } from 'react';
import { useConversion, Config } from './hooks/useConversion';
import { Header } from './components/Header';
import { Editor } from './components/Editor';
import { Previewer } from './components/Previewer';
import { Sidebar } from './components/Sidebar';

function App() {
  const [markdown, setMarkdown] = useState('# Hello Quoin\n\nEdit this to see live preview!');
  const [config, setConfig] = useState<Config>({
    density: 'standard',
    two_cols: false,
    latex_font: false,
    alt_table: true,
    pretty_code: true
  });
  const [liveMode, setLiveMode] = useState(true);
  const [isDarkMode, setIsDarkMode] = useState(false);
  const [showSidebar, setShowSidebar] = useState(true);

  const { pdfUrl, isLoading, error, convert } = useConversion(markdown, config, liveMode);

  // Sync theme with document root
  useEffect(() => {
    if (isDarkMode) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }, [isDarkMode]);

  const handleDownload = () => {
    if (!pdfUrl) return;
    const link = document.createElement('a');
    link.href = pdfUrl;
    link.download = 'quoin-document.pdf';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
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
