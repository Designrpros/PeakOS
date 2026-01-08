import { useState, useEffect, useRef } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { cn } from '../lib/utils';
import { Sparkles, Send, Mic, Settings, Moon, Sun, Monitor, Cpu, Key, Database, LogOut, User, Smile, Heart, Camera, Gamepad2, Music, Coffee, Globe, Star, Cloud, Zap, Aperture, Leaf, Flame, Anchor, Rocket, Plane, Lock, ChevronRight } from 'lucide-react';
import { useOSStore } from '../stores/useOSStore';
import { useSystemLink } from '../hooks/useSystemLink';
import { chatCompletion, Message as AIMessage } from '../lib/ai';

// --- Types ---
export type InspectorMode = 'chat' | 'settings';
export type Theme = 'dark' | 'light' | 'system';
export type AIProvider = 'local' | 'openrouter';

export interface AISettings {
    provider: AIProvider;
    apiKey: string;
    model: string;
}

interface InspectorProps {
    mode: InspectorMode;
    theme: Theme;
    setTheme: (theme: Theme) => void;
}

export function Inspector({ mode, theme, setTheme }: InspectorProps) {
    // AI Settings State (Persisted)
    const [aiSettings, setAiSettings] = useState<AISettings>(() => {
        if (typeof window !== 'undefined') {
            const saved = localStorage.getItem('peak-ai-settings');
            if (saved) {
                try {
                    return JSON.parse(saved);
                } catch (e) { console.error("Failed to parse AI settings", e); }
            }
        }
        return { provider: 'openrouter', apiKey: '', model: 'google/gemini-3-flash-preview' };
    });

    useEffect(() => {
        localStorage.setItem('peak-ai-settings', JSON.stringify(aiSettings));
    }, [aiSettings]);

    return (
        <div className="w-full h-full hidden lg:flex flex-col bg-background/40 backdrop-blur-xl border-l border-border/50 text-foreground transition-all duration-300">
            {/* Header */}
            <div className="h-12 border-b border-border/50 flex items-center justify-between px-4">
                <div className="flex items-center space-x-2 text-foreground font-medium">
                    {mode === 'chat' ? (
                        <>
                            <Sparkles size={16} className="text-amber-500" />
                            <span>Peak Intelligence</span>
                        </>
                    ) : (
                        <>
                            <Settings size={16} className="text-foreground/70" />
                            <span>System Settings</span>
                        </>
                    )}
                </div>
                {mode === 'chat' && (
                    <div className="px-2 py-0.5 rounded-full bg-amber-500/10 text-amber-600 text-[10px] font-bold tracking-wide border border-amber-500/20">
                        BETA
                    </div>
                )}
            </div>

            {/* Content Area */}
            <div className="flex-1 overflow-y-auto">
                {mode === 'chat' ? (
                    <ChatView aiSettings={aiSettings} />
                ) : (
                    <SettingsView
                        theme={theme}
                        setTheme={setTheme}
                        aiSettings={aiSettings}
                        setAiSettings={setAiSettings}
                    />
                )}
            </div>
        </div>
    );
}

// --- Sub-Components ---


import { ShieldAlert, Check, X } from 'lucide-react';

