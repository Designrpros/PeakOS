import { motion } from 'framer-motion';
import {
    Chrome, Terminal, Settings, Folder, Music, ShoppingBag, LayoutGrid,
    Sparkles, Globe, Zap, Code, MessageCircle, PenTool, Video, Layout
} from 'lucide-react';
import { useOSStore } from '../stores/useOSStore';

const DOCK_ICON_MAP: Record<string, any> = {
    sparkles: Sparkles,
    globe: Globe,
    zap: Zap,
    code: Code,
    'message-circle': MessageCircle,
    'pen-tool': PenTool,
    video: Video,
    'file-text': Layout,
    terminal: Terminal,
    music: Music,
};

const SYSTEM_APPS = [
    { id: 'desktop', name: 'Finder', icon: Folder, color: 'text-stone-800 dark:text-stone-200' },
    { id: 'app-store', name: 'App Store', icon: ShoppingBag, color: 'text-stone-800 dark:text-stone-200' },
    { id: 'terminal', name: 'Terminal', icon: Terminal, color: 'text-stone-800 dark:text-stone-200' },
    { id: 'browser', name: 'Browser', icon: Chrome, color: 'text-stone-800 dark:text-stone-200' },
    { id: 'media', name: 'Media', icon: Music, color: 'text-stone-800 dark:text-stone-200' },
    { id: 'settings', name: 'Settings', icon: Settings, color: 'text-stone-800 dark:text-stone-200' },
];

interface DockProps {
    autoHide?: boolean;
    onAppClick?: (id: string) => void;
}

export function Dock({ autoHide = true, onAppClick }: DockProps) {
    const { shortcuts = [] } = useOSStore();

    const allApps = [
        ...SYSTEM_APPS,
        ...shortcuts.filter(s => s.pinnedToDock).map(s => ({
            id: s.id,
            name: s.name,
            icon: DOCK_ICON_MAP[s.icon] || LayoutGrid,
            color: 'text-stone-800 dark:text-stone-200'
        }))
    ];

    return (
        // Root container: Covers bottom area but lets clicks pass through
        <div className="fixed bottom-0 left-0 right-0 z-50 flex justify-center pointer-events-none group h-16">

            {/* Trigger Zone: Invisible, always sits at bottom. Captures pointer events. */}
            {autoHide && (
                <div className="absolute bottom-0 w-full h-4 bg-transparent pointer-events-auto" />
            )}

            {/* Dock Payload: Animates in/out */}
            <div className={`
                pointer-events-auto
                relative px-4 py-1.5 rounded-2xl 
                bg-white/60 dark:bg-black/40 backdrop-blur-xl border border-white/20 shadow-2xl 
                flex items-center space-x-4
                transition-all duration-500 ease-spring
                ${autoHide
                    ? 'translate-y-[150%] group-hover:translate-y-[-10px] scale-90 group-hover:scale-100 opacity-50 group-hover:opacity-100'
                    : 'mb-4 translate-y-0'
                }
            `}>
                {allApps.map((app) => (
                    <motion.div
                        key={app.id}
                        whileHover={{ y: -5, scale: 1.2 }}
                        whileTap={{ scale: 0.9 }}
                        className="group/icon relative flex flex-col items-center cursor-pointer"
                        onClick={() => onAppClick?.(app.id)}
                    >
                        {/* Icon: Monochrome Adaptive & Small */}
                        <div className={`transition-all duration-200 ${app.color} opacity-90 group-hover/icon:opacity-100 filter drop-shadow-sm`}>
                            <app.icon size={24} strokeWidth={1.5} />
                        </div>

                        {/* Label Tooltip */}
                        <span className="absolute -top-8 bg-white/90 dark:bg-black/90 backdrop-blur-md px-2 py-0.5 rounded-md text-[10px] font-medium text-stone-800 dark:text-stone-200 opacity-0 group-hover/icon:opacity-100 transition-all duration-200 pointer-events-none border border-black/5 dark:border-white/10 shadow-lg transform translate-y-2 group-hover/icon:translate-y-0 min-w-max">
                            {app.name}
                        </span>

                        {/* Active Dot */}
                        <div className="absolute -bottom-1 w-0.5 h-0.5 bg-stone-800/40 dark:bg-stone-200/40 rounded-full opacity-0 group-hover/icon:opacity-100 transition-opacity" />
                    </motion.div>
                ))}
            </div>
        </div>
    );
}
