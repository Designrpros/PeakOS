import { useState } from 'react';
import { motion } from 'framer-motion';
import { cn } from '../lib/utils';
import { useOSStore, InstalledApp } from '../stores/useOSStore';
import {
    Search, Download, Check, Shield, Zap, Globe,
    Music, Code, MessageCircle, PenTool, Video, Layout, Info, HardDrive, Sparkles, Terminal as TerminalIcon
} from 'lucide-react';

const CATEGORIES = ['All', 'Productivity', 'Media', 'Developer', 'Security', 'System'];

const AVAILABLE_APPS: InstalledApp[] = [
    { id: 'antigravity', name: 'Antigravity', icon: 'sparkles', type: 'native-app' },
    { id: 'firefox', name: 'Firefox', icon: 'globe', type: 'native-app' },
    { id: 'chrome', name: 'Google Chrome', icon: 'globe', type: 'native-app' },
    { id: 'vlc', name: 'VLC Media Player', icon: 'zap', type: 'native-app' },
    { id: 'vscode', name: 'VS Code', icon: 'code', type: 'native-app' },
    { id: 'wireshark', name: 'Wireshark', icon: 'shield', type: 'native-app' },
    { id: 'gimp', name: 'GIMP', icon: 'pen-tool', type: 'native-app' },
    { id: 'inkscape', name: 'Inkscape', icon: 'pen-tool', type: 'native-app' },
    { id: 'obs', name: 'OBS Studio', icon: 'video', type: 'native-app' },
    { id: 'discord', name: 'Discord', icon: 'message-circle', type: 'native-app' },
    { id: 'libreoffice', name: 'LibreOffice', icon: 'file-text', type: 'native-app' },
    { id: 'terminal', name: 'Peak Terminal', icon: 'terminal', type: 'system' },
    { id: 'youtube', name: 'YouTube', icon: 'video', type: 'web-app', url: 'https://www.youtube.com/embed' },
    { id: 'excalidraw', name: 'Excalidraw', icon: 'pen-tool', type: 'web-app', url: 'https://excalidraw.com' },
];

const ICON_MAP: Record<string, any> = {
    sparkles: Sparkles,
    globe: Globe,
    zap: Zap,
    code: Code,
    shield: Shield,
    'pen-tool': PenTool,
    video: Video,
    'message-circle': MessageCircle,
    terminal: TerminalIcon,
    'file-text': Layout,
    music: Music,
};

