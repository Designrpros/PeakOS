import { useState, useMemo } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import {
    ArrowRight,
    Code,
    User,
    Lock,
    ChevronLeft,
    Smile,
    Heart,
    Camera,
    Gamepad2,
    Music,
    Coffee,
    Globe,
    Star,
    Cloud,
    Moon,
    Sun,
    Zap,
    Aperture,
    Leaf,
    Flame,
    Anchor,
    Rocket,
    Plane,
    Check
} from 'lucide-react';
import peakLogo from '../assets/peak-logo-small.png';
import { VectorBackground } from './VectorBackground';

interface SetupAssistantProps {
    onComplete: (userData: any) => void;
    onInspectCode: () => void;
}

export function SetupAssistant({ onComplete, onInspectCode }: SetupAssistantProps) {
    const [step, setStep] = useState(0);
    const [formData, setFormData] = useState({
        fullName: '',
        username: '',
        password: '',
        confirmPassword: '',
        hint: '',
        avatar: 'User'
    });

    const updateForm = (key: string, value: string) => {
        setFormData(prev => ({ ...prev, [key]: value }));
    };

    const nextStep = () => setStep(s => s + 1);
    const prevStep = () => setStep(s => s - 1);

    const icons: Record<string, any> = {
        User, Smile, Heart, Camera, Gamepad2,
        Music, Code, Coffee, Globe, Star,
        Cloud, Moon, Sun, Zap, Aperture,
        Leaf, Flame, Anchor, Rocket, Plane
    };

    const steps = useMemo(() => [
        {
            id: 'welcome',
            title: "Welcome to PeakOS",
            description: "Let's get you set up.",
            content: null
        },
        {
            id: 'account',
            title: "Create a Computer Account",
            description: "Set up the local account you'll use to log in and manage PeakOS.",
            content: (
                <div className="w-full space-y-5 text-left max-w-md">
                    <div className="space-y-1.5">
                        <label className="text-[13px] font-medium text-black/40 ml-1">Full Name</label>
                        <div className="relative">
                            <input
                                type="text"
                                placeholder="John Appleseed"
                                value={formData.fullName}
                                onChange={(e) => updateForm('fullName', e.target.value)}
                                className="w-full h-11 bg-white border border-black/10 rounded-lg px-4 text-black placeholder:text-black/20 focus:border-black outline-none transition-all shadow-sm"
                            />
                            <User className="absolute right-4 top-1/2 -translate-y-1/2 text-black/20" size={16} />
                        </div>
                        <p className="text-[11px] text-black/30 ml-1">Your display name for the user interface.</p>
                    </div>
                    <div className="space-y-1.5">
                        <label className="text-[13px] font-medium text-black/40 ml-1">Account Name</label>
                        <input
                            type="text"
                            placeholder="john"
                            value={formData.username}
                            onChange={(e) => updateForm('username', e.target.value.toLowerCase().replace(/\s/g, ''))}
                            className="w-full h-11 bg-white border border-black/10 rounded-lg px-4 text-black placeholder:text-black/20 focus:border-black outline-none transition-all shadow-sm"
                        />
                        <p className="text-[11px] text-black/30 ml-1">Used for your home folder. Lowercase, no spaces.</p>
                    </div>
                </div>
            )
        },
        {
            id: 'avatar',
            title: "Choose Your Icon",
            description: "Pick an icon that represents you. You can change this later in System Settings.",
            content: (
                <div className="w-full max-w-lg">
                    <div className="grid grid-cols-5 gap-4 p-4 max-h-[300px] overflow-y-auto no-scrollbar">
                        {Object.keys(icons).map((iconName) => {
                            const IconComponent = icons[iconName];
                            const isSelected = formData.avatar === iconName;

                            return (
                                <button
                                    key={iconName}
                                    onClick={() => updateForm('avatar', iconName)}
                                    className={`
                                        relative group flex flex-col items-center justify-center aspect-square rounded-2xl transition-all duration-300
                                        ${isSelected
                                            ? 'bg-black text-white shadow-xl scale-105'
                                            : 'bg-white border border-black/5 text-black/40 hover:border-black/20 hover:text-black/60'}
                                    `}
                                >
                                    <IconComponent size={32} strokeWidth={1.5} />
                                    {isSelected && (
                                        <motion.div
                                            layoutId="selected-check"
                                            className="absolute -top-1 -right-1 w-5 h-5 bg-green-500 text-white rounded-full flex items-center justify-center shadow-md"
                                        >
                                            <Check size={12} strokeWidth={3} />
                                        </motion.div>
                                    )}
                                </button>
                            );
                        })}
                    </div>
                </div>
            )
        },
        {
            id: 'security',
            title: "Choose Your Password",
            description: "A strong password helps keep your PeakOS data secure.",
            content: (
                <div className="w-full space-y-5 text-left max-w-md">
                    <div className="grid grid-cols-2 gap-4">
                        <div className="space-y-1.5">
                            <label className="text-[13px] font-medium text-black/40 ml-1">Password</label>
                            <div className="relative">
                                <input
                                    type="password"
                                    value={formData.password}
                                    onChange={(e) => updateForm('password', e.target.value)}
                                    className="w-full h-11 bg-white border border-black/10 rounded-lg px-4 text-black focus:border-black outline-none transition-all shadow-sm"
                                />
                                <Lock className="absolute right-4 top-1/2 -translate-y-1/2 text-black/20" size={16} />
                            </div>
                        </div>
                        <div className="space-y-1.5">
                            <label className="text-[13px] font-medium text-black/40 ml-1">Confirm</label>
                            <input
                                type="password"
                                value={formData.confirmPassword}
                                onChange={(e) => updateForm('confirmPassword', e.target.value)}
                                className="w-full h-11 bg-white border border-black/10 rounded-lg px-4 text-black focus:border-black outline-none transition-all shadow-sm"
                            />
                        </div>
                    </div>
                    <div className="space-y-1.5">
                        <label className="text-[13px] font-medium text-black/40 ml-1 flex items-center gap-2">
                            Password Hint
                        </label>
                        <input
                            type="text"
                            placeholder="Required"
                            value={formData.hint}
                            onChange={(e) => updateForm('hint', e.target.value)}
                            className="w-full h-11 bg-white border border-black/10 rounded-lg px-4 text-black placeholder:text-black/20 focus:border-black outline-none transition-all shadow-sm"
                        />
                        <p className="text-[11px] text-black/30 ml-1">Something to help you remember your password.</p>
                    </div>
                </div>
            )
        },
        {
            id: 'finalizing',
            title: "Setting Up PeakOS...",
            description: "Finalizing system configuration and creating your profile.",
            content: (
                <div className="flex flex-col items-center py-8">
                    <div className="w-64 h-1.5 bg-black/5 rounded-full overflow-hidden shadow-inner">
                        <motion.div
                            initial={{ width: 0 }}
                            animate={{ width: "100%" }}
                            transition={{ duration: 3, ease: "easeInOut" }}
                            onAnimationComplete={() => {
                                onComplete({
                                    fullName: formData.fullName,
                                    username: formData.username,
                                    avatar: formData.avatar
                                });
                            }}
                            className="h-full bg-black shadow-[0_0_10px_rgba(0,0,0,0.1)]"
                        />
                    </div>
                </div>
            )
        }
    ], [formData, icons, onComplete]);

    const currentStep = steps[step] || steps[0];
    const isLastStep = step === steps.length - 1;
    const isReadyForNext = () => {
        if (step === 1) return formData.fullName && formData.username;
        if (step === 2) return true;
        if (step === 3) return formData.password && formData.password === formData.confirmPassword && formData.hint;
        return true;
    };

    return (
        <div className="fixed inset-0 z-[100] bg-[#F5F5F7] flex items-center justify-center p-6 overflow-hidden select-none font-sans text-[#1d1d1f]">
            <VectorBackground particleCount={40} color="100, 100, 100" />
            <div className="absolute inset-0 opacity-[0.03] bg-[url('https://grainy-gradients.vercel.app/noise.svg')] pointer-events-none" />

            {/* Ambient Background Elements */}
            <div className="absolute top-[-10%] right-[-10%] w-[50%] h-[50%] bg-black/[0.02] rounded-full blur-[120px]" />
            <div className="absolute bottom-[-10%] left-[-10%] w-[50%] h-[50%] bg-black/[0.01] rounded-full blur-[120px]" />

            <AnimatePresence mode="popLayout">
                <motion.div
                    key={step}
                    initial={{ opacity: 0, scale: 0.98, y: 10 }}
                    animate={{ opacity: 1, scale: 1, y: 0 }}
                    exit={{ opacity: 0, scale: 1.02, y: -10 }}
                    transition={{ duration: 0.4, ease: [0.16, 1, 0.3, 1] }}
                    className="relative max-w-2xl w-full flex flex-col items-center py-12"
                >
                    <div className="flex flex-col items-center mb-12 w-full text-center">
                        <h1 className="text-4xl font-semibold tracking-tight text-black mb-4">
                            {currentStep.title}
                        </h1>
                        <p className="text-black/50 text-lg leading-relaxed max-w-[80%]">
                            {currentStep.description}
                        </p>
                    </div>

                    {currentStep.content && (
                        <div className="w-full flex-1 flex flex-col items-center">
                            {currentStep.content}
                        </div>
                    )}

                    {/* Immersive Action Bar */}
                    {!isLastStep && (
                        <div className="flex flex-col gap-4 w-full max-w-sm mt-12 items-center">
                            <button
                                disabled={!isReadyForNext()}
                                onClick={nextStep}
                                className="w-64 h-12 bg-black text-white rounded-full font-medium text-[15px] flex items-center justify-center gap-2 hover:bg-black/90 transition-all active:scale-[0.98] disabled:opacity-30 disabled:cursor-not-allowed shadow-lg shadow-black/10"
                            >
                                {step === 0 ? "Get Started" : "Continue"}
                                <ArrowRight size={18} />
                            </button>

                            {step === 0 && (
                                <button
                                    onClick={() => onComplete(null)}
                                    className="text-[13px] font-medium text-black/40 hover:text-black transition-colors"
                                >
                                    Continue as Guest
                                </button>
                            )}
                        </div>
                    )}
                </motion.div>
            </AnimatePresence>

            {/* Top Navigation - Back Button (Only if not step 0 and not last step) */}
            {step > 0 && !isLastStep && (
                <button
                    onClick={prevStep}
                    className="absolute top-10 left-10 p-3 text-black/40 hover:text-black transition-colors rounded-full hover:bg-black/5 z-[110]"
                >
                    <ChevronLeft size={24} />
                </button>
            )}

            {/* Logo - ONLY shown on step 0 */}
            {!isLastStep && step === 0 && (
                <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-[240px] z-[110]">
                    <img
                        src={peakLogo}
                        alt="Peak Logo"
                        className="w-48 h-auto brightness-0"
                    />
                </div>
            )}

            {/* Top Right Corner: Diagnostic Mode */}
            {!isLastStep && (
                <button
                    onClick={onInspectCode}
                    className="absolute top-10 right-10 p-2 text-black/10 flex items-center gap-2 text-[10px] font-bold tracking-widest uppercase z-[110]"
                >
                    <Code size={14} />
                    <span>Diagnostic Mode</span>
                </button>
            )}

            {/* Footer Branding */}
            <div className="absolute bottom-10 left-0 right-0 text-center pointer-events-none">
                <p className="text-black/10 text-[10px] font-bold tracking-[0.4em] uppercase">
                    PeakOS 2026
                </p>
            </div>
        </div>
    );
}