function ChatView({ aiSettings }: { aiSettings: AISettings }) {
    const [input, setInput] = useState('');
    const [messages, setMessages] = useState<AIMessage[]>([]);
    const [isThinking, setIsThinking] = useState(false);
    const [pendingTool, setPendingTool] = useState<{ tool: any, args: any, depth: number } | null>(null);

    const { isConnected, initError, callTool } = useSystemLink();
    const hasInit = useRef(false);
    const messagesEndRef = useRef<HTMLDivElement>(null);

    const scrollToBottom = () => {
        messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
    };

    useEffect(() => {
        scrollToBottom();
    }, [messages, isThinking, pendingTool]);

    // Initial Error Check
    useEffect(() => {
        if (initError) {
            setMessages(prev => [
                ...prev,
                { role: 'assistant', content: `❌ Helper Initializtion Failed: ${initError}\n\nDid you restart the terminal process (npm run tauri dev) after the update?` }
            ]);
        }
    }, [initError]);

    // Connection Timeout Check
    useEffect(() => {
        if (!isConnected && !hasInit.current) {
            const timer = setTimeout(() => {
                setMessages(prev => [
                    ...prev,
                    { role: 'assistant', content: "⚠️ Connection timed out. Please restart the application to apply the new system permissions." }
                ]);
            }, 5000);
            return () => clearTimeout(timer);
        }
    }, [isConnected]);

    useEffect(() => {
        if (isConnected && !hasInit.current) {
            hasInit.current = true;
            // Silent init - no auto messages
        }
    }, [isConnected, callTool]);

    const handleSend = async () => {
        if (!input.trim() || isThinking || pendingTool) return;

        const userText = input;
        setInput('');

        const newMsg: AIMessage = { role: 'user', content: userText };
        const history = [...messages, newMsg];
        setMessages(history);
        setIsThinking(true);

        try {
            await processTurn(history, 0);
        } catch (e: any) {
            setMessages(prev => [...prev, { role: 'assistant', content: `Error: ${e.message || "Unknown error"}` }]);
        } finally {
            setIsThinking(false);
        }
    };

    const processTurn = async (history: AIMessage[], depth: number) => {
        if (depth > 5) {
            setMessages(prev => [...prev, { role: 'assistant', content: "⚠️ Request too complex (max tool depth reached)." }]);
            setIsThinking(false);
            return;
        }

        // 1. Call AI
        const response = await chatCompletion(history, aiSettings);

        // 2. Add response to history
        const newHistory = [...history, response];
        setMessages(newHistory);

        // 3. Handle Tool Calls
        if (response.tool_calls && response.tool_calls.length > 0) {
            // We only handle one tool at a time for simplicity in approval flow
            // But realistically LLM could call multiple. We'll take the first one.
            const tool = response.tool_calls[0];
            const args = JSON.parse(tool.function.arguments);

            // SECURITY CHECK
            const sensitiveTools = ['kill_process', 'write_file'];
            if (sensitiveTools.includes(tool.function.name)) {
                // Pause execution and ask for approval
                setPendingTool({ tool, args, depth });
                setIsThinking(false); // Stop thinking spinner, wait for user
                return;
            }

            // If safe, execute immediately
            await executeToolResult(tool, args, newHistory, depth);
        } else {
            setIsThinking(false);
        }
    };

    const executeToolResult = async (tool: any, args: any, history: AIMessage[], depth: number) => {
        setIsThinking(true);
        try {
            console.log("Executing tool:", tool.function.name, args);
            let result = await callTool(tool.function.name, args);

            // Optimization: Truncate massive process lists
            if (tool.function.name === 'list_processes' && Array.isArray(result) && result.length > 50) {
                const originalLen = result.length;
                result = result.slice(0, 50);
                result = {
                    items: result,
                    note: `Output truncated. Showing 50 of ${originalLen} processes.`
                };
            }

            const toolResultMsg: AIMessage = {
                role: 'tool',
                tool_call_id: tool.id,
                name: tool.function.name,
                content: typeof result === 'string' ? result : JSON.stringify(result)
            };

            const nextHistory = [...history, toolResultMsg];
            // setMessages(nextHistory); // Optional: update UI with the "hidden" tool result? Usually better to just wait for AI response.

            // Recursion
            await processTurn(nextHistory, depth + 1);

        } catch (err: any) {
            console.error("Tool execution failed:", err);
            const errorMsg: AIMessage = {
                role: 'tool',
                tool_call_id: tool.id,
                name: tool.function.name,
                content: `Error: ${err.message}`
            };
            await processTurn([...history, errorMsg], depth + 1);
        }
    };

    const handleApprove = async () => {
        if (!pendingTool) return;
        const { tool, args, depth } = pendingTool;
        setPendingTool(null);

        // Re-construct the history up to this point
        // The last message in 'messages' is the assistant's request.
        // We need to execute and append result.
        await executeToolResult(tool, args, messages, depth);
    };

    const handleReject = () => {
        if (!pendingTool) return;
        setPendingTool(null);

        const rejectionMsg: AIMessage = {
            role: 'tool',
            tool_call_id: pendingTool.tool.id,
            name: pendingTool.tool.function.name,
            content: "User rejected the action."
        };

        // Tell AI it was rejected
        const newHistory = [...messages, rejectionMsg];
        setMessages(newHistory);
        setIsThinking(true);
        processTurn(newHistory, pendingTool.depth + 1);
    };

    const textareaRef = useRef<HTMLTextAreaElement>(null);

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            handleSend();
        }
    };

    useEffect(() => {
        if (textareaRef.current) {
            textareaRef.current.style.height = 'inherit'; // Reset height to shrink
            const scrollHeight = textareaRef.current.scrollHeight;
            textareaRef.current.style.height = `${Math.min(scrollHeight, 200)}px`;
        }
    }, [input]);

    return (
        <div className="flex flex-col h-full">
            <div className="flex-1 p-4 space-y-4 overflow-y-auto">
                {messages.filter(m => m.role !== 'tool' && m.role !== 'system').map((m, i) => (
                    <Message key={i} role={m.role} text={m.content || ''} toolCalls={m.tool_calls} />
                ))}

                {/* Approval UI */}
                {pendingTool && (
                    <div className="flex justify-start animate-in fade-in slide-in-from-bottom-4 duration-300">
                        <div className="bg-amber-500/10 border border-amber-500/50 rounded-2xl p-4 max-w-sm space-y-3">
                            <div className="flex items-center gap-2 text-amber-500 font-bold text-sm">
                                <ShieldAlert size={16} />
                                <span>Approval Required</span>
                            </div>
                            <p className="text-sm text-foreground/90">
                                The AI wants to execute a sensitive action:
                            </p>
                            <div className="bg-black/40 rounded p-2 font-mono text-xs border border-white/10">
                                <span className="text-blue-400">{pendingTool.tool.function.name}</span>
                                <span className="text-muted-foreground">(</span>
                                <span className="text-orange-300">{JSON.stringify(pendingTool.args)}</span>
                                <span className="text-muted-foreground">)</span>
                            </div>
                            <div className="flex gap-2 pt-1">
                                <button
                                    onClick={handleApprove}
                                    className="flex-1 bg-green-500/20 hover:bg-green-500/30 text-green-500 border border-green-500/50 py-1.5 rounded-lg text-xs font-bold transition-colors flex items-center justify-center gap-1"
                                >
                                    <Check size={12} /> Allow
                                </button>
                                <button
                                    onClick={handleReject}
                                    className="flex-1 bg-red-500/20 hover:bg-red-500/30 text-red-500 border border-red-500/50 py-1.5 rounded-lg text-xs font-bold transition-colors flex items-center justify-center gap-1"
                                >
                                    <X size={12} /> Deny
                                </button>
                            </div>
                        </div>
                    </div>
                )}

                {isThinking && (
                    <div className="flex justify-start">
                        <div className="bg-card border border-border/50 px-4 py-2 rounded-2xl text-xs text-muted-foreground animate-pulse">
                            Thinking...
                        </div>
                    </div>
                )}
                <div ref={messagesEndRef} />
            </div>

            {/* Input Area */}
            <div className="p-4 border-t border-border/50 bg-background/40">
                <div className="relative flex items-end bg-background/50 border border-border rounded-xl shadow-inner focus-within:ring-2 focus-within:ring-amber-500/20 focus-within:border-amber-500/50 transition-all">
                    <textarea
                        ref={textareaRef}
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        onKeyDown={handleKeyDown}
                        disabled={isThinking || !!pendingTool}
                        placeholder={pendingTool ? "Waiting for approval..." : (isConnected ? (isThinking ? "Processing..." : "Ask Peak...") : "Connecting...")}
                        rows={1}
                        className="w-full bg-transparent border-none pl-4 pr-12 py-3 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-0 resize-none max-h-[200px] disabled:opacity-50 disabled:cursor-not-allowed"
                    />
                    <div className="absolute right-2 bottom-1.5 flex items-center space-x-1 pb-0.5">
                        <button className="p-1.5 hover:bg-black/5 dark:hover:bg-white/10 rounded-lg text-muted-foreground transition-colors">
                            <Mic size={16} />
                        </button>
                        <button
                            onClick={handleSend}
                            disabled={isThinking || !!pendingTool || !input.trim()}
                            className="p-1.5 bg-foreground text-background rounded-lg hover:opacity-90 transition-opacity disabled:opacity-50"
                        >
                            <Send size={14} />
                        </button>
                    </div>
                </div>
            </div>
        </div>
    );
}

