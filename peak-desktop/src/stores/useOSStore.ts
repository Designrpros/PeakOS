import { create } from 'zustand';

export interface WindowData {
    id: number;
    title: string;
    type: 'text' | 'terminal' | 'browser' | 'media' | 'settings' | 'antigravity';
    content?: string;
    src?: string;
    mediaType?: 'audio' | 'video';
    appUrl?: string; // For web apps
    x?: number;
    y?: number;
    width?: number;
    height?: number;
    isNative?: boolean;
    initialSection?: string; // For apps with internal tabs (Settings)
    initialPath?: string; // For file explorer
    zIndex?: number;
}

export interface Space {
    id: number;
    windows: WindowData[];
}

export interface InstalledApp {
    id: string;
    name: string;
    icon: string; // lucide icon name or image url
    url?: string; // For web apps
    systemPath?: string; // For native apps
    type: 'system' | 'web-app' | 'native-app';
}

export interface User {
    fullName: string;
    username: string;
    avatar: string; // lucide icon name
}

export interface Shortcut {
    id: string;
    name: string;
    icon: string;
    type: 'app' | 'url' | 'file';
    target: string; // appId, url, or path
    pinnedToDock: boolean;
    pinnedToSidebar: boolean;
}

interface OSState {
    // Spaces
    spaces: Space[];
    activeSpaceIndex: number;
    isOverviewOpen: boolean;

    // Installed Apps
    installedApps: InstalledApp[];
    users: User[];

    // Layout
    sidebarWidth: number;
    showSidebar: boolean;
    theme: 'light' | 'dark' | 'system';
    currentView: string; // 'desktop', 'dashboard', 'terminal', etc.
    isSetupComplete: boolean;
    isLoggedIn: boolean;
    isGuestModeEnabled: boolean;
    user: User | null;

    // Spotlight
    isSpotlightOpen: boolean;

    // Shortcuts
    shortcuts: Shortcut[];

