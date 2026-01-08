
import { useState } from 'react';
import {
    ChevronRight, ChevronDown, Folder, LayoutGrid, Hash, FileText, Monitor,
    Download, Music, Image, Video, HardDrive, AppWindow, Globe, Server, TerminalSquare, Compass, Sparkles,
    Shield, Zap, Code, MessageCircle, PenTool, Layout
} from 'lucide-react';
import { cn } from '../lib/utils';

export type TreeItemData = {
    id: string;
    title: string;
    icon?: any;
    slug?: string;
    type?: 'folder' | 'file';
    children?: TreeItemData[];
};

interface TreeItemProps {
    item: TreeItemData;
    level?: number;
    activeId?: string;
    onSelect?: (item: TreeItemData) => void;
}

export function TreeItem({ item, level = 0, activeId, onSelect }: TreeItemProps) {
    const [isOpen, setIsOpen] = useState(level < 1); // Open top level by default
    const hasChildren = item.children && item.children.length > 0;

    // Dynamic Icon Logic:
    // 1. Explicit Icon (Overrides everything)
    // 2. Explicit Type 'folder' -> Folder Icon
    // 3. Has Children -> Folder Icon
    // 4. Default -> FileText Icon
    const IconComponent = item.icon || ((item.type === 'folder' || hasChildren) ? Folder : FileText);

    const isActive = activeId === item.id;

    const handleToggle = (e: React.MouseEvent) => {
        e.stopPropagation();
        setIsOpen(!isOpen);
    };

    const handleClick = (e: React.MouseEvent) => {
        e.stopPropagation();
        if (onSelect) onSelect(item);
        if (hasChildren && !isOpen) setIsOpen(true);
    };

    return (
        <div className="select-none text-foreground/80">
            <div
                className={cn(
                    "flex items-center gap-2 px-2 py-1.5 rounded-md transition-colors group text-sm cursor-pointer",
                    isActive
                        ? "" // No background for active state
                        : "hover:bg-black/5 dark:hover:bg-white/5",
                    level > 0 && "ml-3"
                )}
                onClick={handleClick}
            >
                {/* Expander Arrow */}
                <div
                    className={cn(
                        "w-4 h-4 flex items-center justify-center shrink-0 transition-colors rounded",
                        isActive ? "text-amber-500" : "text-muted-foreground/70 hover:text-foreground hover:bg-black/5 dark:hover:bg-white/10",
                        !hasChildren && "opacity-0 pointer-events-none"
                    )}
                    onClick={hasChildren ? handleToggle : undefined}
                >
                    {hasChildren && (
                        isOpen ? <ChevronDown className="w-3 H-3" /> : <ChevronRight className="w-3 h-3" />
                    )}
                </div>

                {/* Icon */}
                <div className={cn("w-4 h-4 flex items-center justify-center shrink-0", isActive ? "text-amber-500" : "text-muted-foreground")}>
                    <IconComponent className="w-4 h-4" />
                </div>

                {/* Title */}
                <span className={cn("flex-1 truncate transition-colors", isActive ? "text-amber-500 font-medium" : "group-hover:text-foreground")}>
                    {item.title}
                </span>
            </div>

            {/* Recursive Children */}
            {isOpen && hasChildren && (
                <div className="overflow-hidden border-l border-black/5 dark:border-white/5 ml-3.5 pl-1">
                    {item.children!.map((child) => (
                        <TreeItem
                            key={child.id}
                            item={child}
                            level={level + 1}
                            activeId={activeId}
                            onSelect={onSelect}
                        />
                    ))}
                </div>
            )}
        </div>
    );
}

const ICON_MAP: Record<string, any> = {
    sparkles: Sparkles,
    globe: Globe,
    zap: Zap,
    code: Code,
    shield: Shield,
    'pen-tool': PenTool,
    video: Video,
    'message-circle': MessageCircle,
    terminal: TerminalSquare,
    'file-test': Layout,
    'file-text': FileText,
    music: Music,
};

export function getDynamicTree(shortcuts: any[]): TreeItemData[] {
    const appsItems: TreeItemData[] = (shortcuts || [])
        .filter(s => s.pinnedToSidebar)
        .map(s => ({
            id: s.id,
            title: s.name,
            icon: ICON_MAP[s.icon] || Globe,
            slug: s.id
        }));

    return BASE_TREE_DATA.map(section => {
        if (section.id === 'favorites') {
            return {
                ...section,
                children: section.children?.map(child => {
                    if (child.id === 'applications') {
                        return { ...child, children: appsItems };
                    }
                    return child;
                })
            };
        }
        return section;
    });
}

const BASE_TREE_DATA: TreeItemData[] = [
    {
        id: 'favorites',
        title: 'Favorites',
        icon: LayoutGrid,
        type: 'folder',
        children: [
            { id: 'desktop', title: 'Desktop', icon: Monitor, slug: 'desktop' },
            { id: 'applications', title: 'Applications', icon: AppWindow, slug: 'applications', children: [] },
            { id: 'documents', title: 'Documents', icon: FileText, slug: 'documents' },
            { id: 'downloads', title: 'Downloads', icon: Download, slug: 'downloads' },
            { id: 'recents', title: 'Recents', icon: Hash, slug: 'recents' },
        ]
    },
    {
        id: 'media',
        title: 'Media',
        icon: Image,
        type: 'folder',
        children: [
            { id: 'pictures', title: 'Pictures', icon: Image, slug: 'pictures' },
            { id: 'music', title: 'Music', icon: Music, slug: 'music' },
            { id: 'movies', title: 'Movies', icon: Video, slug: 'movies' },
        ]
    },
    {
        id: 'system',
        title: 'PeakOS',
        icon: HardDrive, // Peak HD concept
        type: 'folder',
        children: [
            { id: 'root', title: 'Peak HD', icon: HardDrive, slug: 'root' },
            { id: 'network', title: 'Network', icon: Globe, slug: 'network' },
            { id: 'server', title: 'Servers', icon: Server, slug: 'servers' },
            { id: 'terminal', title: 'Terminal', icon: TerminalSquare, slug: 'terminal' },
            { id: 'browser', title: 'Browser', icon: Compass, slug: 'browser' },
        ]
    },
    {
        id: 'tags',
        title: 'Tags',
        children: [
            { id: 'red', title: 'Important', icon: Hash },
            { id: 'orange', title: 'Work', icon: Hash },
            { id: 'green', title: 'Personal', icon: Hash },
        ]
    }
];