interface SettingsViewProps {
    theme: Theme;
    setTheme: (t: Theme) => void;
    aiSettings: AISettings;
    setAiSettings: (s: AISettings) => void;
}

function SettingsView({ theme, setTheme, aiSettings, setAiSettings }: SettingsViewProps) {
    const { user, setUser, logout } = useOSStore();
    const [isEditingProfile, setIsEditingProfile] = useState(false);
    const [editName, setEditName] = useState('');
    const [editAvatar, setEditAvatar] = useState('User');

    // Sync edit state with store when user changes or when entering edit mode
    useEffect(() => {
        if (user) {
            setEditName(user.fullName);
            setEditAvatar(user.avatar);
        }
    }, [user, isEditingProfile]);

    const avatarIcons: Record<string, any> = {
        User, Smile, Heart, Camera, Gamepad2,
        Music, Coffee, Globe, Star, Cloud,
        Moon, Sun, Zap, Aperture, Leaf,
        Flame, Anchor, Rocket, Plane
    };
    const AvatarIcon = avatarIcons[user?.avatar || 'User'] || User;

    const handleSaveProfile = () => {
        const currentUser = user || { username: 'peak', fullName: '', avatar: 'User' };
        setUser({
            ...currentUser,
            fullName: editName,
            avatar: editAvatar
        });
        setIsEditingProfile(false);
    };

    return (
        <div className="p-6 space-y-8">
            {/* Section: Profile */}
            <section className="space-y-4">
                <div className="flex items-center justify-between">
                    <h3 className="text-xs font-bold text-muted-foreground uppercase tracking-wider">Profile</h3>
                    {!isEditingProfile && (
                        <button
                            onClick={() => setIsEditingProfile(true)}
                            className="text-[10px] font-bold text-amber-500 hover:text-amber-600 transition-colors uppercase tracking-widest"
                        >
                            Edit
                        </button>
                    )}
                </div>

                <AnimatePresence mode="wait">
                    {isEditingProfile ? (
                        <motion.div
                            key="edit-profile"
                            initial={{ opacity: 0, scale: 0.98 }}
                            animate={{ opacity: 1, scale: 1 }}
                            exit={{ opacity: 0, scale: 0.98 }}
                            className="bg-background/50 border border-amber-500/30 rounded-2xl p-4 space-y-4 shadow-lg shadow-amber-500/5"
                        >
                            <div className="space-y-4">
                                <div className="grid grid-cols-5 gap-2 pb-2 border-b border-border/50 overflow-x-auto no-scrollbar py-2">
                                    {Object.keys(avatarIcons).map((iconName) => {
                                        const Icon = avatarIcons[iconName];
                                        const isSelected = editAvatar === iconName;
                                        return (
                                            <button
                                                key={iconName}
                                                onClick={() => setEditAvatar(iconName)}
                                                className={cn(
                                                    "p-2 rounded-lg flex items-center justify-center transition-all",
                                                    isSelected ? "bg-foreground text-background" : "hover:bg-foreground/5 text-muted-foreground"
                                                )}
                                            >
                                                <Icon size={16} />
                                            </button>
                                        );
                                    })}
                                </div>
                                <div className="space-y-1.5">
                                    <label className="text-[10px] font-bold text-muted-foreground uppercase ml-1">Display Name</label>
                                    <input
                                        type="text"
                                        value={editName}
                                        onChange={(e) => setEditName(e.target.value)}
                                        className="w-full bg-background border border-border rounded-lg px-3 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-amber-500/50"
                                    />
                                </div>
                                <div className="flex gap-2">
                                    <button
                                        onClick={handleSaveProfile}
                                        className="flex-1 bg-foreground text-background py-1.5 rounded-lg text-xs font-bold hover:opacity-90 transition-opacity"
                                    >
                                        Save Changes
                                    </button>
                                    <button
                                        onClick={() => {
                                            setIsEditingProfile(false);
                                            setEditName(user?.fullName || '');
                                            setEditAvatar(user?.avatar || 'User');
                                        }}
                                        className="px-3 border border-border py-1.5 rounded-lg text-xs font-bold hover:bg-foreground/5"
                                    >
                                        Cancel
                                    </button>
                                </div>
                            </div>
                        </motion.div>
                    ) : (
                        <motion.div
                            key="view-profile"
                            initial={{ opacity: 0, scale: 0.98 }}
                            animate={{ opacity: 1, scale: 1 }}
                            exit={{ opacity: 0, scale: 0.98 }}
                            className="bg-background/50 border border-border/50 rounded-2xl p-4 flex items-center gap-4 group/profile"
                        >
                            <button
                                onClick={() => setIsEditingProfile(true)}
                                className="w-12 h-12 bg-foreground text-background rounded-full flex items-center justify-center shadow-lg relative overflow-hidden group/avatar transition-transform hover:scale-105 active:scale-95"
                                title="Edit Profile"
                            >
                                <AvatarIcon size={24} strokeWidth={1.5} />
                                <div className="absolute inset-0 bg-black/40 opacity-0 group-hover/avatar:opacity-100 flex items-center justify-center transition-opacity">
                                    <Settings size={14} className="text-white animate-spin-slow" />
                                </div>
                            </button>
                            <div className="flex-1 min-w-0 pointer-events-none">
                                <h4 className="font-semibold text-sm truncate">{user?.fullName || 'Guest'}</h4>
                                <p className="text-[11px] text-muted-foreground truncate">@{user?.username || 'peak'}</p>
                            </div>
                            <button
                                onClick={() => logout()}
                                className="p-2 hover:bg-white/5 rounded-lg text-muted-foreground transition-colors group"
                                title="Lock Screen"
                            >
                                <Lock size={16} className="group-hover:text-amber-500 transition-colors" />
                            </button>
                        </motion.div>
                    )}
                </AnimatePresence>
            </section>

            {/* Section: Advanced Settings Link */}
            <section className="px-1">
                <button
                    onClick={() => {
                        const { launchApp, setCurrentView } = useOSStore.getState();
                        launchApp('settings', { type: 'settings', title: 'System Settings' });
                        setCurrentView('desktop');
                    }}
                    className="w-full bg-secondary/20 hover:bg-secondary/40 border border-border/50 rounded-2xl p-4 flex items-center justify-between group transition-all"
                >
                    <div className="flex items-center gap-3">
                        <div className="p-2 bg-foreground/5 rounded-xl group-hover:scale-110 transition-transform">
                            <Settings size={18} />
                        </div>
                        <div className="text-left">
                            <p className="text-sm font-bold">Advanced Settings</p>
                            <p className="text-[10px] text-muted-foreground">Manage users, guest mode, and system</p>
                        </div>
                    </div>
                    <ChevronRight size={16} className="text-muted-foreground group-hover:translate-x-1 transition-transform" />
                </button>
            </section>

            {/* Section: Appearance */}
            <section className="space-y-4">
                <h3 className="text-xs font-bold text-muted-foreground uppercase tracking-wider">Appearance</h3>

                <div className="space-y-3">
                    <label className="text-sm font-medium block">Theme Mode</label>
                    <div className="grid grid-cols-3 gap-2">
                        <ThemeOption
                            label="Light"
                            icon={Sun}
                            active={theme === 'light'}
                            onClick={() => setTheme('light')}
                        />
                        <ThemeOption
                            label="Dark"
                            icon={Moon}
                            active={theme === 'dark'}
                            onClick={() => setTheme('dark')}
                        />
                        <ThemeOption
                            label="System"
                            icon={Monitor}
                            active={theme === 'system'}
                            onClick={() => setTheme('system')}
                        />
                    </div>
                </div>
            </section>

            {/* Section: Intelligence */}
            <section className="space-y-4 pt-4 border-t border-border/50">
                <h3 className="text-xs font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-2">
                    <Sparkles size={12} className="text-amber-500" />
                    Intelligence
                </h3>

                <div className="space-y-4">
                    {/* Provider Selection */}
                    <div className="space-y-2">
                        <label className="text-sm font-medium block">AI Provider</label>
                        <div className="grid grid-cols-2 gap-2">
                            <ThemeOption
                                label="Local (Ollama)"
                                icon={Cpu}
                                active={aiSettings.provider === 'local'}
                                onClick={() => setAiSettings({ ...aiSettings, provider: 'local' })}
                            />
                            <ThemeOption
                                label="OpenRouter"
                                icon={Database}
                                active={aiSettings.provider === 'openrouter'}
                                onClick={() => setAiSettings({ ...aiSettings, provider: 'openrouter' })}
                            />
                        </div>
                    </div>

                    {/* API Key (Conditional) */}
                    {aiSettings.provider === 'openrouter' && (
                        <div className="space-y-2">
                            <label className="text-sm font-medium block flex items-center gap-2">
                                <Key size={14} />
                                API Key
                            </label>
                            <input
                                type="password"
                                value={aiSettings.apiKey}
                                onChange={(e) => setAiSettings({ ...aiSettings, apiKey: e.target.value })}
                                placeholder="sk-or-..."
                                className="w-full bg-background/50 border border-border rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-amber-500/20 focus:border-amber-500/50 transition-all font-mono"
                            />
                            <p className="text-[10px] text-muted-foreground">Key is stored locally in your browser.</p>
                        </div>
                    )}

                    {/* Model ID */}
                    <div className="space-y-2">
                        <label className="text-sm font-medium block">Model ID</label>
                        <input
                            type="text"
                            value={aiSettings.model}
                            onChange={(e) => setAiSettings({ ...aiSettings, model: e.target.value })}
                            placeholder={aiSettings.provider === 'local' ? "llama3" : "google/gemini-pro-1.5"}
                            className="w-full bg-background/50 border border-border rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-amber-500/20 focus:border-amber-500/50 transition-all font-mono"
                        />
                    </div>
                </div>
            </section>

            {/* Section: Session */}
            <section className="space-y-4 pt-4 border-t border-border/50">
                <h3 className="text-xs font-bold text-muted-foreground uppercase tracking-wider">Session</h3>
                <div className="space-y-2">
                    <button
                        onClick={() => logout()}
                        className="w-full flex items-center justify-center gap-2 p-3 rounded-xl border border-border bg-background/50 text-foreground hover:bg-black/5 dark:hover:bg-white/5 transition-all font-medium text-sm"
                    >
                        <Lock size={16} />
                        Lock Screen
                    </button>
                    <button
                        onClick={() => {
                            localStorage.removeItem('peakos-setup-complete');
                            localStorage.removeItem('peakos-user');
                            window.location.reload();
                        }}
                        className="w-full flex items-center justify-center gap-2 p-3 rounded-xl border border-red-500/20 bg-red-500/5 text-red-500 hover:bg-red-500/10 transition-all font-medium text-[11px] opacity-50 hover:opacity-100"
                    >
                        <LogOut size={14} />
                        Reset System & Re-run Setup
                    </button>
                </div>
            </section>
        </div>
    );
}

