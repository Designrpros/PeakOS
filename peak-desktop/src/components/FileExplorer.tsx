import { useState, useEffect } from 'react';
import { useSystemLink } from '../hooks/useSystemLink';
import { open } from '@tauri-apps/plugin-shell';
import { Folder, FileText, FileImage, FileCode, Hexagon, ArrowLeft, Home, File, Film, Music, AppWindow } from 'lucide-react';
import { cn } from '../lib/utils';
import { motion } from 'framer-motion';

interface FileEntry {
    name: string;
    is_dir: boolean;
    size: number;
    path: string;
}

interface FileExplorerProps {
    initialPath?: string;
    onOpenFile?: (path: string, type: 'audio' | 'video') => void;
}

export function FileExplorer({ initialPath = '/Users/vegarberentsen', onOpenFile }: FileExplorerProps) {
    const [currentPath, setCurrentPath] = useState(initialPath);
    // If the prop changes (e.g. sidebar click), reset the path
    useEffect(() => {
        if (initialPath) setCurrentPath(initialPath);
    }, [initialPath]);

    const [files, setFiles] = useState<FileEntry[]>([]);
    const [isLoading, setIsLoading] = useState(false);
    const [userError, setUserError] = useState<string | null>(null);
    const { callTool, isConnected, initError } = useSystemLink();

    // History stack for back button
    // (Simple implementation: just go up one level for "Back" or keep a stack)
    // For "Up", we can just split path.

    const fetchFiles = async (path: string) => {
        if (!isConnected) return;
        setIsLoading(true);
        setUserError(null);
        try {
            const response = await callTool('read_dir', { path }) as any;
            // Handle MCP CallToolResult structure
            // Response is { content: [{ type: 'text', text: 'JSON_STRING' }], is_error: boolean }

            let data = response;
            if (response && response.content && Array.isArray(response.content) && response.content[0]?.text) {
                const rawText = response.content[0].text;
                try {
                    data = JSON.parse(rawText);
                } catch (e) {
                    // Sometimes the tool might return a plain string or error message
                    console.warn("FileExplorer: Failed to parse inner JSON. Raw text:", rawText);
                    if (rawText.startsWith("Error")) {
                        setUserError(rawText);
                    } else {
                        setUserError("Invalid server response");
                    }
                    return;
                }
            }

            if (Array.isArray(data)) {
                setFiles(data);
            } else {
                console.log("Unexpected data format:", data);
                setUserError("Invalid response format");
            }
        } catch (e: any) {
            console.error("Failed to read dir:", e);
            setUserError(e.message || "Failed to read directory");
        } finally {
            setIsLoading(false);
        }
    };

    useEffect(() => {
        fetchFiles(currentPath);
    }, [currentPath, isConnected]);

    const handleNavigate = async (entry: FileEntry) => {
        if (entry.name.endsWith('.app')) {
            console.log("Launching app:", entry.path);
            try {
                await open(entry.path);
            } catch (e) {
                console.error("Failed to open app:", e);
                setUserError("Failed to launch application");
            }
            return;
        }

        if (entry.is_dir) {
            setCurrentPath(entry.path);
            return;
        }

        // Check for media types
        const ext = entry.name.split('.').pop()?.toLowerCase();
        const audioExts = ['mp3', 'wav', 'flac', 'ogg', 'aac', 'm4a'];
        const videoExts = ['mp4', 'mov', 'mkv', 'webm', 'avi'];

        if (ext && audioExts.includes(ext)) {
            onOpenFile?.(entry.path, 'audio');
            return;
        }
        if (ext && videoExts.includes(ext)) {
            onOpenFile?.(entry.path, 'video');
            return;
        }

        // Default open
        console.log("Opening file:", entry.path);
        try {
            await open(entry.path);
        } catch (e) {
            console.error("Failed to open file:", e);
            // Don't show user error for file open failures, maybe just log it
            // or show a toast
        }
    };

    const handleUp = () => {
        // Simple parent resolution
        const parent = currentPath.split('/').slice(0, -1).join('/') || '/';
        setCurrentPath(parent);
    };

    const formatSize = (bytes: number) => {
        if (bytes === 0) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
    };

    const getIcon = (entry: FileEntry) => {
        if (entry.name.endsWith('.app')) return <AppWindow className="w-12 h-12 text-white/80" strokeWidth={1.5} />;
        if (entry.is_dir) return <Folder className="w-12 h-12 text-blue-400 fill-blue-400/20" strokeWidth={1.5} />;
        const ext = entry.name.split('.').pop()?.toLowerCase();
        switch (ext) {
            case 'png': case 'jpg': case 'jpeg': case 'gif': return <FileImage className="w-10 h-10 text-purple-400" strokeWidth={1.5} />;
            case 'mov': case 'mp4': return <Film className="w-10 h-10 text-red-400" strokeWidth={1.5} />;
            case 'mp3': case 'wav': return <Music className="w-10 h-10 text-pink-400" strokeWidth={1.5} />;
            case 'ts': case 'tsx': case 'js': case 'json': case 'rs': case 'css': case 'html': return <FileCode className="w-10 h-10 text-yellow-400" strokeWidth={1.5} />;
            case 'md': case 'txt': return <FileText className="w-10 h-10 text-gray-400" strokeWidth={1.5} />;
            default: return <File className="w-10 h-10 text-gray-500" strokeWidth={1.5} />;
        }
    };

    return (
        <div className="flex flex-col h-full bg-background text-foreground">
            {/* Toolbar */}
            <div className="flex items-center gap-4 px-6 py-3 border-b border-white/10 bg-black/20 backdrop-blur-md">
                <div className="flex items-center gap-2">
                    <button
                        onClick={handleUp}
                        disabled={currentPath === '/'}
                        className="p-1.5 rounded-md hover:bg-white/10 disabled:opacity-30 transition-colors"
                    >
                        <ArrowLeft size={18} />
                    </button>
                    <button
                        onClick={() => setCurrentPath('/Users/vegarberentsen')}
                        className="p-1.5 rounded-md hover:bg-white/10 transition-colors"
                    >
                        <Home size={18} />
                    </button>
                </div>

                {/* Path Bar */}
                <div className="flex-1 px-3 py-1.5 bg-black/30 border border-white/10 rounded-lg text-sm font-mono text-white/70 overflow-hidden text-ellipsis whitespace-nowrap shadow-inner">
                    {currentPath}
                </div>
            </div>

            {/* Content Area */}
            <div className="flex-1 overflow-y-auto p-6">
                {isLoading ? (
                    <div className="flex items-center justify-center h-full text-white/30 animate-pulse">
                        Loading...
                    </div>
                ) : userError ? (
                    <div className="flex flex-col items-center justify-center h-full text-red-400 gap-2">
                        <Hexagon size={48} className="opacity-50" />
                        <p>{userError}</p>
                        <button onClick={() => fetchFiles(currentPath)} className="px-4 py-2 bg-white/10 rounded-lg text-sm hover:bg-white/20 mt-4">Retry</button>
                    </div>
                ) : (
                    <div className="grid grid-cols-[repeat(auto-fill,minmax(120px,1fr))] gap-6">
                        {files.map((entry, idx) => (
                            <motion.div
                                key={entry.path}
                                initial={{ opacity: 0, scale: 0.9 }}
                                animate={{ opacity: 1, scale: 1 }}
                                transition={{ delay: idx * 0.02 }}
                                onDoubleClick={() => handleNavigate(entry)}
                                className={cn(
                                    "flex flex-col items-center gap-3 p-4 rounded-xl transition-all cursor-pointer group",
                                    "hover:bg-white/5 active:bg-white/10 border border-transparent hover:border-white/10"
                                )}
                            >
                                <div className="relative group-hover:scale-110 transition-transform duration-200">
                                    {getIcon(entry)}
                                </div>
                                <div className="text-center w-full">
                                    <p className="text-xs font-medium truncate w-full px-1" title={entry.name}>
                                        {entry.name.endsWith('.app') ? entry.name.replace('.app', '') : entry.name}
                                    </p>
                                    <p className="text-[10px] text-white/30 mt-0.5">
                                        {entry.name.endsWith('.app') ? 'Application' : (entry.is_dir ? 'Folder' : formatSize(entry.size))}
                                    </p>
                                </div>
                            </motion.div>
                        ))}
                        {files.length === 0 && (
                            <div className="col-span-full text-center text-white/20 py-20">
                                Empty Directory
                            </div>
                        )}
                    </div>
                )}
            </div>

            {/* Status Bar */}
            <div className="px-4 py-1.5 bg-black/40 border-t border-white/5 text-[10px] text-white/30 flex justify-between">
                <span>{files.length} items</span>
                <span className={cn(isConnected ? "text-green-500/50" : "text-red-500/50")}>
                    {isConnected ? "Connected" : (initError ? `Error: ${initError}` : "Disconnected")}
                </span>
            </div>
        </div>
    );
}
