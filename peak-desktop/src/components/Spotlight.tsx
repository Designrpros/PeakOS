import { useState, useEffect, useRef, useMemo } from 'react';
import { Search, Command, Globe, AppWindow, ArrowRight, FileText, Folder } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';
import { useOSStore } from '../stores/useOSStore';
import { useSystemLink } from '../hooks/useSystemLink';
import { cn } from '../lib/utils';

export function Spotlight() {
    const { isSpotlightOpen, toggleSpotlight, installedApps, shortcuts, launchApp } = useOSStore();
    const { callTool } = useSystemLink();
    const [query, setQuery] = useState('');
    const [backendResults, setBackendResults] = useState<any[]>([]);
    const [selectedIndex, setSelectedIndex] = useState(0);
    const [isLoading, setIsLoading] = useState(false);
    const inputRef = useRef<HTMLInputElement>(null);

    // Core logic: Search apps and shortcuts Locally
    const localResults = useMemo(() => {
        const trimmed = query.trim().toLowerCase();
        if (!trimmed) return [];

        return [
            ...installedApps.map(app => ({
                id: app.id,
                name: app.name,
                type: 'app',
                icon: AppWindow,
                category: 'Applications'
            })),
            ...shortcuts.map(s => ({
                id: s.id,
                name: s.name,
                type: s.type,
                icon: s.type === 'url' ? Globe : AppWindow,
                category: 'Shortcuts'
            }))
        ].filter(item =>
            item.name.toLowerCase().includes(trimmed)
        );
    }, [query, installedApps, shortcuts]);

    // Backend Search (Debounced)
    useEffect(() => {
        const trimmed = query.trim();
        if (trimmed.length < 2) {
            setBackendResults([]);
            return;
        }

        const timeout = setTimeout(async () => {
            setIsLoading(true);
            try {
                // Search from home dir or root for demo
                const response = await callTool('search_files', {
                    query: trimmed,
                    base_path: '/' // Simplified, usually would be user home
                });

                if (response && Array.isArray(response)) {
                    setBackendResults(response.map((f: any) => ({
                        id: f.path,
                        name: f.name,
                        type: f.is_dir ? 'directory' : 'file',
                        icon: f.is_dir ? Folder : FileText,
                        category: f.is_dir ? 'Folders' : 'Files',
                        path: f.path
                    })));
                }
            } catch (e) {
                console.error("Spotlight backend search failed:", e);
            } finally {
                setIsLoading(false);
            }
        }, 300);

        return () => clearTimeout(timeout);
    }, [query, callTool]);

    const results = [...localResults, ...backendResults].slice(0, 8);

    useEffect(() => {
        if (isSpotlightOpen) {
            setQuery('');
            setBackendResults([]);
            setSelectedIndex(0);
            setTimeout(() => inputRef.current?.focus(), 10);
        }
    }, [isSpotlightOpen]);

    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if (!isSpotlightOpen) {
                if ((e.metaKey || e.ctrlKey) && e.code === 'Space') {
                    e.preventDefault();
                    toggleSpotlight();
                }
                return;
            }

            if (e.key === 'Escape') {
                toggleSpotlight();
            } else if (e.key === 'ArrowDown') {
                e.preventDefault();
                setSelectedIndex(prev => (prev + 1) % Math.max(1, results.length));
            } else if (e.key === 'ArrowUp') {
                e.preventDefault();
                setSelectedIndex(prev => (prev - 1 + results.length) % Math.max(1, results.length));
            } else if (e.key === 'Enter') {
                e.preventDefault();
                if (results[selectedIndex]) {
                    launchItem(results[selectedIndex]);
                }
            }
        };

        window.addEventListener('keydown', handleKeyDown);
        return () => window.removeEventListener('keydown', handleKeyDown);
    }, [isSpotlightOpen, results, selectedIndex]);

    const launchItem = (item: any) => {
        if (item.type === 'app' || item.type === 'url') {
            launchApp(item.id);
        } else if (item.type === 'file') {
            // Intelligent launch based on extension
            const ext = item.name.split('.').pop()?.toLowerCase();
            if (['mp3', 'wav', 'ogg', 'mp4', 'mov'].includes(ext || '')) {
                launchApp('media', { src: item.path, title: item.name, mediaType: ['mp4', 'mov'].includes(ext!) ? 'video' : 'audio' });
            } else {
                // Default to File Explorer at that path
                launchApp('desktop', { initialPath: item.path });
            }
        } else if (item.type === 'directory') {
            launchApp('desktop', { initialPath: item.path });
        }
        toggleSpotlight();
    };

    return (
        <AnimatePresence>
            {isSpotlightOpen && (
                <motion.div
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    exit={{ opacity: 0 }}
                    className="fixed inset-0 z-[100] flex items-start justify-center pt-[20vh] bg-black/20 backdrop-blur-sm px-4"
                    onClick={toggleSpotlight}
                >
                    <motion.div
                        initial={{ scale: 0.95, y: -20 }}
                        animate={{ scale: 1, y: 0 }}
                        exit={{ scale: 0.95, y: -20 }}
                        className="w-full max-w-2xl bg-zinc-900/90 backdrop-blur-2xl rounded-2xl shadow-[0_32px_64px_-16px_rgba(0,0,0,0.5)] border border-white/10 overflow-hidden"
                        onClick={e => e.stopPropagation()}
                    >
                        <div className="flex items-center px-6 py-5 border-b border-white/5 gap-4">
                            <Search className={cn("text-white/40 transition-colors", isLoading && "text-amber-500 animate-pulse")} size={24} />
                            <input
                                ref={inputRef}
                                type="text"
                                placeholder="Search apps, files, and more..."
                                value={query}
                                onChange={e => setQuery(e.target.value)}
                                className="flex-1 bg-transparent border-none outline-none text-xl text-white placeholder:text-white/20"
                            />
                            <div className="flex items-center gap-1.5 px-2 py-1 bg-white/5 rounded-md border border-white/10 opacity-40">
                                <Command size={12} />
                                <span className="text-[10px] font-bold">SPACE</span>
                            </div>
                        </div>

                        <div className="p-2 max-h-[450px] overflow-y-auto">
                            {results.length > 0 ? (
                                results.map((item, idx) => {
                                    const Icon = item.icon;
                                    const selected = idx === selectedIndex;
                                    return (
                                        <div
                                            key={`${item.type}-${item.id}`}
                                            className={cn(
                                                "flex items-center justify-between px-4 py-3 rounded-xl cursor-pointer transition-all gap-4 group",
                                                selected ? "bg-amber-500 text-white shadow-lg" : "hover:bg-white/5 text-white/70"
                                            )}
                                            onMouseEnter={() => setSelectedIndex(idx)}
                                            onClick={() => launchItem(item)}
                                        >
                                            <div className="flex items-center gap-4">
                                                <div className={cn(
                                                    "w-10 h-10 rounded-lg flex items-center justify-center transition-colors",
                                                    selected ? "bg-white/20" : "bg-white/5 group-hover:bg-white/10"
                                                )}>
                                                    <Icon size={20} />
                                                </div>
                                                <div className="min-w-0">
                                                    <p className="font-bold text-sm truncate">{item.name}</p>
                                                    <p className={cn(
                                                        "text-[10px] font-bold uppercase tracking-widest opacity-60",
                                                        selected ? "text-white" : "text-amber-500"
                                                    )}>{item.category}</p>
                                                </div>
                                            </div>
                                            {selected && (
                                                <div className="flex items-center gap-2 opacity-60">
                                                    <span className="text-[10px] font-bold">{item.type === 'app' ? 'LAUNCH' : 'OPEN'}</span>
                                                    <ArrowRight size={14} />
                                                </div>
                                            )}
                                        </div>
                                    );
                                })
                            ) : (
                                <div className="py-12 text-center text-white/20">
                                    <Search className="mx-auto mb-4 opacity-10" size={48} />
                                    <p className="text-sm font-medium italic">
                                        {query.trim() ? `No results found for "${query}"` : "Start typing to search..."}
                                    </p>
                                </div>
                            )}
                        </div>

                        <div className="px-6 py-3 bg-white/[0.02] border-t border-white/5 flex justify-between items-center">
                            <div className="flex items-center gap-4 text-[10px] font-bold text-white/30 uppercase tracking-widest">
                                <div className="flex items-center gap-1">
                                    <span className="px-1.5 py-0.5 bg-white/5 rounded border border-white/10">ESC</span>
                                    <span>to close</span>
                                </div>
                                <div className="flex items-center gap-1">
                                    <span className="px-1.5 py-0.5 bg-white/5 rounded border border-white/10">↑↓</span>
                                    <span>to navigate</span>
                                </div>
                                <div className="flex items-center gap-1">
                                    <span className="px-1.5 py-0.5 bg-white/5 rounded border border-white/10">ENTER</span>
                                    <span>to open</span>
                                </div>
                            </div>
                            <div className="text-[10px] font-bold text-amber-500/50 uppercase tracking-widest">
                                Peak Search
                            </div>
                        </div>
                    </motion.div>
                </motion.div>
            )}
        </AnimatePresence>
    );
}