export function AppStore() {
    const { installApp, installedApps, addShortcut } = useOSStore();
    const [searchQuery, setSearchQuery] = useState('');
    const [selectedCategory, setSelectedCategory] = useState('All');

    const handleInstall = (app: InstalledApp) => {
        installApp(app);
        addShortcut({
            id: app.id,
            name: app.name,
            icon: app.icon,
            type: 'app',
            target: app.id,
            pinnedToDock: true,
            pinnedToSidebar: true
        });
    };

    const filteredApps = AVAILABLE_APPS.filter(app => {
        const matchesSearch = app.name.toLowerCase().includes(searchQuery.toLowerCase());
        const matchesCategory = selectedCategory === 'All' ||
            (selectedCategory === 'Productivity' && ['libreoffice', 'vscode', 'excalidraw'].includes(app.id)) ||
            (selectedCategory === 'Media' && ['vlc', 'obs', 'youtube'].includes(app.id)) ||
            (selectedCategory === 'Developer' && ['vscode', 'terminal', 'wireshark'].includes(app.id)) ||
            (selectedCategory === 'Security' && ['wireshark', 'antigravity'].includes(app.id)) ||
            (selectedCategory === 'System' && app.type === 'system');

        return matchesSearch && matchesCategory;
    });

    const isInstalled = (id: string) => (installedApps || []).some((app: InstalledApp) => app.id === id);

    return (
        <div className="flex flex-col h-full bg-zinc-50 dark:bg-zinc-950 text-foreground overflow-hidden">
            {/* Header / Search */}
            <div className="p-8 pb-4 border-b border-border/50 bg-white/50 dark:bg-zinc-900/50 backdrop-blur-md">
                <div className="flex flex-col md:flex-row md:items-center justify-between gap-6 mb-8">
                    <div>
                        <h1 className="text-4xl font-extrabold tracking-tight mb-2">Software Center</h1>
                        <p className="text-muted-foreground font-medium">Discover and refine your PeakOS experience.</p>
                    </div>
                    <div className="relative group max-w-md w-full">
                        <Search className="absolute left-4 top-1/2 -translate-y-1/2 text-muted-foreground group-focus-within:text-amber-500 transition-colors" size={20} />
                        <input
                            type="text"
                            placeholder="Explore applications..."
                            value={searchQuery}
                            onChange={(e) => setSearchQuery(e.target.value)}
                            className="w-full pl-12 pr-6 py-3 rounded-2xl bg-white dark:bg-zinc-800 border border-border/40 focus:border-amber-500/50 focus:outline-none focus:ring-4 focus:ring-amber-500/5 transition-all shadow-sm font-medium"
                        />
                    </div>
                </div>

                {/* Categories */}
                <div className="flex items-center gap-2 overflow-x-auto no-scrollbar pb-4">
                    {CATEGORIES.map(cat => (
                        <button
                            key={cat}
                            onClick={() => setSelectedCategory(cat)}
                            className={cn(
                                "px-4 py-2 rounded-xl text-sm font-bold transition-all whitespace-nowrap",
                                selectedCategory === cat
                                    ? "bg-amber-500 text-white shadow-lg shadow-amber-500/20 scale-105"
                                    : "bg-background/80 dark:bg-zinc-800/80 border border-border/40 hover:bg-zinc-100 dark:hover:bg-zinc-700 text-muted-foreground hover:text-foreground"
                            )}
                        >
                            {cat}
                        </button>
                    ))}
                </div>
            </div>

            {/* App Grid */}
            <div className="flex-1 overflow-y-auto p-8 custom-scrollbar pt-4">
                <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
                    {filteredApps.map((app) => {
                        const Icon = ICON_MAP[app.icon] || Globe;
                        const installed = isInstalled(app.id);

                        return (
                            <motion.div
                                key={app.id}
                                layout
                                initial={{ opacity: 0, y: 10 }}
                                animate={{ opacity: 1, y: 0 }}
                                className="group relative flex flex-col bg-white dark:bg-zinc-900 border border-border/40 rounded-3xl p-6 hover:shadow-2xl hover:shadow-black/5 dark:hover:shadow-white/5 transition-all duration-300"
                            >
                                <div className="flex items-start justify-between mb-6">
                                    <div className={cn(
                                        "w-14 h-14 rounded-2xl flex items-center justify-center text-white shadow-xl rotate-3 group-hover:rotate-0 transition-transform duration-500",
                                        app.id === 'antigravity' ? "bg-gradient-to-br from-blue-500 via-purple-500 to-pink-500" : "bg-gradient-to-br from-zinc-700 to-zinc-900 dark:from-zinc-100 dark:to-zinc-300 dark:text-zinc-900"
                                    )}>
                                        <Icon size={28} />
                                    </div>
                                    <div className="opacity-0 group-hover:opacity-100 transition-opacity">
                                        <Info size={18} className="text-muted-foreground" />
                                    </div>
                                </div>

                                <div className="mb-6 flex-1">
                                    <h3 className="font-bold text-lg mb-1">{app.name}</h3>
                                    <p className="text-xs text-muted-foreground font-medium uppercase tracking-wider">{app.type.replace('-', ' ')}</p>
                                </div>

                                <button
                                    onClick={() => !installed && handleInstall(app)}
                                    disabled={installed}
                                    className={cn(
                                        "w-full py-3 rounded-2xl text-sm font-bold transition-all flex items-center justify-center gap-2",
                                        installed
                                            ? "bg-green-500/10 text-green-500 cursor-default border border-green-500/20"
                                            : "bg-foreground/5 dark:bg-white/5 hover:bg-amber-500 hover:text-white border border-transparent shadow-sm"
                                    )}
                                >
                                    {installed ? (
                                        <><Check size={16} /> Ready</>
                                    ) : (
                                        <><Download size={16} /> Install</>
                                    )}
                                </button>
                            </motion.div>
                        );
                    })}
                </div>

                {filteredApps.length === 0 && (
                    <div className="flex flex-col items-center justify-center h-64 text-center">
                        <Search size={48} className="text-muted-foreground/20 mb-4" />
                        <h3 className="text-lg font-bold">No apps found</h3>
                        <p className="text-muted-foreground">Try searching for something else or browse categories.</p>
                    </div>
                )}
            </div>

            {/* Bottom Info */}
            <div className="p-4 bg-zinc-100 dark:bg-zinc-900 border-t border-border/30 flex items-center justify-between text-[11px] font-bold text-muted-foreground/60 uppercase tracking-widest">
                <div className="flex items-center gap-2">
                    <Shield size={12} />
                    <span>Peak Security Verified</span>
                </div>
                <div className="flex items-center gap-2">
                    <HardDrive size={12} />
                    <span>System Repository v3.2.1</span>
                </div>
            </div>
        </div>
    );
}
