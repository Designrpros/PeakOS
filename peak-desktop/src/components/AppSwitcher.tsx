import { motion, AnimatePresence } from 'framer-motion';
import { useOSStore } from '../stores/useOSStore';
import { cn } from '../lib/utils';
import { Terminal as TerminalIcon, Globe, Music, Settings, Layout, Monitor } from 'lucide-react';

interface AppSwitcherProps {
    isOpen: boolean;
    selectedIndex: number;
}

export function AppSwitcher({ isOpen, selectedIndex }: AppSwitcherProps) {
    const { spaces } = useOSStore();

    // Gather all windows from all spaces
    const allWindows = spaces.flatMap(space =>
        space.windows.map(win => ({ ...win, spaceId: space.id }))
    );

    // Sort by zIndex (most recent first) - this is still per-space zIndex, but it's a good rough order
    const sortedWindows = [...allWindows].sort((a, b) =>
        (b.zIndex || 0) - (a.zIndex || 0)
    );

    if (sortedWindows.length === 0) return null;

    const getIcon = (type: string) => {
        switch (type) {
            case 'terminal': return TerminalIcon;
            case 'browser': return Globe;
            case 'media': return Music;
            case 'settings': return Settings;
            case 'antigravity': return Monitor;
            default: return Layout;
        }
    };

    return (
        <AnimatePresence>
            {isOpen && (
                <motion.div
                    initial={{ opacity: 0, scale: 1.1 }}
                    animate={{ opacity: 1, scale: 1 }}
                    exit={{ opacity: 0, scale: 1.1 }}
                    transition={{ duration: 0.15, ease: "easeOut" }}
                    className="fixed inset-0 z-[1000] flex items-center justify-center bg-black/5 pointer-events-none"
                >
                    <div className="bg-zinc-800/80 backdrop-blur-2xl border border-white/10 p-4 rounded-2xl shadow-[0_0_80px_rgba(0,0,0,0.5)] flex items-center gap-4">
                        {sortedWindows.map((win, index) => {
                            const Icon = getIcon(win.type);
                            const isSelected = index === selectedIndex % sortedWindows.length;

                            return (
                                <div
                                    key={win.id}
                                    className={cn(
                                        "w-24 h-24 flex flex-col items-center justify-center gap-2 rounded-xl transition-all duration-200",
                                        isSelected
                                            ? "bg-white/20 ring-1 ring-white/30 scale-105"
                                            : "opacity-60"
                                    )}
                                >
                                    <div className="w-12 h-12 flex items-center justify-center bg-white/5 rounded-lg">
                                        <Icon size={32} className="text-white" />
                                    </div>
                                    <span className="text-[10px] font-medium text-white/80 truncate max-w-[80px]">
                                        {win.title}
                                    </span>
                                </div>
                            );
                        })}
                    </div>
                </motion.div>
            )}
        </AnimatePresence>
    );
}
