import { useState, useEffect } from 'react';
import { ArrowLeft, ArrowRight, Globe, Plus, X, RotateCcw, Home, Layout } from 'lucide-react';
import { Window } from '@tauri-apps/api/window';
import { Command } from '@tauri-apps/plugin-shell';
import { cn } from '../lib/utils';

interface Tab {
    id: string;
    url: string;
    title: string;
    history: string[];
    historyIndex: number;
}

interface BrowserProps {
    initialUrl?: string;
    isAppMode?: boolean;
}

export function Browser({ initialUrl = 'https://www.bing.com', isAppMode = false }: BrowserProps) {
    const [tabs, setTabs] = useState<Tab[]>([
        { id: '1', url: initialUrl, title: 'New Tab', history: [initialUrl], historyIndex: 0 }
    ]);
    const [activeTabId, setActiveTabId] = useState('1');
    const activeTab = tabs.find(t => t.id === activeTabId) || tabs[0];

    const [urlInput, setUrlInput] = useState(activeTab.url);

    useEffect(() => {
        setUrlInput(activeTab.url);
    }, [activeTabId]);

    const navigate = (input: string, tabId = activeTabId) => {
        let finalUrl = input;
        const isUrl = input.includes('.') && !input.includes(' ');

        if (isUrl) {
            if (!finalUrl.startsWith('http')) {
                finalUrl = `https://${finalUrl}`;
            }
        } else {
            finalUrl = `https://www.bing.com/search?q=${encodeURIComponent(input)}`;
        }

        setTabs(prev => prev.map(t => {
            if (t.id === tabId) {
                const newHistory = t.history.slice(0, t.historyIndex + 1);
                newHistory.push(finalUrl);
                return {
                    ...t,
                    url: finalUrl,
                    title: isUrl ? input.split('/')[2] || input : 'Search',
                    history: newHistory,
                    historyIndex: newHistory.length - 1
                };
            }
            return t;
        }));

        if (tabId === activeTabId) setUrlInput(finalUrl);
    };

    const addTab = () => {
        const newId = Date.now().toString();
        const newTab: Tab = {
            id: newId,
            url: 'https://www.bing.com',
            title: 'New Tab',
            history: ['https://www.bing.com'],
            historyIndex: 0
        };
        setTabs([...tabs, newTab]);
        setActiveTabId(newId);
    };

    const closeTab = (e: React.MouseEvent, id: string) => {
        e.stopPropagation();
        if (tabs.length === 1) return;
        const newTabs = tabs.filter(t => t.id !== id);
        setTabs(newTabs);
        if (activeTabId === id) {
            setActiveTabId(newTabs[newTabs.length - 1].id);
        }
    };

    const openNative = async () => {
        try {
            const browserWindow = await Window.getByLabel('browser');
            if (browserWindow) {
                await browserWindow.show();
                await browserWindow.setFocus();
            }
        } catch (e) {
            // Fallback: Try launching Host Firefox
            console.error('No tauri browser window, trying host app...', e);
            Command.create('open', ['-a', 'Firefox']).execute()
                .catch(() => Command.create('open', ['-a', 'Google Chrome']).execute())
                .catch(() => console.error('Failed to launch any browser'));
        }
    };

    return (
        <div className="flex flex-col h-full bg-white dark:bg-zinc-950 text-foreground overflow-hidden">
            {/* Tab Bar */}
            {!isAppMode && (
                <div className="flex items-center gap-1 px-2 pt-2 bg-zinc-100 dark:bg-zinc-900 border-b border-border/50 overflow-x-auto no-scrollbar">
                    {tabs.map((tab) => (
                        <div
                            key={tab.id}
                            onClick={() => setActiveTabId(tab.id)}
                            className={cn(
                                "flex items-center gap-2 px-3 py-1.5 min-w-[120px] max-w-[200px] rounded-t-xl text-xs font-medium cursor-pointer transition-all border-x border-t",
                                activeTabId === tab.id
                                    ? "bg-white dark:bg-zinc-800 border-border/50 text-foreground"
                                    : "bg-transparent border-transparent text-muted-foreground hover:bg-black/5 dark:hover:bg-white/5"
                            )}
                        >
                            <Globe size={12} className={activeTabId === tab.id ? "text-amber-500" : "opacity-50"} />
                            <span className="flex-1 truncate">{tab.title}</span>
                            <button
                                onClick={(e) => closeTab(e, tab.id)}
                                className="p-0.5 rounded-md hover:bg-black/10 dark:hover:bg-white/10 opacity-0 group-hover:opacity-100 transition-opacity"
                            >
                                <X size={10} />
                            </button>
                        </div>
                    ))}
                    <button
                        onClick={addTab}
                        className="p-1.5 mb-1 rounded-full hover:bg-black/5 dark:hover:bg-white/10 text-muted-foreground transition-colors"
                    >
                        <Plus size={14} />
                    </button>
                </div>
            )}

            {/* Toolbar */}
            {!isAppMode && (
                <div className="flex items-center gap-2 p-2 bg-white dark:bg-zinc-800 border-b border-border/50">
                    <div className="flex gap-0.5">
                        <button
                            onClick={() => {
                                if (activeTab.historyIndex > 0) {
                                    const newIdx = activeTab.historyIndex - 1;
                                    setTabs(tabs.map(t => t.id === activeTabId ? { ...t, url: t.history[newIdx], historyIndex: newIdx } : t));
                                }
                            }}
                            disabled={activeTab.historyIndex === 0}
                            className="p-1.5 rounded-lg hover:bg-black/5 dark:hover:bg-white/10 disabled:opacity-20"
                        >
                            <ArrowLeft size={16} />
                        </button>
                        <button
                            onClick={() => {
                                if (activeTab.historyIndex < activeTab.history.length - 1) {
                                    const newIdx = activeTab.historyIndex + 1;
                                    setTabs(tabs.map(t => t.id === activeTabId ? { ...t, url: t.history[newIdx], historyIndex: newIdx } : t));
                                }
                            }}
                            disabled={activeTab.historyIndex === activeTab.history.length - 1}
                            className="p-1.5 rounded-lg hover:bg-black/5 dark:hover:bg-white/10 disabled:opacity-20"
                        >
                            <ArrowRight size={16} />
                        </button>
                        <button
                            onClick={() => setTabs(tabs.map(t => t.id === activeTabId ? { ...t, url: t.url } : t))}
                            className="p-1.5 rounded-lg hover:bg-black/5 dark:hover:bg-white/10"
                        >
                            <RotateCcw size={16} />
                        </button>
                        <button
                            onClick={() => navigate('https://www.bing.com')}
                            className="p-1.5 rounded-lg hover:bg-black/5 dark:hover:bg-white/10"
                        >
                            <Home size={16} />
                        </button>
                    </div>

                    <form
                        onSubmit={(e) => { e.preventDefault(); navigate(urlInput); }}
                        className="flex-1"
                    >
                        <div className="relative group">
                            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-muted-foreground/50">
                                <Globe size={14} />
                            </div>
                            <input
                                type="text"
                                value={urlInput}
                                onChange={(e) => setUrlInput(e.target.value)}
                                className="w-full h-9 pl-9 pr-4 rounded-xl bg-zinc-100 dark:bg-zinc-900 border border-transparent focus:border-amber-500/50 text-sm focus:outline-none transition-all shadow-inner"
                                placeholder="Search or enter address"
                            />
                        </div>
                    </form>

                    <button
                        onClick={openNative}
                        className="p-1.5 rounded-lg hover:bg-amber-500 hover:text-white transition-all group"
                        title="Launch Host Browser (Firefox/Chrome)"
                    >
                        <Layout size={16} className="group-hover:scale-110 transition-transform" />
                    </button>
                </div>
            )}

            {/* Viewport */}
            <div className="flex-1 bg-white relative w-full h-full overflow-hidden">
                {tabs.map(tab => (
                    <iframe
                        key={tab.id}
                        src={tab.url}
                        className={cn(
                            "w-full h-full border-none absolute inset-0 transition-opacity duration-300",
                            activeTabId === tab.id ? "opacity-100 z-10" : "opacity-0 z-0 pointer-events-none"
                        )}
                        title={`Browser View - ${tab.title}`}
                        sandbox="allow-same-origin allow-scripts allow-forms allow-popups"
                    />
                ))}
            </div>

            {/* Status Bar */}
            <div className="px-3 py-1 bg-zinc-100 dark:bg-zinc-900 border-t border-border/30 text-[10px] text-muted-foreground flex justify-between items-center">
                <div className="flex items-center gap-2">
                    <div className="w-1.5 h-1.5 rounded-full bg-green-500 animate-pulse" />
                    <span>Peak Browser Engine v1.0</span>
                </div>
                <span>{activeTab.url}</span>
            </div>
        </div>
    );
}
