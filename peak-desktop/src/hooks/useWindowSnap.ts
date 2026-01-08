import { useEffect } from 'react';
import { useOSStore } from '../stores/useOSStore';

export function useWindowSnap() {
    const { spaces, activeSpaceIndex, snapWindow } = useOSStore();

    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            // Primarily use Option (Alt). 
            // We ignore if focused on an input to preserve text navigation.
            if (!e.altKey) return;
            const target = e.target as HTMLElement;
            if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable) return;

            const space = spaces[activeSpaceIndex];
            // Find the window with the highest zIndex (active window)
            const activeWindow = [...space.windows].sort((a, b) => (b.zIndex || 0) - (a.zIndex || 0))[0];

            if (!activeWindow) return;

            e.preventDefault();

            switch (e.code) {
                // Halves
                case 'ArrowLeft': snapWindow(activeWindow.id, 'left'); break;
                case 'ArrowRight': snapWindow(activeWindow.id, 'right'); break;
                case 'ArrowUp': snapWindow(activeWindow.id, 'top'); break;
                case 'ArrowDown': snapWindow(activeWindow.id, 'bottom'); break;

                // Quarters
                case 'KeyU': snapWindow(activeWindow.id, 'top-left'); break;
                case 'KeyI': snapWindow(activeWindow.id, 'top-right'); break;
                case 'KeyJ': snapWindow(activeWindow.id, 'bottom-left'); break;
                case 'KeyK': snapWindow(activeWindow.id, 'bottom-right'); break;

                // Thirds
                case 'KeyD': snapWindow(activeWindow.id, 'third-left'); break;
                case 'KeyF': snapWindow(activeWindow.id, 'third-center'); break;
                case 'KeyG': snapWindow(activeWindow.id, 'third-right'); break;

                // Utils
                case 'Enter': snapWindow(activeWindow.id, 'maximize'); break;
                case 'KeyC': snapWindow(activeWindow.id, 'center'); break;
            }
        };

        window.addEventListener('keydown', handleKeyDown);
        return () => window.removeEventListener('keydown', handleKeyDown);
    }, [spaces, activeSpaceIndex, snapWindow]);
}
