import { useState, useEffect } from 'react';
import {
    Settings, User, Lock, Shield,
    Monitor, Globe, Link,
    Trash2, Search, Moon, Key,
    ChevronRight, Database, Zap, Users, Plus, Pin,
    Wifi, Loader2, Bell, Volume2, Clock, Accessibility,
    Layout, Image as ImageIcon, Sparkles, Smartphone, Keyboard,
    MousePointer2, MonitorPlay
} from 'lucide-react';
import { useOSStore } from '../stores/useOSStore';
import { useSystemLink } from '../hooks/useSystemLink';
import { cn } from '../lib/utils';

interface SettingsAppProps {
    initialSection?: string;
}

interface CategoryItem {
    id: string;
    name: string;
    icon: any;
    color: string;
    subtext?: string;
}

interface CategoryGroup {
    title: string | null;
    items: CategoryItem[];
}

export function SettingsApp({ initialSection = 'profile' }: SettingsAppProps) {
    const { user, users, addUser, logout, resetSetup, theme, setTheme } = useOSStore();
    const { isConnected } = useSystemLink();
    const [activeSection, setActiveSection] = useState(initialSection);

    // New User Form State
    const [isAddingUser, setIsAddingUser] = useState(false);
    const [newUserName, setNewUserName] = useState('');
    const [newUserUsername, setNewUserUsername] = useState('');
    const [newUserAvatar] = useState('User');

    const [searchQuery, setSearchQuery] = useState('');
    const [history, setHistory] = useState<string[]>([initialSection]);
    const [historyIndex, setHistoryIndex] = useState(0);

    const categories: CategoryGroup[] = [
        {
            title: null,
            items: [
                { id: 'profile', name: user?.fullName || 'Peak User', subtext: 'Peak ID, iCloud+', icon: User, color: 'bg-gradient-to-br from-zinc-400 to-zinc-600' },
                { id: 'family', name: 'Family', icon: Users, color: 'bg-gradient-to-br from-sky-400 to-blue-600' },
            ]
        },
        {
            title: null,
            items: [
                { id: 'wifi', name: 'Wi-Fi', icon: Wifi, color: 'bg-blue-500' },
                { id: 'bluetooth', name: 'Bluetooth', icon: Zap, color: 'bg-blue-600' },
                { id: 'network', name: 'Network', icon: Globe, color: 'bg-blue-400' },
            ]
        },
        {
            title: null,
            items: [
                { id: 'notifications', name: 'Notifications', icon: Bell, color: 'bg-red-500' },
                { id: 'sound', name: 'Sound', icon: Volume2, color: 'bg-pink-500' },
                { id: 'focus', name: 'Focus', icon: Moon, color: 'bg-indigo-500' },
                { id: 'screen-time', name: 'Screen Time', icon: Clock, color: 'bg-indigo-600' },
            ]
        },
        {
            title: null,
            items: [
                { id: 'general', name: 'General', icon: Settings, color: 'bg-zinc-500' },
                { id: 'wallpaper', name: 'Wallpaper', icon: ImageIcon, color: 'bg-cyan-500' },
                { id: 'accessibility', name: 'Accessibility', icon: Accessibility, color: 'bg-blue-500' },
                { id: 'control-center', name: 'Control Center', icon: Layout, color: 'bg-zinc-500' },
                { id: 'shortcuts', name: 'Shortcuts', icon: Link, color: 'bg-indigo-500' },
            ]
        },
        {
            title: null,
            items: [
                { id: 'displays', name: 'Displays', icon: MonitorPlay, color: 'bg-blue-500' },
                { id: 'trackpad', name: 'Trackpad', icon: MousePointer2, color: 'bg-zinc-500' },
                { id: 'keyboard', name: 'Keyboard', icon: Keyboard, color: 'bg-zinc-500' },
                { id: 'mouse', name: 'Mouse', icon: MousePointer2, color: 'bg-zinc-500' },
            ]
        },
        {
            title: null,
            items: [
                { id: 'security', name: 'Privacy & Security', icon: Lock, color: 'bg-blue-500' },
                { id: 'users', name: 'Users & Groups', icon: Users, color: 'bg-zinc-500' },
                { id: 'passwords', name: 'Passwords', icon: Key, color: 'bg-zinc-600' },
            ]
        }
    ];

    const navigateTo = (id: string) => {
        const newHistory = history.slice(0, historyIndex + 1);
        newHistory.push(id);
        setHistory(newHistory);
        setHistoryIndex(newHistory.length - 1);
        setActiveSection(id);
    };

    const goBack = () => {
        if (historyIndex > 0) {
            setHistoryIndex(historyIndex - 1);
            setActiveSection(history[historyIndex - 1]);
        }
    };

    const goForward = () => {
        if (historyIndex < history.length - 1) {
            setHistoryIndex(historyIndex + 1);
            setActiveSection(history[historyIndex + 1]);
        }
    };

    const filteredCategories = categories.map(group => ({
        ...group,
        items: group.items.filter(item =>
            item.name.toLowerCase().includes(searchQuery.toLowerCase())
        )
    })).filter(group => group.items.length > 0);

    const activeItem = categories.flatMap(g => g.items).find(i => i.id === activeSection);

    return (
        <div className="flex h-full bg-background/80 backdrop-blur-3xl overflow-hidden rounded-xl border border-border flex-col md:flex-row text-foreground">
            {/* Sidebar */}
            <div className="w-full md:w-64 flex flex-col border-r border-border/40 bg-muted/30">
                {/* Window Traffic Lights Spacing */}
                <div className="h-12 flex-shrink-0" />

                {/* Search Bar */}
                <div className="px-4 mb-4">
                    <div className="relative group">
                        <Search size={13} className="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground/30 transition-colors group-focus-within:text-foreground/60" />
                        <input
                            type="text"
                            placeholder="Søk"
                            value={searchQuery}
                            onChange={(e) => setSearchQuery(e.target.value)}
                            className="w-full bg-foreground/5 border border-border/40 rounded-lg pl-8 pr-3 py-1.5 text-[12px] focus:outline-none focus:ring-1 ring-ring/20 placeholder:text-muted-foreground/20 transition-all"
                        />
                    </div>
                </div>

                <div className="flex-1 overflow-y-auto px-2 space-y-4 custom-scrollbar pb-10">
                    {/* Software Update Badge Mockup */}
                    <div className="px-2 mb-2">
                        <button
                            onClick={() => navigateTo('general')}
                            className="w-full flex items-center justify-between px-3 py-2 bg-foreground/5 hover:bg-foreground/10 rounded-md transition-all group"
                        >
                            <span className="text-[12px] text-muted-foreground/70">Oppdatering tilgjengelig</span>
                            <span className="w-5 h-5 bg-red-500 rounded-full flex items-center justify-center text-[10px] font-bold shadow-sm text-white group-hover:scale-105 transition-transform">1</span>
                        </button>
                    </div>

                    {filteredCategories.map((group, idx) => (
                        <div key={idx} className="space-y-0.5">
                            {group.items.map((item) => {
                                const Icon = item.icon;
                                const isActive = activeSection === item.id;
                                return (
                                    <button
                                        key={item.id}
                                        onClick={() => navigateTo(item.id)}
                                        className={cn(
                                            "w-full flex items-center gap-2.5 px-3 py-1.5 rounded-lg text-[13px] transition-all group relative",
                                            isActive
                                                ? "bg-blue-600 text-white shadow-lg"
                                                : "text-foreground/80 hover:bg-foreground/5"
                                        )}
                                    >
                                        <div className={cn(
                                            "w-6 h-6 rounded-md flex items-center justify-center text-white shadow-sm transition-transform group-hover:scale-105",
                                            item.color,
                                            isActive && "shadow-none"
                                        )}>
                                            <Icon size={14} fill={item.id === 'general' ? 'currentColor' : 'none'} className={isActive ? "text-white" : ""} />
                                        </div>
                                        <div className="flex flex-col items-start overflow-hidden flex-1">
                                            <span className={cn("truncate w-full text-left font-medium", isActive && "font-bold")}>{item.name}</span>
                                            {item.subtext && !isActive && (
                                                <span className="text-[10px] text-muted-foreground/40 truncate w-full text-left -mt-0.5">{item.subtext}</span>
                                            )}
                                        </div>
                                    </button>
                                );
                            })}
                        </div>
                    ))}
                </div>
            </div>

            {/* Content Area */}
            <div className="flex-1 flex flex-col min-w-0 bg-background/40">
                {/* Header */}
                <header className="h-12 flex items-center px-6 gap-4 flex-shrink-0">
                    <div className="flex items-center gap-1">
                        <button
                            onClick={goBack}
                            disabled={historyIndex === 0}
                            className="p-1.5 hover:bg-foreground/5 rounded-md disabled:opacity-20 transition-all active:scale-95"
                        >
                            <ChevronRight size={18} className="rotate-180" />
                        </button>
                        <button
                            onClick={goForward}
                            disabled={historyIndex === history.length - 1}
                            className="p-1.5 hover:bg-foreground/5 rounded-md disabled:opacity-20 transition-all active:scale-95"
                        >
                            <ChevronRight size={18} />
                        </button>
                    </div>
                    <h2 className="text-[14px] font-bold tracking-tight text-foreground/90 uppercase">
                        {activeItem?.name || 'Innstillinger'}
                    </h2>
                </header>

                <div className="flex-1 overflow-y-auto custom-scrollbar p-8">
                    <div className="max-w-2xl mx-auto space-y-8 animate-in fade-in slide-in-from-bottom-2 duration-300">
                        {activeSection === 'profile' && (
                            <div className="space-y-6">
                                <section className="bg-card/30 border border-border/40 rounded-2xl p-6 flex items-center gap-5 shadow-sm">
                                    <div className="w-16 h-16 bg-foreground/10 text-foreground rounded-full flex items-center justify-center shadow-lg relative group overflow-hidden border border-border/40">
                                        <User size={32} />
                                        <div className="absolute inset-0 bg-black/60 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity cursor-pointer">
                                            <Zap size={16} className="text-white" />
                                        </div>
                                    </div>
                                    <div className="space-y-0.5">
                                        <h2 className="text-lg font-bold tracking-tight">{user?.fullName || 'Peak User'}</h2>
                                        <p className="text-xs text-muted-foreground">@{user?.username || 'peakos'}</p>
                                        <div className="inline-flex items-center gap-1.5 px-2 py-0.5 bg-blue-500/10 text-blue-500 rounded text-[9px] font-bold uppercase mt-2 border border-blue-500/20">
                                            <Shield size={10} />
                                            Administrator
                                        </div>
                                    </div>
                                </section>

                                <SettingsGroup>
                                    <SettingsRow icon={User} iconColor="bg-blue-500" title="Personal Information" subtext="Name, phone, email" />
                                    <SettingsRow icon={Lock} iconColor="bg-amber-500" title="Sign-In & Security" subtext="Password, two-factor" />
                                    <SettingsRow icon={Database} iconColor="bg-pink-500" title="Cloud & Storage" subtext="1TB available" />
                                </SettingsGroup>
                            </div>
                        )}

                        {activeSection === 'wifi' && (
                            <div className="space-y-6">
                                <SettingsGroup>
                                    <div className="p-4 flex items-start gap-4">
                                        <div className="w-10 h-10 rounded-lg bg-blue-500 flex items-center justify-center text-white shadow-lg">
                                            <Wifi size={24} />
                                        </div>
                                        <div className="flex-1">
                                            <div className="flex items-center justify-between mb-1">
                                                <h3 className="text-[14px] font-bold">Wi-Fi</h3>
                                                <Toggle enabled={isConnected} onToggle={() => { }} />
                                            </div>
                                            <p className="text-[11px] text-muted-foreground leading-tight pr-10">
                                                Still inn Wi-Fi for å koble Macen trådløst til internett. Slå på Wi-Fi, og velg et nettverk du vil koble til. <span className="text-blue-500 cursor-pointer hover:underline font-medium">Finn ut mer...</span>
                                            </p>
                                        </div>
                                    </div>
                                </SettingsGroup>

                                {isConnected && (
                                    <SettingsGroup>
                                        <div className="p-3 flex items-center justify-between group cursor-pointer">
                                            <div className="flex items-center gap-3">
                                                <div className="flex flex-col items-center justify-center text-green-500">
                                                    <Wifi size={16} />
                                                    <div className="w-1.5 h-1.5 rounded-full bg-current mt-1" />
                                                </div>
                                                <div>
                                                    <p className="text-[13px] font-bold">Free your mandala 5G</p>
                                                    <p className="text-[11px] text-green-500/80">Tilkoblet</p>
                                                </div>
                                            </div>
                                            <div className="flex items-center gap-2">
                                                <Lock size={12} className="text-muted-foreground/20" />
                                                <Wifi size={14} className="text-muted-foreground/60" />
                                                <button className="px-3 py-1 bg-foreground/10 hover:bg-foreground/15 rounded-md text-[11px] font-medium transition-all">Detaljer...</button>
                                            </div>
                                        </div>
                                    </SettingsGroup>
                                )}

                                <SettingsGroup title="Delt internett">
                                    <SettingsRow
                                        title="vegar sin iPhone"
                                        right={<div className="flex items-center gap-2"><Lock size={12} className="text-muted-foreground/20" /><Link size={14} className="text-muted-foreground/40" /></div>}
                                    />
                                </SettingsGroup>

                                <SettingsGroup title="Kjente nettverk">
                                    <SettingsRow
                                        title="Free your mandala"
                                        right={<div className="flex items-center gap-2"><Lock size={12} className="text-muted-foreground/20" /><Wifi size={14} className="text-muted-foreground/40" /><button className="p-1 hover:bg-foreground/10 rounded-full"><Plus className="rotate-45" size={14} /></button></div>}
                                    />
                                    <div className="p-3 flex items-center justify-between hover:bg-foreground/5 transition-colors group cursor-pointer">
                                        <div className="flex items-center gap-3">
                                            <div className="w-4 flex justify-center text-blue-500">
                                                <div className="w-1 h-3 border-l-2 border-b-2 border-current -rotate-45 -translate-y-0.5" />
                                            </div>
                                            <p className="text-[13px] font-medium">Free your mandala 5G</p>
                                        </div>
                                        <div className="flex items-center gap-2">
                                            <Lock size={12} className="text-muted-foreground/20" /><Wifi size={14} className="text-muted-foreground/40" /><button className="p-1 hover:bg-foreground/10 rounded-full"><Plus className="rotate-45" size={14} /></button>
                                        </div>
                                    </div>
                                </SettingsGroup>

                                <SettingsGroup title="Andre nettverk">
                                    <NetworkPanel />
                                </SettingsGroup>
                            </div>
                        )}

                        {activeSection === 'network' && (
                            <div className="space-y-6">
                                <SettingsGroup>
                                    <SettingsRow
                                        icon={Wifi}
                                        iconColor="bg-blue-500"
                                        title="Wi-Fi"
                                        subtext="Tilkoblet"
                                        right={<div className="flex items-center gap-2"><div className="w-2 h-2 rounded-full bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.5)]" /><ChevronRight size={14} className="opacity-30" /></div>}
                                    />
                                    <SettingsRow
                                        icon={Shield}
                                        iconColor="bg-orange-500"
                                        title="Brannmur"
                                        subtext="Inaktiv"
                                        right={<div className="flex items-center gap-2"><div className="w-2 h-2 rounded-full bg-white/20" /><ChevronRight size={14} className="opacity-30" /></div>}
                                    />
                                </SettingsGroup>

                                <SettingsGroup title="Andre tjenester">
                                    <SettingsRow
                                        icon={Globe}
                                        iconColor="bg-zinc-600"
                                        title="USB 10/100/1000 LAN"
                                        subtext="Ikke tilkoblet"
                                        right={<div className="flex items-center gap-2"><div className="w-2 h-2 rounded-full bg-red-500/50" /><ChevronRight size={14} className="opacity-30" /></div>}
                                    />
                                    <SettingsRow
                                        icon={Zap}
                                        iconColor="bg-zinc-600"
                                        title="Thunderbolt-bro"
                                        subtext="Ikke tilkoblet"
                                        right={<div className="flex items-center gap-2"><div className="w-2 h-2 rounded-full bg-red-500/50" /><ChevronRight size={14} className="opacity-30" /></div>}
                                    />
                                    <SettingsRow
                                        icon={Smartphone}
                                        iconColor="bg-zinc-600"
                                        title="iPhone-USB"
                                        subtext="Ikke tilkoblet"
                                        right={<div className="flex items-center gap-2"><div className="w-2 h-2 rounded-full bg-red-500/50" /><ChevronRight size={14} className="opacity-30" /></div>}
                                    />
                                </SettingsGroup>
                            </div>
                        )}

                        {activeSection === 'battery' && (
                            <div className="space-y-6">
                                <SettingsGroup>
                                    <SettingsRow
                                        title="Spareblussmodus"
                                        right={<div className="flex items-center gap-2 text-[11px] text-white/40">Kun på batteri <ChevronRight size={14} className="rotate-90 opacity-30" /></div>}
                                    />
                                </SettingsGroup>

                                <SettingsGroup>
                                    <SettingsRow
                                        title="Batteritilstand"
                                        right={<div className="flex items-center gap-2 text-[11px] text-white/40">Normal <div className="w-5 h-5 rounded-full border border-white/10 flex items-center justify-center text-[12px] font-serif">i</div></div>}
                                    />
                                </SettingsGroup>

                                <div className="bg-card/20 border border-border/40 rounded-2xl p-6 space-y-6">
                                    <div className="flex items-center justify-between">
                                        <div className="flex p-0.5 bg-foreground/10 rounded-md">
                                            <button className="px-4 py-1.5 bg-blue-600 rounded-md text-[11px] font-bold shadow-sm text-white">Siste 24 timer</button>
                                            <button className="px-4 py-1.5 hover:bg-foreground/5 rounded-md text-[11px] font-bold transition-all text-foreground/60">Siste 10 dager</button>
                                        </div>
                                    </div>

                                    <div className="space-y-2">
                                        <div className="flex justify-between items-end h-32 gap-1 px-1">
                                            {Array.from({ length: 24 }).map((_, i) => (
                                                <div key={i} className="flex-1 flex flex-col items-center gap-1 group">
                                                    <div className="w-full bg-foreground/5 rounded-t-sm relative overflow-hidden" style={{ height: '100%' }}>
                                                        <div
                                                            className={cn(
                                                                "absolute bottom-0 w-full transition-all duration-1000",
                                                                i === 18 ? "bg-red-500" : "bg-green-500"
                                                            )}
                                                            style={{ height: `${Math.random() * 60 + 20}%` }}
                                                        />
                                                    </div>
                                                </div>
                                            ))}
                                        </div>
                                        <div className="flex justify-between text-[10px] text-muted-foreground/20 px-1">
                                            <span>18</span><span>21</span><span>00</span><span>03</span><span>06</span><span>09</span><span>12</span><span>15</span>
                                        </div>
                                    </div>

                                    <div className="space-y-1 pt-4">
                                        <h4 className="text-[11px] font-bold text-muted-foreground/50">Brukt med skjermen på</h4>
                                        <div className="flex justify-between items-end h-20 gap-1 px-1">
                                            {Array.from({ length: 24 }).map((_, i) => (
                                                <div key={i} className="flex-1 bg-blue-500/80 rounded-t-sm" style={{ height: `${Math.random() * 80}%` }} />
                                            ))}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        )}

                        {activeSection === 'general' && (
                            <div className="space-y-6">
                                <div className="flex flex-col items-center py-6 gap-2">
                                    <div className="w-16 h-16 rounded-2xl bg-zinc-600 flex items-center justify-center text-white shadow-xl">
                                        <Settings size={40} />
                                    </div>
                                    <h3 className="text-[20px] font-bold">Generelt</h3>
                                    <p className="text-[11px] text-muted-foreground/60 text-center max-w-[280px]">
                                        Administrer oppsettet og valgene for Macen, som programvareoppdateringer, lagring på enheten, AirDrop og annet.
                                    </p>
                                </div>

                                <SettingsGroup>
                                    <SettingsRow icon={Monitor} iconColor="bg-zinc-500" title="Om" />
                                    <SettingsRow icon={Settings} iconColor="bg-zinc-500" title="Programvareoppdatering" />
                                    <SettingsRow icon={Database} iconColor="bg-zinc-500" title="Lagring" />
                                </SettingsGroup>

                                <SettingsGroup>
                                    <SettingsRow icon={Shield} iconColor="bg-red-500" title="AppleCare og garanti" />
                                </SettingsGroup>

                                <SettingsGroup>
                                    <SettingsRow icon={Globe} iconColor="bg-blue-500" title="AirDrop og Handoff" />
                                </SettingsGroup>

                                <SettingsGroup>
                                    <SettingsRow icon={Key} iconColor="bg-zinc-600" title="Autoutfylling og passord" />
                                    <SettingsRow icon={Clock} iconColor="bg-blue-600" title="Dato og tid" />
                                    <SettingsRow icon={Zap} iconColor="bg-zinc-500" title="Deling" />
                                </SettingsGroup>
                            </div>
                        )}

                        {activeSection === 'appearance' && (
                            <div className="space-y-6">
                                <SettingsGroup title="Utseende">
                                    <div className="grid grid-cols-3 gap-6 p-6">
                                        {[
                                            { id: 'light', name: 'Lyst', bg: 'bg-zinc-100', border: theme === 'light' ? 'border-blue-500' : 'border-zinc-300 dark:border-zinc-700' },
                                            { id: 'dark', name: 'Mørkt', bg: 'bg-zinc-900', border: theme === 'dark' ? 'border-blue-500' : 'border-zinc-300 dark:border-zinc-700' },
                                            { id: 'auto', name: 'Auto', bg: 'bg-gradient-to-r from-zinc-100 to-zinc-900', border: theme === 'system' ? 'border-blue-500' : 'border-zinc-300 dark:border-zinc-700' }
                                        ].map(m => (
                                            <div
                                                key={m.id}
                                                onClick={() => setTheme(m.id as any)}
                                                className="space-y-2 cursor-pointer group flex flex-col items-center"
                                            >
                                                <div className={cn(
                                                    "w-full aspect-[4/3] rounded-xl border-4 transition-all group-hover:scale-[1.02]",
                                                    m.border, m.bg,
                                                    ((m.id === 'dark' && theme === 'dark') || (m.id === 'light' && theme === 'light') || (m.id === 'auto' && theme === 'system')) ? "shadow-[0_0_15px_rgba(59,130,246,0.3)]" : ""
                                                )} />
                                                <p className="text-[11px] font-medium text-foreground/80">{m.name}</p>
                                            </div>
                                        ))}
                                    </div>
                                </SettingsGroup>

                                <SettingsGroup title="Aksentfarge">
                                    <div className="p-4 flex items-center justify-between">
                                        <div className="flex gap-2.5">
                                            {['blue', 'pink', 'red', 'orange', 'yellow', 'green', 'purple', 'gray'].map(color => (
                                                <button
                                                    key={color}
                                                    className={cn(
                                                        "w-5 h-5 rounded-full border-2 border-border/20 hover:scale-110 transition-transform",
                                                        `bg-${color}-500`,
                                                        color === 'blue' && "border-white ring-2 ring-blue-500/50"
                                                    )}
                                                />
                                            ))}
                                        </div>
                                    </div>
                                </SettingsGroup>

                                <SettingsGroup>
                                    <SettingsRow title="Markørfarge" right={<div className="flex gap-1 items-center bg-foreground/5 px-2 py-1 rounded text-[10px] text-muted-foreground/60">Multifarge <ChevronRight size={12} className="rotate-90" /></div>} />
                                </SettingsGroup>
                            </div>
                        )}

                        {activeSection === 'security' && (
                            <div className="space-y-6">
                                <SettingsGroup title="Privacy">
                                    <SettingsRow icon={Shield} iconColor="bg-blue-500" title="Location Services" right={<span className="text-[11px] opacity-40">On</span>} />
                                    <SettingsRow icon={Monitor} iconColor="bg-blue-400" title="Tracking" right={<ChevronRight size={14} className="opacity-30" />} />
                                </SettingsGroup>
                                <SettingsGroup title="Security">
                                    <SettingsRow icon={Lock} iconColor="bg-zinc-700" title="FileVault" subtext="Encrypted" right={<span className="text-[11px] opacity-40">On</span>} />
                                    <SettingsRow icon={Shield} iconColor="bg-amber-500" title="Firewall" right={<span className="text-[11px] opacity-40">Off</span>} />
                                </SettingsGroup>
                            </div>
                        )}

                        {activeSection === 'users' && (
                            <div className="space-y-6">
                                <div className="flex items-center justify-between">
                                    <h3 className="text-[11px] font-bold uppercase tracking-widest text-muted-foreground px-1">Users & Accounts</h3>
                                    <button
                                        onClick={() => setIsAddingUser(true)}
                                        className="text-[10px] text-blue-500 font-bold hover:underline"
                                    >
                                        Add Account...
                                    </button>
                                </div>

                                <SettingsGroup>
                                    {users.map(u => (
                                        <SettingsRow
                                            key={u.username}
                                            icon={User}
                                            iconColor={u.username === user?.username ? "bg-blue-500" : "bg-zinc-500"}
                                            title={u.fullName}
                                            subtext={u.username === user?.username ? "Current User, Admin" : `@${u.username}`}
                                            right={u.username !== user?.username && <Trash2 size={13} className="text-red-500/50 hover:text-red-500 cursor-pointer" />}
                                        />
                                    ))}
                                </SettingsGroup>

                                {isAddingUser && (
                                    <div className="bg-card/20 border border-indigo-500/20 rounded-2xl p-6 space-y-4 animate-in zoom-in-95 duration-200">
                                        <div className="flex items-center justify-between">
                                            <h3 className="text-sm font-bold">New Account</h3>
                                            <button onClick={() => setIsAddingUser(false)} className="text-[10px] text-muted-foreground hover:text-foreground">Cancel</button>
                                        </div>
                                        <div className="grid grid-cols-2 gap-3">
                                            <input
                                                type="text"
                                                placeholder="Full Name"
                                                value={newUserName}
                                                onChange={e => setNewUserName(e.target.value)}
                                                className="bg-black/20 border border-white/5 rounded-lg px-3 py-1.5 text-[11px] focus:outline-none"
                                            />
                                            <input
                                                type="text"
                                                placeholder="Username"
                                                value={newUserUsername}
                                                onChange={e => setNewUserUsername(e.target.value)}
                                                className="bg-black/20 border border-white/5 rounded-lg px-3 py-1.5 text-[11px] focus:outline-none"
                                            />
                                        </div>
                                        <button
                                            onClick={() => {
                                                if (newUserName && newUserUsername) {
                                                    addUser({ fullName: newUserName, username: newUserUsername.toLowerCase(), avatar: newUserAvatar });
                                                    setIsAddingUser(false);
                                                    setNewUserName('');
                                                    setNewUserUsername('');
                                                }
                                            }}
                                            className="w-full bg-blue-500 text-white rounded-lg py-1.5 text-[11px] font-bold"
                                        >
                                            Create User
                                        </button>
                                    </div>
                                )}
                            </div>
                        )}

                        {activeSection === 'shortcuts' && (
                            <div className="space-y-6">
                                <SettingsGroup title="Dock & Sidebar Shortcuts">
                                    <ShortcutsManager />
                                </SettingsGroup>
                            </div>
                        )}

                        {activeSection === 'passwords' && (
                            <MockSection id="passwords" />
                        )}

                        {(!['wifi', 'bluetooth', 'network', 'battery', 'general', 'appearance', 'users', 'shortcuts', 'passwords', 'profile', 'security'].includes(activeSection)) && (
                            <MockSection id={activeSection} />
                        )}

                        {/* Emergency Recovery Footer */}
                        <div className="pt-12 pb-8 border-t border-border/20 space-y-4">
                            <h3 className="text-[10px] font-bold uppercase tracking-widest text-red-500/50 px-1">Danger Zone</h3>
                            <div className="flex gap-3">
                                <button
                                    onClick={logout}
                                    className="px-4 py-1.5 bg-red-500/10 hover:bg-red-500/20 text-red-500 rounded-lg text-[11px] font-bold transition-all"
                                >
                                    Log Out...
                                </button>
                                <button
                                    onClick={() => { if (confirm('REALLY reset system?')) resetSetup(); }}
                                    className="px-4 py-1.5 bg-muted/20 hover:bg-red-500 hover:text-white rounded-lg text-[11px] font-medium transition-all"
                                >
                                    Factory Reset
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

function SettingsGroup({ title, children }: { title?: string, children: React.ReactNode }) {
    return (
        <div className="space-y-1.5">
            {title && <h3 className="text-[11px] font-bold uppercase tracking-widest text-muted-foreground/50 px-1">{title}</h3>}
            <div className="bg-card/20 border border-border/40 rounded-2xl overflow-hidden divide-y divide-border/20 shadow-sm">
                {children}
            </div>
        </div>
    );
}

function SettingsRow({ icon: Icon, iconColor, title, subtext, right }: { icon?: any, iconColor?: string, title: string, subtext?: string, right?: React.ReactNode }) {
    return (
        <div className="flex items-center justify-between p-3 hover:bg-foreground/5 transition-colors group cursor-pointer">
            <div className="flex items-center gap-3">
                {Icon && (
                    <div className={cn("w-7 h-7 rounded-lg flex items-center justify-center text-white shadow-sm", iconColor || "bg-zinc-500")}>
                        <Icon size={16} />
                    </div>
                )}
                <div>
                    <p className="text-[12px] font-medium leading-none mb-1">{title}</p>
                    {subtext && <p className="text-[10px] text-muted-foreground leading-none">{subtext}</p>}
                </div>
            </div>
            <div className="flex items-center gap-3">
                {right}
            </div>
        </div>
    );
}

function Toggle({ enabled, onToggle }: { enabled: boolean, onToggle: () => void }) {
    return (
        <button
            onClick={(e) => { e.stopPropagation(); onToggle(); }}
            className={cn(
                "w-9 h-5 rounded-full transition-all relative",
                enabled ? "bg-green-500 shadow-[0_0_10px_rgba(34,197,94,0.3)]" : "bg-muted-foreground/20"
            )}
        >
            <div className={cn(
                "absolute top-0.5 w-4 h-4 bg-white rounded-full transition-all shadow-sm",
                enabled ? "left-4.5" : "left-0.5"
            )} />
        </button>
    );
}

function MockSection({ id }: { id: string }) {
    const mockData: Record<string, { icon: any, color: string, rows: any[] }> = {
        bluetooth: {
            icon: Zap, color: 'bg-blue-600',
            rows: [
                { title: 'Bluetooth', right: <Toggle enabled={true} onToggle={() => { }} /> },
                { title: 'AirPods Pro', subtext: 'Connected', right: <div className="flex gap-1 items-center bg-green-500/10 text-green-500 px-1.5 py-0.5 rounded text-[8px] font-bold uppercase">Active</div> },
                { title: 'Magic Keyboard', subtext: 'Connected', right: <span className="text-[10px] opacity-30 font-mono">82%</span> }
            ]
        },
        notifications: {
            icon: Bell, color: 'bg-red-500',
            rows: [
                { title: 'Allow Notifications', right: <Toggle enabled={true} onToggle={() => { }} /> },
                { title: 'Show Previews', subtext: 'When Unlocked', right: <ChevronRight size={14} className="opacity-30" /> },
                { title: 'Browser', subtext: 'Banners, Sounds', right: <ChevronRight size={14} className="opacity-30" /> }
            ]
        },
        sound: {
            icon: Volume2, color: 'bg-pink-500',
            rows: [
                { title: 'Output Volume', right: <div className="w-32 h-1 bg-white/10 rounded-full relative overflow-hidden"><div className="absolute inset-y-0 left-0 bg-white w-2/3" /></div> },
                { title: 'Output Device', subtext: 'MacBook Pro Speakers', right: <ChevronRight size={14} className="opacity-30" /> },
                { title: 'Play sound on startup', right: <Toggle enabled={true} onToggle={() => { }} /> }
            ]
        },
        focus: {
            icon: Moon, color: 'bg-indigo-500',
            rows: [
                { title: 'Do Not Disturb', subtext: 'Off', right: <ChevronRight size={14} className="opacity-30" /> },
                { title: 'Work', subtext: '9:00 AM - 5:00 PM', right: <ChevronRight size={14} className="opacity-30" /> },
                { title: 'Share Across Devices', right: <Toggle enabled={true} onToggle={() => { }} /> }
            ]
        },
        'screen-time': {
            icon: Clock, color: 'bg-indigo-600',
            rows: [
                { title: 'Screen Time', subtext: '3h 42m today', right: <ChevronRight size={14} className="opacity-30" /> },
                { title: 'App Limits', subtext: 'None', right: <ChevronRight size={14} className="opacity-30" /> }
            ]
        },
        general: {
            icon: Settings, color: 'bg-zinc-500',
            rows: [
                { title: 'About', right: <span className="text-[11px] opacity-40">PeakOS 1.0.0</span> },
                { title: 'Software Update', subtext: 'Up to date', right: <ChevronRight size={14} className="opacity-30" /> },
                { title: 'Storage', subtext: '42.5 GB of 256 GB used', right: <ChevronRight size={14} className="opacity-30" /> },
                { title: 'AirDrop', subtext: 'Contacts Only', right: <ChevronRight size={14} className="opacity-30" /> }
            ]
        },
        accessibility: {
            icon: Accessibility, color: 'bg-blue-500',
            rows: [
                { title: 'Vision', subtext: 'VoiceOver, Zoom, Display', right: <ChevronRight size={14} className="opacity-30" /> },
                { title: 'Hearing', subtext: 'Background Sounds', right: <ChevronRight size={14} className="opacity-30" /> }
            ]
        },
        'control-center': {
            icon: Layout, color: 'bg-zinc-500',
            rows: [
                { title: 'Wi-Fi', subtext: 'Show in Menu Bar', right: <Toggle enabled={true} onToggle={() => { }} /> },
                { title: 'Bluetooth', subtext: 'Show in Menu Bar', right: <Toggle enabled={true} onToggle={() => { }} /> },
                { title: 'Battery', subtext: 'Show Percentage', right: <Toggle enabled={false} onToggle={() => { }} /> }
            ]
        },
        wallpaper: {
            icon: ImageIcon, color: 'bg-cyan-500',
            rows: [
                { title: 'Peak Dusk', subtext: 'Dynamic Wallpaper', right: <div className="w-10 h-6 rounded bg-gradient-to-br from-indigo-900 to-purple-800" /> },
                { title: 'Auto-rotate wallpapers', right: <Toggle enabled={true} onToggle={() => { }} /> }
            ]
        },
        displays: {
            icon: MonitorPlay, color: 'bg-blue-500',
            rows: [
                { title: 'Resolution', subtext: 'Default for display', right: <ChevronRight size={14} className="opacity-30" /> },
                { title: 'Brightness', right: <div className="w-32 h-1 bg-white/10 rounded-full relative overflow-hidden"><div className="absolute inset-y-0 left-0 bg-white w-4/5" /></div> },
                { title: 'Night Shift', subtext: 'Off', right: <ChevronRight size={14} className="opacity-30" /> }
            ]
        },
        storage: {
            icon: Database, color: 'bg-pink-500',
            rows: [
                { title: 'System Data', subtext: '12.4 GB', right: <div className="w-24 h-1.5 bg-zinc-800 rounded-full overflow-hidden"><div className="h-full bg-pink-500 w-1/4" /></div> },
                { title: 'Optimize Storage', right: <Toggle enabled={true} onToggle={() => { }} /> }
            ]
        },
        passwords: {
            icon: Key, color: 'bg-zinc-600',
            rows: [
                { title: 'Password Options', subtext: 'AutoFill, Keychain', right: <ChevronRight size={14} className="opacity-30" /> },
                { title: 'Google.com', subtext: 'jane.doe@gmail.com', right: <Lock size={12} className="opacity-20" /> }
            ]
        }
    };

    const data = mockData[id] || { icon: Sparkles, color: 'bg-zinc-500', rows: [{ title: 'Feature coming soon', subtext: 'This module is in the technical roadmap.' }] };

    return (
        <div className="space-y-6">
            <SettingsGroup>
                {data.rows.map((row, i) => (
                    <SettingsRow
                        key={i}
                        icon={row.icon}
                        iconColor={row.iconColor}
                        title={row.title}
                        subtext={row.subtext}
                        right={row.right || <ChevronRight size={14} className="opacity-30" />}
                    />
                ))}
            </SettingsGroup>
        </div>
    );
}

function NetworkPanel() {
    const { callTool, isConnected } = useSystemLink();
    const [networks, setNetworks] = useState<any[]>([]);
    const [isScanning, setIsScanning] = useState(false);

    const scan = async () => {
        if (!isConnected || isScanning) return;
        setIsScanning(true);
        try {
            const result = await callTool('scan_wifi', {});
            if (Array.isArray(result)) setNetworks(result);
        } catch (e) {
            console.error('Scan failed', e);
        } finally {
            setIsScanning(false);
        }
    };

    useEffect(() => {
        scan();
    }, [isConnected]);

    return (
        <div className="space-y-1.5 max-h-64 overflow-y-auto pr-1 select-text">
            {networks.length === 0 && !isScanning && (
                <div className="py-6 text-center text-[11px] text-muted-foreground bg-black/5 rounded-xl border border-dashed border-white/5">
                    No networks found.
                </div>
            )}
            {networks.map((net: any, i: number) => (
                <div key={i} className="flex items-center justify-between p-3 hover:bg-white/5 transition-all group border-b border-white/5 last:border-0">
                    <div className="flex items-center gap-2.5">
                        <Wifi size={14} className="text-muted-foreground/60 group-hover:text-primary transition-colors" />
                        <span className="text-[11px] font-medium">{net.ssid || 'Unknown'}</span>
                    </div>
                    <div className="flex items-center gap-3 text-[10px] text-muted-foreground">
                        <span>{net.security || 'Open'}</span>
                        <span className="w-8 text-right font-mono">{net.signal}%</span>
                    </div>
                </div>
            ))}
            {isScanning && (
                <div className="py-6 flex items-center justify-center gap-2 text-[11px] text-muted-foreground">
                    <Loader2 size={12} className="animate-spin" />
                    Scanning...
                </div>
            )}
        </div>
    );
}

function ShortcutsManager() {
    const { shortcuts, updateShortcut, removeShortcut, addShortcut } = useOSStore();
    const [isAdding, setIsAdding] = useState(false);
    const [newName, setNewName] = useState('');
    const [newUrl, setNewUrl] = useState('');

    const handleAddCustom = () => {
        if (!newName || !newUrl) return;
        addShortcut({
            id: `custom-${Date.now()}`,
            name: newName,
            icon: 'globe',
            type: 'url',
            target: newUrl,
            pinnedToDock: true,
            pinnedToSidebar: true
        });
        setNewName('');
        setNewUrl('');
        setIsAdding(false);
    };

    return (
        <div className="space-y-4">
            <div className="flex items-center justify-between">
                <h3 className="text-sm font-bold">Active Shortcuts</h3>
                <button
                    onClick={() => setIsAdding(true)}
                    className="flex items-center gap-1.5 bg-pink-500/90 hover:bg-pink-500 text-white px-2.5 py-1 rounded-lg text-[10px] font-bold transition-all shadow-sm active:scale-95"
                >
                    <Plus size={12} />
                    Add Link
                </button>
            </div>

            {isAdding && (
                <div className="bg-secondary/10 border border-pink-500/20 rounded-2xl p-5 space-y-3 animate-in fade-in zoom-in-95 duration-200">
                    <div className="grid grid-cols-2 gap-3">
                        <div className="space-y-1">
                            <label className="text-[9px] font-bold uppercase tracking-widest text-muted-foreground ml-1">Label</label>
                            <input
                                type="text"
                                placeholder="Google"
                                value={newName}
                                onChange={e => setNewName(e.target.value)}
                                className="w-full bg-background/50 border border-border/10 rounded-lg px-3 py-1.5 text-[11px] focus:outline-none focus:ring-1 ring-pink-500/30"
                            />
                        </div>
                        <div className="space-y-1">
                            <label className="text-[9px] font-bold uppercase tracking-widest text-muted-foreground ml-1">URL</label>
                            <input
                                type="text"
                                placeholder="https://..."
                                value={newUrl}
                                onChange={e => setNewUrl(e.target.value)}
                                className="w-full bg-background/50 border border-border/10 rounded-lg px-3 py-1.5 text-[11px] focus:outline-none focus:ring-1 ring-pink-500/30"
                            />
                        </div>
                    </div>
                    <div className="flex justify-end gap-2">
                        <button onClick={() => setIsAdding(false)} className="px-3 py-1 text-[10px] font-bold text-muted-foreground hover:text-foreground">Cancel</button>
                        <button onClick={handleAddCustom} className="bg-pink-500 text-white px-4 py-1.5 rounded-lg text-[11px] font-bold hover:brightness-110 active:scale-95 transition-all">Add Shortcut</button>
                    </div>
                </div>
            )}

            <div className="grid grid-cols-1 gap-2">
                {shortcuts.length === 0 && (
                    <div className="py-10 text-center text-[11px] text-muted-foreground bg-secondary/5 rounded-2xl border border-dashed border-white/5">
                        <Link size={24} className="mx-auto mb-2 opacity-20" />
                        <p>No shortcuts yet.</p>
                    </div>
                )}
                {shortcuts.map(s => (
                    <div key={s.id} className="flex items-center justify-between p-3 bg-secondary/5 rounded-xl border border-white/5 hover:bg-secondary/10 transition-all group">
                        <div className="flex items-center gap-3">
                            <div className="w-8 h-8 bg-background/50 rounded-lg flex items-center justify-center text-muted-foreground shadow-sm group-hover:scale-105 transition-transform border border-white/5">
                                <Globe size={16} />
                            </div>
                            <div className="overflow-hidden">
                                <p className="font-bold text-[11px] truncate">{s.name}</p>
                                <p className="text-[9px] text-muted-foreground truncate max-w-[120px]">{s.type === 'url' ? s.target : 'App'}</p>
                            </div>
                        </div>

                        <div className="flex items-center gap-1.5">
                            <button
                                onClick={() => updateShortcut(s.id, { pinnedToSidebar: !s.pinnedToSidebar })}
                                className={cn(
                                    "p-1.5 rounded-md transition-all flex items-center gap-1 text-[9px] font-bold uppercase",
                                    s.pinnedToSidebar ? "bg-blue-500/10 text-blue-500 border border-blue-500/10" : "bg-muted text-muted-foreground hover:bg-foreground/5"
                                )}
                            >
                                <Pin size={10} />
                                <span>Side</span>
                            </button>
                            <button
                                onClick={() => updateShortcut(s.id, { pinnedToDock: !s.pinnedToDock })}
                                className={cn(
                                    "p-1.5 rounded-md transition-all flex items-center gap-1 text-[9px] font-bold uppercase",
                                    s.pinnedToDock ? "bg-purple-500/10 text-purple-500 border border-purple-500/10" : "bg-muted text-muted-foreground hover:bg-foreground/5"
                                )}
                            >
                                <Pin size={10} />
                                <span>Dock</span>
                            </button>
                            <button
                                onClick={() => removeShortcut(s.id)}
                                className="p-1.5 text-red-500/40 hover:text-red-500 hover:bg-red-500/10 rounded-md transition-all"
                            >
                                <Trash2 size={13} />
                            </button>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
}
