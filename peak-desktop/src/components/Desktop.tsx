import { Dock } from './Dock';
import { MenuBar } from './MenuBar';
import { Wallpaper } from './Wallpaper';
import { Window } from './Window';
import { useOSStore } from '../stores/useOSStore';
import { Sidebar } from './Sidebar';
import { Inspector, InspectorMode } from './Inspector';
import { FileExplorer } from './FileExplorer';
import { Dashboard } from './Dashboard';
import { Terminal } from './Terminal';
import { AppStore } from './AppStore';
import { Command } from '@tauri-apps/plugin-shell';
import { Browser } from './Browser';
import { SettingsApp } from './SettingsApp';
import { MediaPlayer } from './MediaPlayer';
import { Spotlight } from './Spotlight';
import { AppSwitcher } from './AppSwitcher';
import { useWindowSnap } from '../hooks/useWindowSnap';
import { useState, useEffect, useCallback, useRef } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { cn } from '../lib/utils';
import { X, Plus } from 'lucide-react';

export function Desktop() {
    useWindowSnap();
    const {
        spaces,
        setSpaces,
        activeSpaceIndex,
        setActiveSpace,
        switchSpace,
        isOverviewOpen,
        toggleOverview,
        sidebarWidth,
        setSidebarWidth,
        showSidebar,
        setShowSidebar,
        currentView,
        setCurrentView,
        theme,
        setTheme,
        launchApp,
        focusWindow,
        closeWindow,
        closeActiveWindow,
        installedApps,
        shortcuts
    } = useOSStore();

    const [isResizingSidebar, setIsResizingSidebar] = useState(false);
    const [isResizingInspector, setIsResizingInspector] = useState(false);
    const [inspectorWidth, setInspectorWidth] = useState(320);
    const [rightPanelMode, setRightPanelMode] = useState<InspectorMode | null>(null);
    const [isSwitcherOpen, setIsSwitcherOpen] = useState(false);
    const [switcherSelectedIndex, setSwitcherSelectedIndex] = useState(0);

    const desktopRef = useRef<HTMLDivElement>(null);

    const handleMouseMove = useCallback((e: MouseEvent) => {
        if (isResizingSidebar) {
            const newWidth = Math.max(160, Math.min(480, e.clientX));
            setSidebarWidth(newWidth);
        }
        if (isResizingInspector) {
            const newWidth = Math.max(280, Math.min(600, window.innerWidth - e.clientX));
            setInspectorWidth(newWidth);
        }
    }, [isResizingSidebar, isResizingInspector, setSidebarWidth]);

    const handleMouseUp = useCallback(() => {
        setIsResizingSidebar(false);
        setIsResizingInspector(false);
    }, []);

    useEffect(() => {
        if (isResizingSidebar || isResizingInspector) {
            window.addEventListener('mousemove', handleMouseMove);
            window.addEventListener('mouseup', handleMouseUp);
            document.body.style.cursor = 'col-resize';
        } else {
            document.body.style.cursor = 'default';
        }

        return () => {
            window.removeEventListener('mousemove', handleMouseMove);
            window.removeEventListener('mouseup', handleMouseUp);
            document.body.style.cursor = 'default';
        };
    }, [isResizingSidebar, isResizingInspector, handleMouseMove, handleMouseUp]);

    const handleAppLaunch = useCallback(async (appId: string, fileSrc?: string, mediaType?: 'audio' | 'video') => {
        const installedApp = installedApps.find(app => app.id === appId);
        const shortcut = shortcuts.find(s => s.id === appId);

        if (shortcut?.type === 'url') {
            launchApp('browser', { type: 'browser', title: shortcut.name, appUrl: shortcut.target });
            return;
        }

        if (appId === 'terminal') {
            launchApp('terminal', { type: 'terminal', title: 'Terminal' });
        } else if (appId === 'browser') {
            launchApp('browser', { type: 'browser', title: 'Browser' });
        } else if (appId === 'settings') {
            launchApp('settings', { type: 'settings', title: 'System Settings' });
            setCurrentView('desktop');
        } else if (appId === 'media') {
            launchApp('media', {
                type: 'media',
                title: 'Media Player',
                src: fileSrc || '',
                mediaType: mediaType || 'audio'
            });
            setCurrentView('desktop');
        } else if (appId === 'app-store') {
            setCurrentView('app-store');
        } else if (installedApp || appId === 'firefox' || appId === 'chrome' || appId === 'antigravity') {
            const isMac = navigator.userAgent.includes('Mac');
            if ((installedApp?.type === 'native-app') || appId === 'firefox' || appId === 'chrome' || appId === 'antigravity') {
                let binary = appId;
                if (appId === 'chrome') binary = isMac ? 'Google Chrome' : 'chromium';
                if (appId === 'firefox') binary = 'firefox';
                if (appId === 'antigravity') binary = 'antigravity';

                if (isMac) {
                    const macName = installedApp?.systemPath || (appId === 'chrome' ? 'Google Chrome' :
                        appId === 'firefox' ? 'Firefox' :
                            appId === 'antigravity' ? 'Anti-Gravity' :
                                appId === 'vscode' ? 'Visual Studio Code' :
                                    appId === 'vlc' ? 'VLC' :
                                        installedApp?.name || appId);
                    Command.create('open', ['-a', macName]).execute()
                        .catch(err => console.error(`Failed to launch Mac app ${macName}:`, err));
                } else {
                    Command.create(binary).execute()
                        .catch(err => console.error(`Failed to launch Linux binary ${binary}:`, err));
                }
            } else if (installedApp) {
                launchApp(installedApp?.name || appId, {
                    type: 'browser',
                    title: installedApp?.name || appId,
                    appUrl: installedApp?.url || 'https://www.bing.com'
                });
                setCurrentView('desktop');
            }
        } else {
            setCurrentView(appId);
        }
    }, [installedApps, shortcuts, launchApp, setCurrentView]);

    const lastWheelTime = useRef(0);

    useEffect(() => {
        const handleWheel = (e: WheelEvent) => {
            if (isOverviewOpen) return;

            if (Math.abs(e.deltaX) > Math.abs(e.deltaY)) {
                e.preventDefault();
            }

            const now = Date.now();
            if (now - lastWheelTime.current < 600) return;

            if (Math.abs(e.deltaX) > 40) {
                if (e.deltaX > 0 && activeSpaceIndex < spaces.length - 1) {
                    lastWheelTime.current = now;
                    switchSpace(activeSpaceIndex + 1);
                } else if (e.deltaX < 0 && activeSpaceIndex > 0) {
                    lastWheelTime.current = now;
                    switchSpace(activeSpaceIndex - 1);
                }
            }
        };

        window.addEventListener('wheel', handleWheel, { passive: false });
        return () => window.removeEventListener('wheel', handleWheel);
    }, [activeSpaceIndex, spaces.length, switchSpace, isOverviewOpen]);

    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if (e.metaKey && e.key === 'w') {
                e.preventDefault();
                closeActiveWindow();
            }

            if (e.altKey && e.key === 'Tab') {
                e.preventDefault();
                setIsSwitcherOpen(true);
                setSwitcherSelectedIndex(prev => prev + 1);
            }

            if (e.ctrlKey) {
                if (e.key === 'ArrowRight' && activeSpaceIndex < spaces.length - 1) {
                    e.preventDefault();
                    switchSpace(activeSpaceIndex + 1);
                } else if (e.key === 'ArrowLeft' && activeSpaceIndex > 0) {
                    e.preventDefault();
                    switchSpace(activeSpaceIndex - 1);
                }
            }
        };

        const handleKeyUp = (e: KeyboardEvent) => {
            if (isSwitcherOpen && e.key === 'Alt') {
                // Gather all windows globally
                const allWindows = spaces.flatMap(space =>
                    space.windows.map(win => ({ ...win, spaceIndex: spaces.findIndex(s => s.id === space.id) }))
                );

                const sortedWindows = [...allWindows].sort((a, b) =>
                    (b.zIndex || 0) - (a.zIndex || 0)
                );

                const selectedWin = sortedWindows[switcherSelectedIndex % sortedWindows.length];
                if (selectedWin) {
                    // Switch space if needed
                    if (selectedWin.spaceIndex !== activeSpaceIndex) {
                        switchSpace(selectedWin.spaceIndex);
                    }
                    focusWindow(selectedWin.id);
                }
                setIsSwitcherOpen(false);
                setSwitcherSelectedIndex(0);
            }
        };

        window.addEventListener('keydown', handleKeyDown);
        window.addEventListener('keyup', handleKeyUp);
        return () => {
            window.removeEventListener('keydown', handleKeyDown);
            window.removeEventListener('keyup', handleKeyUp);
        };
    }, [isSwitcherOpen, switcherSelectedIndex, spaces, activeSpaceIndex, closeActiveWindow, focusWindow, switchSpace]);

    const getExplorerPath = (view: string) => {
        const home = '/Users/vegarberentsen';
        switch (view.toLowerCase()) {
            case 'desktop': return `${home}/Desktop`;
            case 'documents': return `${home}/Documents`;
            case 'downloads': return `${home}/Downloads`;
            case 'applications': return `/Applications`;
            case 'recents': return home;
            case 'pictures': return `${home}/Pictures`;
            case 'music': return `${home}/Music`;
            case 'movies': return `${home}/Movies`;
            case 'root': return '/';
            case 'network': return '/Volumes';
            default: return null;
        }
    };

    return (
        <div className="relative w-screen h-screen overflow-hidden font-sans text-foreground flex flex-col bg-background">
            {/* 1. Global Static MenuBar */}
            <MenuBar
                showSidebar={showSidebar}
                setShowSidebar={setShowSidebar}
                rightPanelMode={rightPanelMode}
                setRightPanelMode={setRightPanelMode}
                onToggleOverview={toggleOverview}
            />

            <div className="relative flex-1 flex overflow-hidden">
                {/* 2. Global Static Sidebar */}
                {showSidebar && (
                    <div className="relative flex-shrink-0 z-30 h-full border-r border-black/5 dark:border-white/5" style={{ width: sidebarWidth }}>
                        <Sidebar
                            activeId={currentView}
                            onSelect={(item) => handleAppLaunch(item.slug || item.id)}
                        />
                        <div
                            className="absolute top-0 right-0 w-1.5 h-full cursor-col-resize hover:bg-amber-500/50 transition-colors z-50 delay-75"
                            onMouseDown={() => setIsResizingSidebar(true)}
                        />
                    </div>
                )}

                <div className="relative flex-1 flex min-w-0 h-full overflow-hidden">
                    {/* 3. Multi-plane Workspace Layer (Keeps all spaces mounted for multitasking) */}
                    <div className="relative flex-1 h-full overflow-hidden">
                        {spaces.map((space, index) => (
                            <motion.div
                                key={space.id}
                                animate={{
                                    x: (index - activeSpaceIndex) * 100 + '%',
                                    scale: index === activeSpaceIndex ? 1 : 0.98,
                                    opacity: Math.abs(index - activeSpaceIndex) > 1 ? 0 : 1
                                }}
                                transition={{ x: { type: "spring", stiffness: 260, damping: 35, mass: 1.2 }, opacity: { duration: 0.2 } }}
                                className="absolute inset-0 w-full h-full flex flex-col overflow-hidden"
                                style={{
                                    pointerEvents: index === activeSpaceIndex ? 'auto' : 'none',
                                    visibility: Math.abs(index - activeSpaceIndex) > 1 ? 'hidden' : 'visible'
                                }}
                            >
                                <Wallpaper />
                                <main
                                    ref={index === activeSpaceIndex ? desktopRef : null}
                                    className="relative flex-1 overflow-hidden"
                                >
                                    {/* Desktop/Explorer Layer */}
                                    <div className="absolute inset-0 z-0 h-full w-full">
                                        {currentView === 'desktop' ? (
                                            <div className="p-8 h-full" />
                                        ) : (
                                            <div className="h-full w-full">
                                                {currentView === 'dashboard' && (
                                                    <div className="h-full p-8 pt-20 overflow-y-auto">
                                                        <h2 className="text-3xl font-bold mb-6">Dashboard</h2>
                                                        <div className="grid grid-cols-3 gap-6">
                                                            <Dashboard type="system" />
                                                            <Dashboard type="weather" />
                                                            <Dashboard type="calendar" />
                                                        </div>
                                                    </div>
                                                )}
                                                {currentView === 'app-store' && <div className="h-full pt-16"><AppStore /></div>}
                                                {getExplorerPath(currentView) && (
                                                    <FileExplorer
                                                        initialPath={getExplorerPath(currentView)!}
                                                        onOpenFile={(path, type) => handleAppLaunch('media', path, type)}
                                                    />
                                                )}
                                            </div>
                                        )}
                                    </div>

                                    {/* Window Layer */}
                                    {space.windows.map((win) => (
                                        <Window
                                            key={win.id}
                                            id={win.id}
                                            spaceId={space.id}
                                            constraintsRef={desktopRef}
                                            title={win.title}
                                            x={win.x || 100}
                                            y={win.y || 80}
                                            width={win.width || 600}
                                            height={win.height || 400}
                                            zIndex={win.zIndex || 1}
                                            onFocus={() => {
                                                if (index !== activeSpaceIndex) switchSpace(index);
                                                focusWindow(win.id);
                                            }}
                                            onClose={() => closeWindow(space.id, win.id)}
                                        >
                                            {win.type === 'terminal' ? (
                                                <Terminal />
                                            ) : win.type === 'browser' ? (
                                                <Browser isAppMode={!!win.appUrl} initialUrl={win.appUrl} />
                                            ) : win.type === 'media' ? (
                                                <MediaPlayer src={win.src || ''} type={win.mediaType || 'audio'} autoPlay />
                                            ) : win.type === 'settings' ? (
                                                <SettingsApp initialSection={win.initialSection} />
                                            ) : (
                                                <div className="prose prose-invert">
                                                    <h1 className="text-2xl font-bold mb-4">{win.title}</h1>
                                                    <p className="text-foreground/80">{win.content}</p>
                                                </div>
                                            )}
                                        </Window>
                                    ))}
                                </main>
                            </motion.div>
                        ))}
                    </div>

                    {/* 4. Global Static Inspector */}
                    {rightPanelMode && (
                        <div className="relative h-full flex-shrink-0 z-40 bg-background/50 backdrop-blur-md border-l border-white/10" style={{ width: inspectorWidth }}>
                            <div
                                className="absolute top-0 left-0 w-1.5 h-full cursor-col-resize hover:bg-amber-500/50 transition-colors z-50"
                                onMouseDown={() => setIsResizingInspector(true)}
                            />
                            <Inspector mode={rightPanelMode as InspectorMode} theme={theme} setTheme={setTheme} />
                        </div>
                    )}
                </div>
            </div>

            {/* 5. Global Overlays */}
            <div className="absolute inset-0 pointer-events-none flex flex-col z-50">
                <div className="flex-1" />
                <div className="flex-none pointer-events-auto">
                    <Dock onAppClick={handleAppLaunch} />
                </div>
            </div>

            <AnimatePresence>
                {isOverviewOpen && (
                    <motion.div
                        initial={{ opacity: 0 }}
                        animate={{ opacity: 1 }}
                        exit={{ opacity: 0 }}
                        className="absolute inset-0 z-[100] bg-black/40 backdrop-blur-md flex flex-col items-center justify-center gap-8"
                        onClick={toggleOverview}
                    >
                        <div className="flex gap-6 max-w-[90vw] overflow-x-auto p-8" onClick={e => e.stopPropagation()}>
                            {spaces.map((space, index) => (
                                <div key={space.id} className="group relative">
                                    <div
                                        onClick={() => {
                                            switchSpace(index);
                                            toggleOverview();
                                        }}
                                        className={cn(
                                            "w-64 h-40 rounded-xl border-2 transition-all cursor-pointer bg-background overflow-hidden relative shadow-2xl",
                                            activeSpaceIndex === index ? "border-primary scale-110" : "border-white/10 hover:border-white/30 hover:scale-105"
                                        )}
                                    >
                                        <div className="absolute inset-0 flex items-center justify-center opacity-50">
                                            <span className="text-4xl font-bold">{index + 1}</span>
                                        </div>
                                    </div>
                                    <button
                                        className="absolute -top-3 -right-3 p-1.5 bg-red-500 rounded-full opacity-0 group-hover:opacity-100 transition-opacity"
                                        onClick={(e) => {
                                            e.stopPropagation();
                                            const newSpaces = spaces.filter(s => s.id !== space.id);
                                            setSpaces(newSpaces);
                                            if (activeSpaceIndex >= newSpaces.length) setActiveSpace(Math.max(0, newSpaces.length - 1));
                                        }}
                                    >
                                        <X size={12} className="text-white" />
                                    </button>
                                </div>
                            ))}
                            <button
                                onClick={() => {
                                    const nextIndex = spaces.length;
                                    setSpaces([...spaces, { id: Date.now(), windows: [] }]);
                                    switchSpace(nextIndex);
                                    toggleOverview();
                                }}
                                className="w-64 h-40 rounded-xl border-2 border-dashed border-white/20 flex items-center justify-center hover:bg-white/5 transition-colors cursor-pointer"
                            >
                                <Plus size={32} className="text-white/30" />
                            </button>
                        </div>
                    </motion.div>
                )}
            </AnimatePresence>

            <Spotlight />
            <AppSwitcher isOpen={isSwitcherOpen} selectedIndex={switcherSelectedIndex} />
        </div>
    );
}
