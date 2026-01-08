import { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { ArrowRight, User, Smile, Heart, Camera, Gamepad2, Music, Coffee, Globe, Star, Cloud, Moon, Sun, Zap, Aperture, Leaf, Flame, Anchor, Rocket, Plane, Lock, Shield, Users } from 'lucide-react';
import { useOSStore } from '../stores/useOSStore';
import { cn } from '../lib/utils';
import peakLogo from '../assets/peak-logo-small.png';
import { VectorBackground } from './VectorBackground';


export function LoginScreen() {
    const { users, login, selectUser, isGuestModeEnabled } = useOSStore();

    // selectedUser can be a User object or the string 'guest'
    const [selectedUser, setSelectedUser] = useState<any>(
        !isGuestModeEnabled && users.length === 1 ? users[0] : null
    );
    const [password, setPassword] = useState('');
    const [isError, setIsError] = useState(false);
    const [showPasswordField, setShowPasswordField] = useState(false);

    const avatarIcons: Record<string, any> = {
        User, Smile, Heart, Camera, Gamepad2,
        Music, Coffee, Globe, Star, Cloud,
        Moon, Sun, Zap, Aperture, Leaf,
        Flame, Anchor, Rocket, Plane
    };

    const handleLogin = (e: React.FormEvent) => {
        e.preventDefault();
        // Prototype logic: allow login with any non-empty password
        // In production, this would verify against a secure hash
        if (password.length > 0) {
            selectUser(selectedUser);
            login();
        } else {
            setIsError(true);
            setTimeout(() => setIsError(false), 500);
        }
    };

    return (
        <div className="fixed inset-0 z-[200] bg-[#F5F5F7] flex flex-col items-center justify-center p-6 select-none overflow-hidden text-black">
            <VectorBackground particleCount={40} color="100, 100, 100" />
            <div className="absolute inset-0 opacity-[0.03] bg-[url('https://grainy-gradients.vercel.app/noise.svg')] pointer-events-none" />

            {/* Ambient Background Elements */}
            <div className="absolute top-[-10%] right-[-10%] w-[50%] h-[50%] bg-black/[0.02] rounded-full blur-[120px]" />
            <div className="absolute bottom-[-10%] left-[-10%] w-[50%] h-[50%] bg-black/[0.01] rounded-full blur-[120px]" />

            {/* Logo in Corner */}
            <div className="absolute top-10 left-10 opacity-20 scale-50 pointer-events-none z-[210]">
                <img src={peakLogo} alt="Peak Logo" className="w-32 h-auto brightness-0" />
            </div>

            {/* User Selection View */}
            {!selectedUser ? (
                <motion.div
                    initial={{ opacity: 0, scale: 0.95 }}
                    animate={{ opacity: 1, scale: 1 }}
                    className="z-10 flex flex-col items-center gap-12"
                >
                    <div className="flex flex-wrap justify-center gap-8">
                        {users.map((u) => {
                            const Icon = avatarIcons[u.avatar] || User;
                            return (
                                <button
                                    key={u.username}
                                    onClick={() => setSelectedUser(u)}
                                    className="flex flex-col items-center gap-4 group transition-all"
                                >
                                    <div className="w-32 h-32 rounded-full bg-white shadow-xl flex items-center justify-center relative overflow-hidden transition-all group-hover:scale-110 group-hover:shadow-2xl active:scale-95 group-hover:ring-8 ring-black/5">
                                        <Icon size={64} strokeWidth={1} className="text-black/80" />
                                    </div>
                                    <span className="font-bold text-lg tracking-tight text-black group-hover:translate-y-1 transition-transform">{u.fullName}</span>
                                </button>
                            );
                        })}

                        {isGuestModeEnabled && (
                            <button
                                onClick={() => setSelectedUser('guest')}
                                className="flex flex-col items-center gap-6 group transition-all"
                            >
                                <div className="w-32 h-32 rounded-full bg-black/5 shadow-inner flex items-center justify-center relative overflow-hidden transition-all group-hover:scale-110 active:scale-95 group-hover:ring-8 ring-black/5 group-hover:bg-black/10">
                                    <Shield size={64} strokeWidth={1} className="text-black/40 group-hover:text-black/60 transition-colors" />
                                </div>
                                <span className="font-bold text-lg tracking-tight text-black/40 group-hover:text-black/60 transition-colors group-hover:translate-y-1 transition-transform">Guest</span>
                            </button>
                        )}
                    </div>

                    <div className="mt-12">
                        <button
                            onClick={() => window.location.reload()}
                            className="text-[11px] font-bold tracking-widest uppercase text-black/30 hover:text-black transition-colors flex flex-col items-center gap-3 group"
                        >
                            <div className="w-10 h-10 rounded-full bg-black/5 flex items-center justify-center group-hover:bg-black group-hover:text-white transition-all shadow-sm">
                                <Lock size={16} />
                            </div>
                            Sleep
                        </button>
                    </div>
                </motion.div>
            ) : (
                /* Password Entry View */
                <motion.div
                    initial={{ opacity: 0, scale: 0.95 }}
                    animate={{ opacity: 1, scale: 1 }}
                    className="z-10 flex flex-col items-center gap-10 w-full max-w-sm"
                >
                    <div className={cn("flex flex-col items-center gap-8 w-full", isError && "animate-shake")}>
                        <button
                            onClick={() => !showPasswordField && setShowPasswordField(true)}
                            className={cn(
                                "flex flex-col items-center gap-8 transition-all duration-500",
                                !showPasswordField && "hover:scale-105 active:scale-95 grayscale-[0.5] hover:grayscale-0"
                            )}
                        >
                            <div className="w-32 h-32 rounded-full bg-white shadow-2xl flex items-center justify-center relative overflow-hidden border-4 border-white">
                                {selectedUser === 'guest' ? (
                                    <Shield size={64} strokeWidth={1} className="text-black/40" />
                                ) : (
                                    (() => {
                                        const Icon = avatarIcons[selectedUser.avatar] || User;
                                        return <Icon size={64} strokeWidth={1} className="text-black/80" />;
                                    })()
                                )}
                            </div>

                            <div className="text-center space-y-1">
                                <h2 className="text-2xl font-bold tracking-tight text-black">
                                    {selectedUser === 'guest' ? 'Guest' : selectedUser.fullName}
                                </h2>
                                {!showPasswordField && (
                                    <p className="text-[11px] font-bold tracking-[0.2em] uppercase text-black/20 animate-pulse">Click to Unlock</p>
                                )}
                            </div>
                        </button>

                        <AnimatePresence>
                            {showPasswordField && (
                                <motion.div
                                    initial={{ opacity: 0, y: 10, filter: 'blur(10px)' }}
                                    animate={{ opacity: 1, y: 0, filter: 'blur(0px)' }}
                                    className="w-full flex flex-col items-center"
                                >
                                    {selectedUser === 'guest' ? (
                                        <button
                                            onClick={() => login()}
                                            className="w-72 bg-black text-white rounded-full py-4 text-sm font-bold shadow-xl hover:scale-105 active:scale-95 transition-all flex items-center justify-center gap-3"
                                        >
                                            <span>Login as Guest</span>
                                            <ArrowRight size={18} />
                                        </button>
                                    ) : (
                                        <form onSubmit={handleLogin} className="relative w-72">
                                            <input
                                                autoFocus
                                                type="password"
                                                placeholder="Enter Password"
                                                value={password}
                                                onChange={(e) => setPassword(e.target.value)}
                                                className="w-full bg-white/60 backdrop-blur-md border border-black/[0.05] rounded-full pl-6 pr-12 py-4 text-center text-sm focus:outline-none focus:ring-8 focus:ring-black/5 transition-all shadow-xl placeholder:text-black/20 text-black"
                                            />
                                            <button
                                                type="submit"
                                                className="absolute right-1.5 top-1/2 -translate-y-1/2 p-2.5 bg-black text-white rounded-full hover:scale-110 active:scale-90 transition-all shadow-lg flex items-center justify-center"
                                            >
                                                <ArrowRight size={18} />
                                            </button>
                                        </form>
                                    )}
                                </motion.div>
                            )}
                        </AnimatePresence>
                    </div>

                    {/* Options */}
                    <div className="mt-12 flex items-start gap-16">
                        <button
                            onClick={() => window.location.reload()}
                            className="text-[10px] font-bold tracking-widest uppercase text-black/30 hover:text-black transition-colors flex flex-col items-center gap-3 group"
                        >
                            <div className="w-10 h-10 rounded-full bg-black/5 flex items-center justify-center group-hover:bg-black group-hover:text-white transition-all shadow-sm">
                                <Lock size={16} />
                            </div>
                            Sleep
                        </button>

                        {(users.length > 0 || isGuestModeEnabled) && (
                            <button
                                onClick={() => {
                                    setSelectedUser(null);
                                    setPassword('');
                                }}
                                className="text-[10px] font-bold tracking-widest uppercase text-black/30 hover:text-black transition-colors flex flex-col items-center gap-3 group"
                            >
                                <div className="w-10 h-10 rounded-full bg-black/5 flex items-center justify-center group-hover:bg-black group-hover:text-white transition-all shadow-sm">
                                    <Users size={16} />
                                </div>
                                Switch User
                            </button>
                        )}
                    </div>
                </motion.div>
            )}

            {/* Footer */}
            <div className="absolute bottom-12 flex flex-col items-center gap-4">
                <p className="text-[10px] font-bold tracking-[0.4em] text-black/10 uppercase">
                    PeakOS 2026
                </p>
            </div>
        </div>
    );
}
