import { useState, useEffect, useCallback } from 'react'
import { FileText, Play, Settings, Download, Moon, Sun, Loader2, AlertCircle } from 'lucide-react'

interface Config {
  density: string
  two_cols: boolean
  latex_font: boolean
  alt_table: boolean
  pretty_code: boolean
}

function App() {
  const [markdown, setMarkdown] = useState('# Hello Quoin\n\nEdit this to see live preview!')
  const [config, setConfig] = useState<Config>({
    density: 'standard',
    two_cols: false,
    latex_font: false,
    alt_table: true,
    pretty_code: true
  })
  const [pdfUrl, setPdfUrl] = useState<string | null>(null)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [isDarkMode, setIsDarkMode] = useState(false)

  const handleConvert = useCallback(async () => {
    setIsLoading(true)
    setError(null)
    try {
      const response = await fetch('/api/convert', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          markdown,
          ...config
        })
      })

      if (!response.ok) {
        throw new Error(`Conversion failed: ${await response.text()}`)
      }

      const blob = await response.blob()
      if (pdfUrl) URL.revokeObjectURL(pdfUrl)
      setPdfUrl(URL.createObjectURL(blob))
    } catch (err: any) {
      setError(err.message)
    } finally {
      setIsLoading(false)
    }
  }, [markdown, config, pdfUrl])

  // Simple debounce for live preview
  useEffect(() => {
    const timer = setTimeout(() => {
      handleConvert()
    }, 1000)
    return () => clearTimeout(timer)
  }, [markdown, config])

  return (
    <div className={`h-screen flex flex-col ${isDarkMode ? 'dark' : ''}`}>
      <header className="h-14 border-b flex items-center justify-between px-4 bg-background z-20 shadow-sm">
        <div className="flex items-center gap-2">
          <div className="w-8 h-8 bg-black dark:bg-white rounded-md flex items-center justify-center">
            <FileText className="text-white dark:text-black w-5 h-5" />
          </div>
          <span className="font-bold text-xl tracking-tight uppercase">Quoin</span>
        </div>

        <div className="flex items-center gap-2">
          <button
            onClick={handleConvert}
            disabled={isLoading}
            className="flex items-center gap-2 px-3 py-1.5 bg-primary text-primary-foreground rounded-md text-sm font-medium hover:opacity-90 disabled:opacity-50 transition-all shadow-sm"
          >
            {isLoading ? <Loader2 className="w-4 h-4 animate-spin" /> : <Play className="w-4 h-4" />}
            Compile
          </button>
          <button className="p-2 hover:bg-muted rounded-md transition-colors text-muted-foreground">
            <Download className="w-5 h-5" />
          </button>
          <div className="w-px h-6 bg-border mx-2" />
          <button
            onClick={() => setIsDarkMode(!isDarkMode)}
            className="p-2 hover:bg-muted rounded-md transition-colors text-muted-foreground"
          >
            {isDarkMode ? <Sun className="w-5 h-5" /> : <Moon className="w-5 h-5" />}
          </button>
          <button className="p-2 hover:bg-muted rounded-md transition-colors text-muted-foreground">
            <Settings className="w-5 h-5" />
          </button>
        </div>
      </header>

      <main className="flex-1 flex overflow-hidden">
        {/* Editor Side */}
        <div className="w-1/2 flex flex-col border-r bg-background">
          <div className="h-9 border-b flex items-center px-4 bg-muted/20 justify-between">
            <span className="text-[10px] font-bold uppercase tracking-widest text-muted-foreground">Markdown Editor</span>
            {error && (
              <div className="flex items-center gap-1.5 text-destructive text-[10px] font-medium">
                <AlertCircle className="w-3 h-3" />
                {error}
              </div>
            )}
          </div>
          <textarea
            value={markdown}
            onChange={(e) => setMarkdown(e.target.value)}
            className="flex-1 p-6 resize-none focus:outline-none font-mono text-sm leading-relaxed bg-transparent"
            placeholder="Type your markdown here..."
          />
        </div>

        {/* Preview Side */}
        <div className="w-1/2 bg-muted/40 flex flex-col">
          <div className="h-9 border-b flex items-center px-4 bg-muted/20">
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
        </div>

        {/* Mini Sidebar for Quick Config */}
        <div className="w-64 border-l bg-background p-4 flex flex-col gap-6">
          <section>
            <h3 className="text-xs font-bold uppercase tracking-widest text-muted-foreground mb-3">Layout Density</h3>
            <select
              value={config.density}
              onChange={(e) => setConfig({ ...config, density: e.target.value })}
              className="w-full bg-muted/50 border rounded-md p-2 text-sm focus:outline-none focus:ring-1 focus:ring-primary"
            >
              <option value="ultra-dense">Ultra-Dense (8pt)</option>
              <option value="dense">Dense (10pt)</option>
              <option value="standard">Standard (10pt)</option>
              <option value="comfort">Comfort (12pt)</option>
            </select>
          </section>

          <section className="space-y-4">
            <h3 className="text-xs font-bold uppercase tracking-widest text-muted-foreground mb-1">Display Options</h3>

            {[
              { label: 'Two Columns', key: 'two_cols' },
              { label: 'LaTeX Font', key: 'latex_font' },
              { label: 'Alt Tables', key: 'alt_table' },
              { label: 'Pretty Code', key: 'pretty_code' }
            ].map(({ label, key }) => (
              <label key={key} className="flex items-center justify-between group cursor-pointer">
                <span className="text-sm font-medium group-hover:text-primary transition-colors">{label}</span>
                <input
                  type="checkbox"
                  checked={(config as any)[key]}
                  onChange={(e) => setConfig({ ...config, [key]: e.target.checked })}
                  className="rounded border-muted-foreground/30 text-primary focus:ring-primary h-4 w-4"
                />
              </label>
            ))}
          </section>
        </div>
      </main>
    </div>
  )
}

export default App
