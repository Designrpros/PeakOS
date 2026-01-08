import { Desktop } from './components/Desktop';
import { SetupAssistant } from './components/SetupAssistant';
import { LoginScreen } from './components/LoginScreen';
import { useOSStore } from './stores/useOSStore';
import { Terminal } from './components/Terminal';
import { useState, useEffect } from 'react';
import { X } from 'lucide-react';

function App() {
    const { isSetupComplete, isLoggedIn, completeSetup, users, isGuestModeEnabled, theme, setTheme } = useOSStore();
    const [isAnalyzing, setIsAnalyzing] = useState(false);

    useEffect(() => {
        setTheme(theme);
    }, []);

    // Secure Boot: If no users exist and guest mode isn't enabled, the system is uninitialized.
    const isActuallyReady = isSetupComplete && (users.length > 0 || isGuestModeEnabled);

    if (!isActuallyReady) {
        return (
            <div className="w-full h-full relative bg-[#F5F5F7]">
                <SetupAssistant
                    onComplete={(userData) => completeSetup(userData)}
                    onInspectCode={() => setIsAnalyzing(true)}
                />

                {isAnalyzing && (
                    <div className="fixed inset-0 z-[200] bg-black/80 backdrop-blur-sm flex items-center justify-center p-12">
                        <div className="w-full max-w-4xl aspect-video bg-zinc-950 rounded-3xl overflow-hidden border border-white/10 shadow-2xl flex flex-col">
                            <div className="flex items-center justify-between px-6 py-4 bg-zinc-900 border-b border-white/5">
                                <div className="flex items-center gap-2">
                                    <div className="w-3 h-3 rounded-full bg-red-500/20 border border-red-500/50" />
                                    <div className="w-3 h-3 rounded-full bg-amber-500/20 border border-amber-500/50" />
                                    <div className="w-3 h-3 rounded-full bg-green-500/20 border border-green-500/50" />
                                    <span className="ml-4 text-[11px] font-bold tracking-widest uppercase text-white/40">Installation Logs</span>
                                </div>
                                <button
                                    onClick={() => setIsAnalyzing(false)}
                                    className="text-white/40 hover:text-white transition-colors"
                                >
                                    <X size={20} />
                                </button>
                            </div>
                            <div className="flex-1">
                                <Terminal />
                            </div>
                        </div>
                    </div>
                )}
            </div>
        );
    }

    if (!isLoggedIn) {
        return <LoginScreen />;
    }

    return (
        <Desktop />
    );
}

export default App;