function ThemeOption({ label, icon: Icon, active, onClick }: { label: string, icon: any, active: boolean, onClick: () => void }) {
    return (
        <button
            onClick={onClick}
            className={cn(
                "flex flex-col items-center justify-center gap-2 p-3 rounded-xl border transition-all duration-200",
                active
                    ? "bg-amber-500/10 border-amber-500/50 text-amber-600 dark:text-amber-400"
                    : "bg-background/50 border-border hover:bg-black/5 dark:hover:bg-white/5 text-muted-foreground"
            )}
        >
            <Icon size={18} />
            <span className="text-xs font-medium">{label}</span>
        </button>
    );
}


import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

function Message({ role, text, toolCalls }: { role: string, text: string, toolCalls?: any[] }) {
    const isAi = role === 'assistant';

    return (
        <div className={`flex flex-col space-y-2 ${isAi ? 'items-start' : 'items-end'}`}>
            {/* Tool Calls Indicator */}
            {toolCalls && toolCalls.map((tc, i) => (
                <div key={i} className="text-[10px] font-mono bg-amber-500/10 text-amber-600 px-2 py-1 rounded border border-amber-500/20 flex items-center gap-1">
                    <Cpu size={10} />
                    Executing: {tc.function.name}
                </div>
            ))}

            {/* Message Content */}
            {text && (
                <div className={`max-w-[85%] rounded-2xl px-4 py-2.5 text-sm shadow-sm overflow-hidden ${isAi
                    ? 'bg-card border border-border/50 text-card-foreground'
                    : 'bg-primary text-primary-foreground'
                    }`}>
                    {isAi ? (
                        <ReactMarkdown
                            remarkPlugins={[remarkGfm]}
                            components={{
                                table: ({ node, ...props }) => <div className="overflow-x-auto my-2 rounded-lg border border-border/50"><table className="w-full text-xs text-left" {...props} /></div>,
                                thead: ({ node, ...props }) => <thead className="bg-muted/50 text-muted-foreground" {...props} />,
                                tbody: ({ node, ...props }) => <tbody className="divide-y divide-border/20" {...props} />,
                                tr: ({ node, ...props }) => <tr className="hover:bg-muted/20 transition-colors" {...props} />,
                                th: ({ node, ...props }) => <th className="p-2 font-semibold whitespace-nowrap" {...props} />,
                                td: ({ node, ...props }) => <td className="p-2 align-top" {...props} />,
                                p: ({ node, ...props }) => <p className="mb-2 last:mb-0 leading-relaxed" {...props} />,
                                ul: ({ node, ...props }) => <ul className="list-disc pl-4 mb-2 space-y-1" {...props} />,
                                ol: ({ node, ...props }) => <ol className="list-decimal pl-4 mb-2 space-y-1" {...props} />,
                                code: ({ node, inline, className, children, ...props }: any) => {
                                    const match = /language-(\w+)/.exec(className || '')
                                    return !inline ? (
                                        <div className="my-2 rounded-lg overflow-hidden border border-border/50 bg-[#1e1e1e]">
                                            <div className="bg-white/5 px-3 py-1 text-[10px] font-mono text-muted-foreground border-b border-white/5">
                                                {match ? match[1] : 'code'}
                                            </div>
                                            <code className="block p-3 font-mono text-xs overflow-x-auto" {...props}>
                                                {children}
                                            </code>
                                        </div>
                                    ) : (
                                        <code className="bg-white/10 px-1 py-0.5 rounded text-[11px] font-mono" {...props}>
                                            {children}
                                        </code>
                                    )
                                }
                            }}
                        >
                            {text}
                        </ReactMarkdown>
                    ) : (
                        <div className="whitespace-pre-wrap">{text}</div>
                    )}
                </div>
            )}
        </div>
    );
}
