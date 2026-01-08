import { motion, useDragControls } from 'framer-motion';
import { X } from 'lucide-react';
import { useState, useRef } from 'react';
import { cn } from '../lib/utils';
import { useOSStore } from '../stores/useOSStore';

interface WindowProps {
    id: number;
    spaceId: number;
    title: string;
    children: React.ReactNode;
    x: number;
    y: number;
    width: number;
    height: number;
    zIndex?: number;
    constraintsRef?: React.RefObject<HTMLDivElement>;
    onFocus?: () => void;
    onClose?: () => void;
}

export function Window({ id, spaceId, title, children, x, y, width, height, zIndex = 1, constraintsRef, onFocus, onClose }: WindowProps) {
    const { updateWindow } = useOSStore();
    const [isHovered, setIsHovered] = useState(false);
    const [isResizing, setIsResizing] = useState(false);

    const handleResize = (e: React.MouseEvent) => {
        e.preventDefault();
        e.stopPropagation();
        setIsResizing(true);

        const startX = e.pageX;
        const startY = e.pageY;
        const startWidth = width;
        const startHeight = height;

        const onMouseMove = (moveEvent: MouseEvent) => {
            const newWidth = Math.max(300, startWidth + (moveEvent.pageX - startX));
            const newHeight = Math.max(200, startHeight + (moveEvent.pageY - startY));
            updateWindow(spaceId, id, { width: newWidth, height: newHeight });
        };

        const onMouseUp = () => {
            setIsResizing(false);
            document.removeEventListener('mousemove', onMouseMove);
            document.removeEventListener('mouseup', onMouseUp);
        };

        document.addEventListener('mousemove', onMouseMove);
        document.addEventListener('mouseup', onMouseUp);
    };

    const { spaces, activeSpaceIndex } = useOSStore();
    const isActive = spaces[activeSpaceIndex]?.windows.find(w => w.id === id)?.zIndex ===
        Math.max(...(spaces[activeSpaceIndex]?.windows.map(w => w.zIndex || 0) || [0]));

    const dragControls = useDragControls();
    const windowRef = useRef<HTMLDivElement>(null);

    return (
        <motion.div
            ref={windowRef}
            drag={!isResizing}
            dragControls={dragControls}
            dragListener={false}
            dragMomentum={false}
            dragElastic={0}
            dragConstraints={constraintsRef || { top: 0 }}
            animate={{
                x,
                y,
                width,
                height,
                transition: { type: 'spring', damping: 30, stiffness: 400 }
            }}
            onDragEnd={(_, info) => {
                // Clamping y to at least 0 to prevent windows from vanishing under MenuBar
                updateWindow(spaceId, id, {
                    x: x + info.offset.x,
                    y: Math.max(0, y + info.offset.y)
                });
            }}
            onMouseDown={(e) => {
                // Ensure we focus on any click, but don't steal focus from inputs
                if (!(e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement)) {
                    onFocus?.();
                }
            }}
            onMouseEnter={() => setIsHovered(true)}
            onMouseLeave={() => setIsHovered(false)}
            style={{ zIndex, position: 'absolute', top: 0, left: 0 }}
            className="bg-card/85 backdrop-blur-2xl border border-border/40 rounded-lg flex flex-col overflow-hidden shadow-2xl shadow-black/40 transition-shadow duration-300 pointer-events-auto"
        >
            {/* Interaction Shield - Fixes iframe focus issue */}
            {!isActive && (
                <div
                    className="absolute inset-0 z-[100] cursor-default"
                    onMouseDown={() => {
                        onFocus?.();
                    }}
                />
            )}
            {/* Subtle Close Button - Hover Reveal */}
            <button
                onClick={onClose}
                className={cn(
                    "absolute top-2 right-2 z-50 p-1.5 rounded-full bg-red-500/20 text-red-500 hover:bg-red-500 hover:text-white transition-all duration-200",
                    isHovered ? "opacity-100 scale-100" : "opacity-0 scale-90"
                )}
            >
                <X size={10} strokeWidth={3} />
            </button>

            {/* Drag Handle (Top Bar Zone) */}
            <div
                onPointerDown={(e) => dragControls.start(e)}
                className="h-8 w-full absolute top-0 left-0 cursor-grab active:cursor-grabbing z-40 flex items-center px-3"
            >
                <div className={cn(
                    "text-[10px] font-bold uppercase tracking-widest text-foreground/20 pointer-events-none select-none transition-opacity",
                    isHovered ? "opacity-100" : "opacity-0"
                )}>
                    {title}
                </div>
            </div>

            {/* Content Container */}
            <div className="flex-1 overflow-hidden relative group">
                {/* Content area starts below title bar zone (h-8) */}
                <div className="w-full h-full pt-8 overflow-auto">
                    {children}
                </div>
            </div>

            {/* Resize Handle - Bottom Right */}
            <div
                onMouseDown={handleResize}
                className="absolute bottom-0 right-0 w-4 h-4 cursor-nwse-resize z-50 flex items-end justify-end p-0.5 group/resize"
            >
                <div className="w-1.5 h-1.5 border-r border-b border-foreground/20 rounded-br-sm group-hover/resize:border-foreground/50 transition-colors" />
            </div>
        </motion.div>
    );
}
