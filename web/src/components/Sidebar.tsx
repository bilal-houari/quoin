import { Config } from '../hooks/useConversion';

interface SidebarProps {
  config: Config;
  setConfig: (config: Config) => void;
  liveMode: boolean;
  setLiveMode: (live: boolean) => void;
}

export function Sidebar({ config, setConfig, liveMode, setLiveMode }: SidebarProps) {
  const updateConfig = (key: keyof Config, value: any) => {
    setConfig({ ...config, [key]: value });
  };

  return (
    <div className="w-64 border-l bg-background p-4 flex flex-col gap-6 overflow-y-auto shrink-0 animate-in slide-in-from-right duration-300">
      <section>
        <div className="flex items-center justify-between mb-3">
          <h3 className="text-xs font-bold uppercase tracking-widest text-muted-foreground">General</h3>
        </div>
        <label className="flex items-center justify-between group cursor-pointer">
          <span className="text-sm font-medium group-hover:text-primary transition-colors">Live Mode</span>
          <input
            type="checkbox"
            checked={liveMode}
            onChange={(e) => setLiveMode(e.target.checked)}
            className="rounded border-muted-foreground/30 text-primary focus:ring-primary h-4 w-4"
          />
        </label>
      </section>

      <section>
        <h3 className="text-xs font-bold uppercase tracking-widest text-muted-foreground mb-3">Layout Density</h3>
        <select
          value={config.density}
          onChange={(e) => updateConfig('density', e.target.value)}
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
          { label: 'Pretty Code', key: 'pretty_code' },
          { label: 'Section Numbering', key: 'section_numbering' },
          { label: 'Outline (TOC)', key: 'outline' }
        ].map(({ label, key }) => (
          <label key={key} className="flex items-center justify-between group cursor-pointer">
            <span className="text-sm font-medium group-hover:text-primary transition-colors">{label}</span>
            <input
              type="checkbox"
              checked={(config as any)[key]}
              onChange={(e) => updateConfig(key as keyof Config, e.target.checked)}
              className="rounded border-muted-foreground/30 text-primary focus:ring-primary h-4 w-4"
            />
          </label>
        ))}
      </section>
    </div>
  );
}