    // Actions
    setSpaces: (spaces: Space[]) => void;
    setActiveSpace: (index: number) => void;
    switchSpace: (index: number) => void;
    toggleOverview: () => void;
    toggleSpotlight: () => void;
    setCurrentView: (view: string) => void;
    setShowSidebar: (show: boolean) => void;
    setSidebarWidth: (width: number) => void;
    launchApp: (appId: string, options?: Partial<WindowData>) => void;
    installApp: (app: InstalledApp) => void;
    uninstallApp: (appId: string) => void;
    closeWindow: (spaceId: number, windowId: number) => void;
    updateWindow: (spaceId: number, windowId: number, updates: Partial<WindowData>) => void;
    focusWindow: (windowId: number) => void;
    snapWindow: (windowId: number, type: 'left' | 'right' | 'top' | 'bottom' | 'maximize' | 'center' | 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right' | 'third-left' | 'third-center' | 'third-right') => void;
    completeSetup: (userData?: User) => void;
    setUser: (user: User) => void;
    login: () => void;
    logout: () => void;
    resetSetup: () => void;
    setGuestMode: (enabled: boolean) => void;
    addUser: (user: User) => void;
    selectUser: (user: User | null) => void;

    // Shortcut Actions
    addShortcut: (shortcut: Shortcut) => void;
    removeShortcut: (id: string) => void;
    updateShortcut: (id: string, updates: Partial<Shortcut>) => void;
    setShortcuts: (shortcuts: Shortcut[]) => void;
    setTheme: (theme: 'light' | 'dark' | 'system') => void;
    closeActiveWindow: () => void;
}

export const useOSStore = create<OSState>((set, get) => ({
    spaces: [
        {
            id: 0,
            windows: []
        },
        {
            id: 1,
            windows: []
        },
        {
            id: 2,
            windows: []
        }
    ],
    activeSpaceIndex: 0,
    isOverviewOpen: false,
    isSpotlightOpen: false,
    sidebarWidth: 256,
    showSidebar: true,
    theme: (localStorage.getItem('peakos-theme') as any) || 'dark',
    currentView: 'desktop',
    isLoggedIn: false,
    isGuestModeEnabled: localStorage.getItem('peakos-guest-mode') === 'true',
    users: (() => {
        const storedUsers = JSON.parse(localStorage.getItem('peakos-users') || '[]');
        const singleUser = JSON.parse(localStorage.getItem('peakos-user') || 'null');

        // Migration: If we have a single user but they aren't in the users list, add them
        if (singleUser && !storedUsers.find((u: User) => u.username === singleUser.username)) {
            const migratedUsers = [...storedUsers, singleUser];
            localStorage.setItem('peakos-users', JSON.stringify(migratedUsers));
            return migratedUsers;
        }
        return storedUsers;
    })(),
    isSetupComplete: (() => {
        const flag = localStorage.getItem('peakos-setup-complete') === 'true';
        const usersCount = JSON.parse(localStorage.getItem('peakos-users') || '[]').length;
        const guestEnabled = localStorage.getItem('peakos-guest-mode') === 'true';
        const singleUser = localStorage.getItem('peakos-user');

        // System is complete if flag is true AND we have at least one identity (registered user or guest)
        return flag && (usersCount > 0 || singleUser !== null || guestEnabled);
    })(),
    user: JSON.parse(localStorage.getItem('peakos-user') || 'null'),
    installedApps: (() => {
        const stored = JSON.parse(localStorage.getItem('peakos-installed-apps') || '[]');
        const defaultApps: InstalledApp[] = [
            { id: 'terminal', name: 'Terminal', icon: 'terminal', type: 'system' },
            { id: 'browser', name: 'Browser', icon: 'globe', type: 'system' },
            { id: 'media', name: 'Media Player', icon: 'music', type: 'system' },
            { id: 'settings', name: 'Settings', icon: 'settings', type: 'system' },
            { id: 'app-store', name: 'App Store', icon: 'shopping-bag', type: 'system' },
        ];

        // Add defaults if they don't exist
        let updated = [...stored];
        for (const def of defaultApps) {
            if (!updated.find(a => a.id === def.id)) {
                updated.push(def);
            }
        }

        if (updated.length !== stored.length) {
            localStorage.setItem('peakos-installed-apps', JSON.stringify(updated));
        }
        return updated;
    })(),
    shortcuts: JSON.parse(localStorage.getItem('peakos-shortcuts') || '[]'),

    setSpaces: (spaces) => set({ spaces }),
    setActiveSpace: (index) => set({ activeSpaceIndex: index }),
    switchSpace: (newIndex) => {
        const { activeSpaceIndex, spaces } = get();
        if (newIndex >= 0 && newIndex < spaces.length && newIndex !== activeSpaceIndex) {
            set({
                activeSpaceIndex: newIndex
            });
        }
    },
    toggleOverview: () => set((state) => ({ isOverviewOpen: !state.isOverviewOpen })),
    toggleSpotlight: () => set((state) => ({ isSpotlightOpen: !state.isSpotlightOpen })),
    setShowSidebar: (show) => set({ showSidebar: show }),
    setSidebarWidth: (width) => set({ sidebarWidth: width }),
    setCurrentView: (view) => set({ currentView: view }),

    installApp: (app) => {
        const { installedApps } = get();
        if (installedApps.some(a => a.id === app.id)) return;

        const newApps = [...installedApps, app];
        set({ installedApps: newApps });
        localStorage.setItem('peakos-installed-apps', JSON.stringify(newApps));
    },

    uninstallApp: (appId) => {
        const { installedApps } = get();
        const newApps = installedApps.filter(a => a.id !== appId);
        set({ installedApps: newApps });
        localStorage.setItem('peakos-installed-apps', JSON.stringify(newApps));
    },

    launchApp: (appId, options = {}) => {
        const { spaces, activeSpaceIndex, switchSpace, focusWindow, setCurrentView } = get();

        // 1. Handle View-based Apps (Views that replace the desktop content)
        const isViewApp = ['dashboard', 'desktop', 'app-store', 'documents', 'downloads', 'pictures', 'music', 'movies'].includes(appId);
        if (isViewApp) {
            setCurrentView(appId);
            // If it's a view app, we usually don't want a window, but we might want to toggle it back to 'desktop' if it's already active? 
            // For now, simple switch is better.
            return;
        }

        // 2. Map appId to Type if not explicitly provided
        let type = options.type;
        if (!type) {
            switch (appId) {
                case 'terminal': type = 'terminal'; break;
                case 'browser': type = 'browser'; break;
                case 'media': type = 'media'; break;
                case 'settings': type = 'settings'; break;
                default: type = 'text';
            }
        }

        // 3. Check if an app of this type or title is already open in ANY space
        for (let i = 0; i < spaces.length; i++) {
            const existingWin = spaces[i].windows.find(w =>
                (type && w.type === type) || w.title === (options.title || appId)
            );

            if (existingWin) {
                if (i !== activeSpaceIndex) {
                    switchSpace(i);
                }
                focusWindow(existingWin.id);
                setCurrentView('desktop'); // Ensure we are on the desktop to see the window
                return;
            }
        }

        const activeSpace = spaces[activeSpaceIndex];
        const windowCount = activeSpace.windows.length;
        const offset = (windowCount % 10) * 20;
        const maxZ = activeSpace.windows.reduce((max, w) => Math.max(max, w.zIndex || 0), 0);

        const newWindow: WindowData = {
            id: Date.now(),
            title: options.title || appId.charAt(0).toUpperCase() + appId.slice(1),
            type: (type as any) || 'text',
            x: 100 + offset,
            y: 80 + offset,
            zIndex: maxZ + 1,
            ...options
        };

        const newSpaces = spaces.map((s, i) =>
            i === activeSpaceIndex
                ? { ...s, windows: [...s.windows, newWindow] }
                : s
        );

        set({ spaces: newSpaces, currentView: 'desktop' }); // Auto-switch to desktop
    },

    focusWindow: (windowId) => {
        const { spaces, activeSpaceIndex } = get();
        const activeSpace = spaces[activeSpaceIndex];

        const maxZ = activeSpace.windows.reduce((max, w) => Math.max(max, w.zIndex || 0), 0);
        const windowToFocus = activeSpace.windows.find(w => w.id === windowId);

        if (!windowToFocus) return;

        const newSpaces = spaces.map((s, i) =>
            i === activeSpaceIndex
                ? {
                    ...s,
                    windows: s.windows.map(w =>
                        w.id === windowId ? { ...w, zIndex: maxZ + 1 } : w
                    )
                }
                : s
        );

        set({ spaces: newSpaces });
    },

    snapWindow: (windowId, type) => {
        const { spaces, activeSpaceIndex } = get();
        const space = spaces[activeSpaceIndex];
        const win = space.windows.find(w => w.id === windowId);
        if (!win) return;

        const topOffset = 32; // MenuBar height
        const screenW = window.innerWidth;
        const screenH = window.innerHeight - topOffset;

        let updates: Partial<WindowData> = {};

        switch (type) {
            case 'left': updates = { x: 0, y: 0, width: screenW / 2, height: screenH }; break;
            case 'right': updates = { x: screenW / 2, y: 0, width: screenW / 2, height: screenH }; break;
            case 'top': updates = { x: 0, y: 0, width: screenW, height: screenH / 2 }; break;
            case 'bottom': updates = { x: 0, y: screenH / 2, width: screenW, height: screenH / 2 }; break;

            case 'top-left': updates = { x: 0, y: 0, width: screenW / 2, height: screenH / 2 }; break;
            case 'top-right': updates = { x: screenW / 2, y: 0, width: screenW / 2, height: screenH / 2 }; break;
            case 'bottom-left': updates = { x: 0, y: screenH / 2, width: screenW / 2, height: screenH / 2 }; break;
            case 'bottom-right': updates = { x: screenW / 2, y: screenH / 2, width: screenW / 2, height: screenH / 2 }; break;

            case 'maximize': updates = { x: 0, y: 0, width: screenW, height: screenH }; break;
            case 'center': updates = { x: screenW * 0.15, y: screenH * 0.1, width: screenW * 0.7, height: screenH * 0.8 }; break;

            case 'third-left': updates = { x: 0, y: 0, width: screenW / 3, height: screenH }; break;
            case 'third-center': updates = { x: screenW / 3, y: 0, width: screenW / 3, height: screenH }; break;
            case 'third-right': updates = { x: (screenW / 3) * 2, y: 0, width: screenW / 3, height: screenH }; break;
        }

        const newSpaces = spaces.map((s, i) =>
            i === activeSpaceIndex
                ? { ...s, windows: s.windows.map(w => w.id === windowId ? { ...w, ...updates } : w) }
                : s
        );

        set({ spaces: newSpaces });
    },

    closeWindow: (spaceId, windowId) => {
        const { spaces } = get();
        const newSpaces = spaces.map(space =>
            space.id === spaceId
                ? { ...space, windows: space.windows.filter(w => w.id !== windowId) }
                : space
        );
        set({ spaces: newSpaces });
    },

    updateWindow: (spaceId: number, windowId: number, updates) => {
        const { spaces } = get();
        const newSpaces = spaces.map(space =>
            space.id === spaceId
                ? { ...space, windows: space.windows.map(w => w.id === windowId ? { ...w, ...updates } : w) }
                : space
        );
        set({ spaces: newSpaces });
    },

    setUser: (user) => {
        const { users } = get();
        set({ user });
        localStorage.setItem('peakos-user', JSON.stringify(user));

        // Update user in users list if it exists
        const userExists = users.find(u => u.username === user.username);
        if (userExists) {
            const newUsers = users.map(u => u.username === user.username ? user : u);
            set({ users: newUsers });
            localStorage.setItem('peakos-users', JSON.stringify(newUsers));
        }
    },

    addUser: (newUser) => {
        const { users } = get();
        if (users.find(u => u.username === newUser.username)) return;
        const newUsers = [...users, newUser];
        set({ users: newUsers });
        localStorage.setItem('peakos-users', JSON.stringify(newUsers));
    },

    selectUser: (user) => set({ user }),

    login: () => set({ isLoggedIn: true }),
    logout: () => set({ isLoggedIn: false }),

    completeSetup: (userData) => {
        if (userData) {
            const { users } = get();
            const newUsers = users.find(u => u.username === userData.username)
                ? users.map(u => u.username === userData.username ? userData : u)
                : [...users, userData];

            set({
                user: userData,
                users: newUsers,
                isSetupComplete: true,
                isLoggedIn: true
            });
            localStorage.setItem('peakos-user', JSON.stringify(userData));
            localStorage.setItem('peakos-users', JSON.stringify(newUsers));
        } else {
            set({ isSetupComplete: true, isGuestModeEnabled: true });
            localStorage.setItem('peakos-guest-mode', 'true');
        }
        localStorage.setItem('peakos-setup-complete', 'true');
    },

    resetSetup: () => {
        localStorage.removeItem('peakos-setup-complete');
        localStorage.removeItem('peakos-user');
        localStorage.removeItem('peakos-users');
        localStorage.removeItem('peakos-guest-mode');
        set({ isSetupComplete: false, user: null, users: [], isLoggedIn: false, isGuestModeEnabled: false });
    },

    setGuestMode: (enabled: boolean) => {
        set({ isGuestModeEnabled: enabled });
        localStorage.setItem('peakos-guest-mode', enabled.toString());
    },

    addShortcut: (shortcut) => {
        const { shortcuts } = get();
        if (shortcuts.some(s => s.id === shortcut.id)) return;
        const newShortcuts = [...shortcuts, shortcut];
        set({ shortcuts: newShortcuts });
        localStorage.setItem('peakos-shortcuts', JSON.stringify(newShortcuts));
    },

    removeShortcut: (id) => {
        const { shortcuts } = get();
        const newShortcuts = shortcuts.filter(s => s.id !== id);
        set({ shortcuts: newShortcuts });
        localStorage.setItem('peakos-shortcuts', JSON.stringify(newShortcuts));
    },

    updateShortcut: (id, updates) => {
        const { shortcuts } = get();
        const newShortcuts = shortcuts.map(s => s.id === id ? { ...s, ...updates } : s);
        set({ shortcuts: newShortcuts });
        localStorage.setItem('peakos-shortcuts', JSON.stringify(newShortcuts));
    },

    setShortcuts: (shortcuts) => {
        set({ shortcuts });
        localStorage.setItem('peakos-shortcuts', JSON.stringify(shortcuts));
    },
    setTheme: (theme) => {
        set({ theme });
        localStorage.setItem('peakos-theme', theme);
        const root = window.document.documentElement;
        if (theme === 'dark' || (theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
            root.classList.add('dark');
        } else {
            root.classList.remove('dark');
        }
    },
    closeActiveWindow: () => {
        const { spaces, activeSpaceIndex, closeWindow } = get();
        const activeSpace = spaces[activeSpaceIndex];
        if (activeSpace.windows.length === 0) return;

        const topWindow = activeSpace.windows.reduce((prev, current) =>
            (prev.zIndex || 0) > (current.zIndex || 0) ? prev : current
        );

        if (topWindow) {
            closeWindow(activeSpaceIndex, topWindow.id);
        }
    }
}));
