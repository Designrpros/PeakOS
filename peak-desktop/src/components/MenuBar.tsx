import { useState, useEffect, useRef } from 'react';
import { Wifi, Search, Command, Sparkles, Settings, WifiOff, Loader2, Lock } from 'lucide-react';
import { format } from 'date-fns';
import { InspectorMode } from './Inspector';
import { useSystemLink } from '../hooks/useSystemLink';
import { useOSStore } from '../stores/useOSStore';

interface MenuBarProps {
    showSidebar?: boolean;
    setShowSidebar?: (show: boolean) => void;
    rightPanelMode?: InspectorMode | null;
    setRightPanelMode?: (mode: InspectorMode | null) => void;
    onToggleOverview?: () => void;
}

interface WifiNetwork {
    ssid: string;
    signal: number;
    security: string;
}

export function MenuBar({ showSidebar, setShowSidebar, rightPanelMode, setRightPanelMode, onToggleOverview }: MenuBarProps) {
    const [time, setTime] = useState(new Date());
    const [showWifi, setShowWifi] = useState(false);
    const [networks, setNetworks] = useState<WifiNetwork[]>([]);
    const [isScanning, setIsScanning] = useState(false);
    const [connectingSsid, setConnectingSsid] = useState<string | null>(null);
    const { callTool, isConnected } = useSystemLink();
    const { launchApp, toggleSpotlight, isSpotlightOpen } = useOSStore();
    const wifiRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const timer = setInterval(() => setTime(new Date()), 1000);
        return () => clearInterval(timer);
    }, []);

    const scanWifi = async () => {
        if (!isConnected || isScanning) return;
        setIsScanning(true);
        try {
            // Artificial delay for feel
            await new Promise(r => setTimeout(r, 800));
            const result = await callTool('scan_wifi', {});

            if (result && Array.isArray(result)) {
                setNetworks(result as WifiNetwork[]);
            } else if (result && typeof result === 'object' && 'status' in result && (result as any).status === 'error') {
                console.error('Wifi scan backend error:', (result as any).message);
                setNetworks([]);
            }
        } catch (e) {
            console.error('Wifi scan failed', e);
        } finally {
            setIsScanning(false);
        }
    };

    const connectToWifi = async (ssid: string) => {
        const password = prompt(`Enter password for ${ssid}:`);
        if (!password) return;

        setConnectingSsid(ssid);
        try {
            const result = await callTool('connect_wifi', { ssid, password });
            console.log('Connect result:', result);
            alert(JSON.stringify(result));
        } catch (e) {
            console.error('Wifi connect failed', e);
            alert('Failed to connect to WiFi');
        } finally {
            setConnectingSsid(null);
        }
    };

    useEffect(() => {
        if (showWifi) scanWifi();
    }, [showWifi]);

    return (
        <header className="w-full h-8 bg-background/40 backdrop-blur-xl flex items-center justify-between px-4 text-foreground/90 text-sm z-50 select-none border-b border-black/5 dark:border-white/10 transition-colors duration-300 flex-none">
            <div className="flex items-center space-x-4">
                <div
                    className="flex items-center gap-2 font-bold hover:opacity-80 transition-opacity cursor-pointer"
                    onClick={() => setShowSidebar?.(!showSidebar)}
                    title="Toggle Sidebar"
                >
                    <img src="/src/assets/peak-logo-small.png" alt="PeakOS" className="w-5 h-5 object-contain dark:invert" />
                    <div className="font-medium hidden sm:block">PeakOS</div>
                </div>

                <div className="hidden sm:flex space-x-3 text-foreground/80">
                    <span className="hover:text-foreground cursor-pointer transition-colors">File</span>
                    <span className="hover:text-foreground cursor-pointer transition-colors">Edit</span>
                    <span className="hover:text-foreground cursor-pointer transition-colors">View</span>
                    <span className="hover:text-foreground cursor-pointer transition-colors">Window</span>
                    <span className="hover:text-foreground cursor-pointer transition-colors">Help</span>
                </div>
            </div>

            <div className="flex items-center space-x-4">
                <div className="flex items-center space-x-3 text-foreground/80">

                    <button
                        onClick={toggleSpotlight}
                        className={`p-1 rounded hover:bg-black/5 dark:hover:bg-white/10 transition-colors ${isSpotlightOpen ? 'text-amber-500' : ''}`}
                        title="Spotlight Search"
                    >
                        <Search size={16} />
                    </button>

                    {/* WiFi Selector */}
                    <div className="relative" ref={wifiRef}>
                        <button
                            onClick={() => setShowWifi(!showWifi)}
                            className={`p-1 rounded hover:bg-black/5 dark:hover:bg-white/10 transition-colors ${showWifi ? 'text-amber-500' : ''}`}
                        >
                            {isConnected ? <Wifi size={16} /> : <WifiOff size={16} className="text-red-500" />}
                        </button>

                        {showWifi && (
                            <div className="absolute right-0 mt-1 w-64 bg-white dark:bg-zinc-900 border border-black/10 dark:border-white/10 rounded-lg shadow-2xl backdrop-blur-2xl overflow-hidden py-1 z-50">
                                <div className="px-3 py-2 text-xs font-semibold text-zinc-500 flex justify-between items-center bg-black/5 dark:bg-white/5">
                                    <span>WIFI NETWORKS</span>
                                    {isScanning && <Loader2 size={12} className="animate-spin text-amber-500" />}
                                </div>
                                <div className="max-h-64 overflow-y-auto">
                                    {networks.length === 0 && !isScanning && (
                                        <div className="px-3 py-4 text-center text-xs text-zinc-400">No networks found</div>
                                    )}
                                    {networks.map((net, i) => (
                                        <button
                                            key={i}
                                            onClick={() => connectToWifi(net.ssid)}
                                            disabled={connectingSsid === net.ssid}
                                            className="w-full flex items-center justify-between px-3 py-1.5 hover:bg-amber-500 hover:text-white transition-colors group disabled:opacity-50"
                                        >
                                            <div className="flex items-center gap-2">
                                                {connectingSsid === net.ssid ? (
                                                    <Loader2 size={14} className="animate-spin" />
                                                ) : (
                                                    <Wifi size={14} />
                                                )}
                                                <span className="truncate max-w-[140px]">{net.ssid || 'Unknown Network'}</span>
                                            </div>
                                            <div className="flex items-center gap-2 text-[10px] opacity-60 group-hover:opacity-100">
                                                {net.security.length > 0 && <Lock size={10} />}
                                                <span>{net.signal}%</span>
                                            </div>
                                        </button>
                                    ))}
                                </div>
                                <div className="border-t border-black/5 dark:border-white/5 mt-1 px-1 py-1">
                                    <button
                                        onClick={() => {
                                            setShowWifi(false);
                                            launchApp('settings', { type: 'settings', title: 'System Settings', initialSection: 'network' });
                                        }}
                                        className="w-full text-left px-2 py-1.5 rounded hover:bg-black/5 dark:hover:bg-white/10 text-xs"
                                    >
                                        WiFi Settings...
                                    </button>
                                </div>
                            </div>
                        )}
                    </div>

                    {/* Inspector Toggle */}
                    <button
                        onClick={() => setRightPanelMode?.(rightPanelMode === 'chat' ? null : 'chat')}
                        className={`flex items-center space-x-1 pl-2 pr-1 rounded hover:bg-black/5 dark:hover:bg-white/10 transition-colors ${rightPanelMode === 'chat' ? 'text-amber-500' : 'text-muted-foreground'}`}
                        title="Toggle Intelligence"
                    >
                        <Sparkles size={16} />
                    </button>

                    {/* Mission Control / Overview Button */}
                    <button
                        onClick={onToggleOverview}
                        className="p-1 rounded hover:bg-black/5 dark:hover:bg-white/10 transition-colors text-foreground"
                        title="Mission Control"
                    >
                        <Command size={16} />
                    </button>

                    {/* Settings Button */}
                    <button
                        onClick={() => setRightPanelMode?.(rightPanelMode === 'settings' ? null : 'settings')}
                        className={`p-1 rounded hover:bg-black/5 dark:hover:bg-white/10 transition-colors ${rightPanelMode === 'settings' ? 'text-amber-500' : 'text-foreground'}`}
                        title="Settings"
                    >
                        <Settings size={16} />
                    </button>
                </div>
                <div className="font-medium pl-2">
                    {format(time, 'MMM d h:mm a')}
                </div>
            </div>
        </header>
    );
}
