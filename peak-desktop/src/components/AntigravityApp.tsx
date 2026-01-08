import { useState, useEffect, useRef } from 'react';
import { Send, Bot, User, Sparkles, Loader2, Cpu, Brain } from 'lucide-react';
import { chatCompletion, Message } from '../lib/ai';
import { cn } from '../lib/utils';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

export function AntigravityApp() {
    const [messages, setMessages] = useState<Message[]>([
        { role: 'system', content: 'You are Antigravity, a powerful agentic AI coding assistant designed by the Google Deepmind team. You are integrated into PeakOS. You can help with system tasks, coding, and general questions.' },
        { role: 'assistant', content: 'Hello! I am Antigravity. How can I assist you in PeakOS today?' }
    ]);
    const [input, setInput] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const scrollRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (scrollRef.current) {
            scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
        }
    }, [messages, isLoading]);

    const handleSend = async () => {
        if (!input.trim() || isLoading) return;

        const userMessage: Message = { role: 'user', content: input };
        setMessages(prev => [...prev, userMessage]);
        setInput('');
        setIsLoading(true);

        try {
            // Get settings from Inspector (Theme/AI) - for now using defaults or we can fetch from store
            // We need to ensure we have an API key or a local Ollama running
            const response = await chatCompletion([...messages, userMessage], {
                provider: 'openrouter', // Defaulting to openrouter for now
                model: 'google/gemini-2.0-flash-001', // Appropriate for "Antigravity from google"
                apiKey: localStorage.getItem('peak-ai-key') || ''
            });

            setMessages(prev => [...prev, response]);
        } catch (error: any) {
            setMessages(prev => [...prev, {
                role: 'assistant',
                content: `Error: ${error.message}. Please check your AI settings in the Inspector.`
            }]);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <div className="flex flex-col h-full bg-zinc-50 dark:bg-zinc-950 text-foreground overflow-hidden">
            {/* Header / Intro */}
            <div className="p-6 border-b border-border/50 bg-white/50 dark:bg-zinc-900/50 backdrop-blur-md flex items-center justify-between">
                <div className="flex items-center gap-3">
                    <div className="w-10 h-10 rounded-2xl bg-gradient-to-br from-blue-500 via-purple-500 to-pink-500 flex items-center justify-center text-white shadow-lg shadow-purple-500/20 animate-pulse">
                        <Sparkles size={24} />
                    </div>
                    <div>
                        <h2 className="font-bold text-lg leading-tight">Antigravity</h2>
                        <p className="text-xs text-muted-foreground font-medium flex items-center gap-1">
                            <span className="w-1.5 h-1.5 rounded-full bg-green-500" />
                            Powered by Google Deepmind
                        </p>
                    </div>
                </div>
                <div className="flex gap-2">
                    <div className="p-2 rounded-xl bg-black/5 dark:bg-white/5 text-muted-foreground hover:text-foreground transition-colors cursor-help" title="Neural Link Active">
                        <Brain size={18} />
                    </div>
                    <div className="p-2 rounded-xl bg-black/5 dark:bg-white/5 text-muted-foreground hover:text-foreground transition-colors cursor-help" title="Advanced Reasoning">
                        <Cpu size={18} />
                    </div>
                </div>
            </div>

            {/* Chat Area */}
            <div
                ref={scrollRef}
                className="flex-1 overflow-y-auto p-6 space-y-6 scroll-smooth custom-scrollbar"
            >
                {messages.filter(m => m.role !== 'system').map((msg, i) => (
                    <div
                        key={i}
                        className={cn(
                            "flex gap-4 max-w-[85%]",
                            msg.role === 'user' ? "ml-auto flex-row-reverse" : "mr-auto"
                        )}
                    >
                        <div className={cn(
                            "w-8 h-8 rounded-lg flex items-center justify-center shrink-0 shadow-sm",
                            msg.role === 'user' ? "bg-zinc-200 dark:bg-zinc-800" : "bg-gradient-to-br from-blue-500 to-purple-600 text-white"
                        )}>
                            {msg.role === 'user' ? <User size={16} /> : <Bot size={16} />}
                        </div>
                        <div className={cn(
                            "p-4 rounded-2xl text-sm leading-relaxed shadow-sm border",
                            msg.role === 'user'
                                ? "bg-white dark:bg-zinc-900 border-border/40 rounded-tr-none"
                                : "bg-white dark:bg-zinc-900 border-border/40 rounded-tl-none"
                        )}>
                            <div className="prose prose-sm dark:prose-invert max-w-none prose-p:leading-relaxed prose-pre:bg-zinc-100 dark:prose-pre:bg-zinc-800 prose-pre:border prose-pre:border-border/50">
                                <ReactMarkdown remarkPlugins={[remarkGfm]}>
                                    {msg.content || ''}
                                </ReactMarkdown>
                            </div>
                        </div>
                    </div>
                ))}
                {isLoading && (
                    <div className="flex gap-4 max-w-[85%] animate-pulse">
                        <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white shrink-0">
                            <Bot size={16} />
                        </div>
                        <div className="p-4 rounded-2xl bg-white dark:bg-zinc-900 border border-border/40 rounded-tl-none flex items-center gap-2">
                            <div className="flex gap-1">
                                <div className="w-1.5 h-1.5 rounded-full bg-zinc-300 dark:bg-zinc-700 animate-bounce" />
                                <div className="w-1.5 h-1.5 rounded-full bg-zinc-300 dark:bg-zinc-700 animate-bounce [animation-delay:0.2s]" />
                                <div className="w-1.5 h-1.5 rounded-full bg-zinc-300 dark:bg-zinc-700 animate-bounce [animation-delay:0.4s]" />
                            </div>
                            <span className="text-xs text-muted-foreground font-medium">Antigravity is thinking...</span>
                        </div>
                    </div>
                )}
            </div>

            {/* Input Area */}
            <div className="p-6 bg-white dark:bg-zinc-900/80 backdrop-blur-md border-t border-border/50">
                <div className="relative group">
                    <textarea
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        onKeyDown={(e) => {
                            if (e.key === 'Enter' && !e.shiftKey) {
                                e.preventDefault();
                                handleSend();
                            }
                        }}
                        placeholder="Ask Antigravity anything..."
                        className="w-full bg-zinc-100 dark:bg-zinc-800 border-transparent focus:border-purple-500/50 rounded-2xl py-4 pl-5 pr-14 text-sm focus:outline-none focus:ring-4 focus:ring-purple-500/5 transition-all resize-none shadow-inner"
                        rows={1}
                    />
                    <button
                        onClick={handleSend}
                        disabled={!input.trim() || isLoading}
                        className="absolute right-2 top-2 p-2.5 rounded-xl bg-gradient-to-r from-blue-600 to-purple-600 text-white shadow-lg shadow-purple-500/20 hover:scale-105 active:scale-95 disabled:opacity-50 disabled:grayscale transition-all"
                    >
                        {isLoading ? <Loader2 size={18} className="animate-spin" /> : <Send size={18} />}
                    </button>
                </div>
                <div className="mt-3 flex items-center justify-center gap-4">
                    <p className="text-[10px] text-muted-foreground uppercase tracking-widest font-bold opacity-50">
                        Peak AI Agent System v2.0
                    </p>
                </div>
            </div>
        </div>
    );
}
