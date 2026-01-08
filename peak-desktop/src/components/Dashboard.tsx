import { motion } from 'framer-motion';
import {
    Clock, Star, HardDrive, Activity, Wifi, Shield,
    Image as ImageIcon, Music, Video, FileText, ArrowUpRight
} from 'lucide-react';
import { cn } from '../lib/utils';

interface DashboardProps {
    type: string; // 'favorites', 'media', 'peakos', 'tags'
}

export function Dashboard({ type }: DashboardProps) {
    const getTypeData = () => {
        switch (type.toLowerCase()) {
            case 'favorites': return {
                title: 'Favorites',
                icon: Star,
                color: 'text-amber-400',
                stats: [
                    { label: 'Pinned Items', value: '12', icon: Star },
                    { label: 'Recents', value: '24', icon: Clock },
                    { label: 'Total Size', value: '128 GB', icon: HardDrive }
                ],
                widgets: ['recents', 'pinned']
            };
            case 'media': return {
                title: 'Media Center',
                icon: ImageIcon,
                color: 'text-pink-400',
                stats: [
                    { label: 'Photos', value: '1,204', icon: ImageIcon },
                    { label: 'Songs', value: '432', icon: Music },
                    { label: 'Movies', value: '89', icon: Video }
                ],
                widgets: ['media_grid']
            };
            case 'peakos': // peakos or system
            case 'system': return {
                title: 'System Status',
                icon: Activity,
                color: 'text-blue-400',
                stats: [
                    { label: 'CPU Load', value: '12%', icon: Activity },
                    { label: 'Memory', value: '4.2 GB', icon: HardDrive },
                    { label: 'Network', value: 'Online', icon: Wifi }
                ],
                widgets: ['health', 'security']
            };
            case 'tags': return {
                title: 'Tag Explorer',
                icon: FileText,
                color: 'text-green-400',
                stats: [
                    { label: 'Important', value: '5', icon: Shield },
                    { label: 'Work', value: '12', icon: Briefcase },
                    { label: 'Personal', value: '45', icon: User }
                ],
                widgets: ['tags_cloud']
            };
            default: return {
                title: type,
                icon: FileText,
                color: 'text-white',
                stats: [],
                widgets: []
            };
        }
    };

    const data = getTypeData();
    const Icon = data.icon;

    return (
        <div className="w-full h-full overflow-y-auto p-8 flex flex-col gap-8">
            {/* Hero Section */}
            <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                className="flex items-center gap-6 p-8 rounded-3xl bg-gradient-to-br from-white/5 to-transparent border border-white/10"
            >
                <div className={cn("p-4 rounded-2xl bg-white/5", data.color)}>
                    <Icon size={48} strokeWidth={1.5} />
                </div>
                <div>
                    <h1 className="text-4xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-white to-white/60">
                        {data.title}
                    </h1>
                    <p className="text-white/40 mt-1">Dashboard Overview</p>
                </div>
            </motion.div>

            {/* Stats Grid */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                {data.stats.map((stat, idx) => (
                    <motion.div
                        key={stat.label}
                        initial={{ opacity: 0, scale: 0.95 }}
                        animate={{ opacity: 1, scale: 1 }}
                        transition={{ delay: idx * 0.1 }}
                        className="p-6 rounded-2xl bg-white/5 border border-white/5 hover:bg-white/10 transition-colors group"
                    >
                        <div className="flex items-center justify-between mb-2">
                            <stat.icon size={20} className="text-white/30 group-hover:text-white/60 transition-colors" />
                            <ArrowUpRight size={16} className="text-white/20 opacity-0 group-hover:opacity-100 transition-opacity" />
                        </div>
                        <div className="text-2xl font-semibold">{stat.value}</div>
                        <div className="text-sm text-white/40">{stat.label}</div>
                    </motion.div>
                ))}
            </div>

            {/* Widgets Area */}
            {data.widgets.includes('recents') && (
                <div className="space-y-4">
                    <h2 className="text-lg font-medium text-white/60 pl-2">Recent Access</h2>
                    <div className="grid grid-cols-1 gap-2">
                        {[1, 2, 3].map(i => (
                            <div key={i} className="flex items-center gap-4 p-4 rounded-xl bg-black/20 hover:bg-white/5 transition-colors cursor-pointer border border-transparent hover:border-white/5">
                                <div className="p-2 rounded-lg bg-blue-500/10 text-blue-400">
                                    <FileText size={18} />
                                </div>
                                <div className="flex-1">
                                    <div className="font-medium text-sm">Project_Proposal_Final_v2.pdf</div>
                                    <div className="text-xs text-white/30">Modified 2 hours ago</div>
                                </div>
                                <div className="text-xs text-white/20">Documents</div>
                            </div>
                        ))}
                    </div>
                </div>
            )}

            {data.widgets.includes('health') && (
                <div className="grid grid-cols-2 gap-4">
                    <div className="p-6 rounded-2xl bg-emerald-500/10 border border-emerald-500/20">
                        <h3 className="text-emerald-400 font-medium mb-2">System Healthy</h3>
                        <p className="text-sm text-emerald-400/60">All services running optimally.</p>
                    </div>
                    <div className="p-6 rounded-2xl bg-blue-500/10 border border-blue-500/20">
                        <h3 className="text-blue-400 font-medium mb-2">Peak Intelligence</h3>
                        <p className="text-sm text-blue-400/60">Neural Link Active (v1.0.4)</p>
                    </div>
                </div>
            )}

            {/* Empty State / Coming Soon for others */}
            {data.widgets.length === 0 && (
                <div className="flex-1 flex items-center justify-center p-10 border-2 border-dashed border-white/5 rounded-3xl">
                    <p className="text-white/20">Widgets Configuration Empty</p>
                </div>
            )}
        </div>
    );
}

// Mock Icons for missing imports
const User = ({ className }: { className?: string }) => (
    <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
        className={className}
        width="24" height="24"
    >
        <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" />
        <circle cx="12" cy="7" r="4" />
    </svg>
);

const Briefcase = ({ className }: { className?: string }) => (
    <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
        className={className}
        width="24" height="24"
    >
        <rect width="20" height="14" x="2" y="7" rx="2" ry="2" />
        <path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16" />
    </svg>
);
