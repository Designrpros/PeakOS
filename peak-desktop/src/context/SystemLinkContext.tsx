import React, { createContext, useContext, useEffect, useRef, useState, useCallback } from 'react';
import { Command, Child } from '@tauri-apps/plugin-shell';
import { v4 as uuidv4 } from 'uuid';

// Types (Mirrors of what was in hook)
type JsonRpcRequest = {
    jsonrpc: "2.0";
    method: string;
    params?: any;
    id: string;
};


interface SystemLinkContextType {
    isConnected: boolean;
    initError: string | null;
    callTool: (name: string, args?: Record<string, any>) => Promise<any>;
    onNotification: (method: string, handler: (params: any) => void) => () => void;
}

const SystemLinkContext = createContext<SystemLinkContextType | null>(null);

export function SystemLinkProvider({ children }: { children: React.ReactNode }) {
    const [isConnected, setIsConnected] = useState(false);
    const [initError, setInitError] = useState<string | null>(null);
    const childProcess = useRef<Child | null>(null);
    const pendingRequests = useRef<Map<string, { resolve: (val: any) => void, reject: (err: any) => void }>>(new Map());
    const notificationListeners = useRef<Map<string, Set<(params: any) => void>>>(new Map());
    const bufferRef = useRef<string>("");

    // Initialize Connection (Once)
    useEffect(() => {
        let isMounted = true;

        const init = async () => {
            try {
                // In production, it's 'bin/peak-intelligence'
                // In dev mode, we might need a fallback path
                let cmd = Command.sidecar('bin/peak-intelligence');

                // Attach listeners BEFORE spawn
                cmd.stdout.on('data', (line) => {
                    console.log('[Sidecar Output]', line);
                    bufferRef.current += line;

                    let boundary = bufferRef.current.indexOf('\n');
                    while (boundary !== -1) {
                        const message = bufferRef.current.substring(0, boundary).trim();
                        bufferRef.current = bufferRef.current.substring(boundary + 1);

                        if (message) {
                            try {
                                const payload: any = JSON.parse(message);

                                // Handle Response
                                if (payload.id && pendingRequests.current.has(payload.id)) {
                                    const { resolve, reject } = pendingRequests.current.get(payload.id)!;
                                    if (payload.error) {
                                        reject(new Error(payload.error.message));
                                    } else {
                                        resolve(payload.result);
                                    }
                                    pendingRequests.current.delete(payload.id);
                                }
                                // Handle Notification (Server to Client)
                                else if (payload.method && !payload.id) {
                                    const listeners = notificationListeners.current.get(payload.method);
                                    if (listeners) {
                                        listeners.forEach(handler => handler(payload.params));
                                    }
                                }
                            } catch (e) {
                                console.error("[Sidecar] Failed to parse JSON:", message, e);
                            }
                        }
                        boundary = bufferRef.current.indexOf('\n');
                    }
                });

                cmd.stderr.on('data', (line) => console.error(`[Sidecar Error]: ${line}`));

                cmd.on('close', (data) => {
                    console.log(`[Sidecar] Process finished with code ${data.code} and signal ${data.signal}`);
                    if (isMounted) setIsConnected(false);
                });

                cmd.on('error', (error) => {
                    console.error(`[Sidecar] Command error:`, error);
                    if (isMounted) setInitError(error.toString());
                });

                console.log("[Sidecar] Spawning...");
                const child = await cmd.spawn();
                console.log("[Sidecar] Spawned with PID:", child.pid);

                childProcess.current = child;
                if (isMounted) setIsConnected(true);

            } catch (err: any) {
                console.error("[Sidecar] Failed to spawn:", err);
                if (isMounted) setInitError(err.message || "Failed to start sidecar");
            }
        };

        init();

        return () => {
            isMounted = false;
            if (childProcess.current) {
                console.log("[Sidecar] Killing process...");
                childProcess.current.kill();
            }
        };
    }, []);

    const callTool = useCallback(async (name: string, args: Record<string, any> = {}) => {
        if (!childProcess.current) {
            throw new Error("Sidecar not connected");
        }

        const id = uuidv4();
        const request: JsonRpcRequest = {
            jsonrpc: "2.0",
            method: "tools/call",
            params: {
                name,
                arguments: args
            },
            id
        };

        const promise = new Promise<any>((resolve, reject) => {
            // 15s Timeout
            const timeout = setTimeout(() => {
                if (pendingRequests.current.has(id)) {
                    pendingRequests.current.delete(id);
                    reject(new Error("Request timed out (15s)"));
                }
            }, 15000);

            pendingRequests.current.set(id, {
                resolve: (val) => { clearTimeout(timeout); resolve(val); },
                reject: (err) => { clearTimeout(timeout); reject(err); }
            });
        });

        // Send
        try {
            await childProcess.current.write(JSON.stringify(request) + "\n");
        } catch (e) {
            pendingRequests.current.delete(id);
            throw e;
        }

        return promise;
    }, []);

    const onNotification = useCallback((method: string, handler: (params: any) => void) => {
        if (!notificationListeners.current.has(method)) {
            notificationListeners.current.set(method, new Set());
        }
        notificationListeners.current.get(method)!.add(handler);

        return () => {
            notificationListeners.current.get(method)?.delete(handler);
        };
    }, []);

    return (
        <SystemLinkContext.Provider value={{ isConnected, initError, callTool, onNotification }}>
            {children}
        </SystemLinkContext.Provider>
    );
}

// Hook to consume the context
export function useSystemLink() {
    const context = useContext(SystemLinkContext);
    if (!context) {
        throw new Error("useSystemLink must be used within a SystemLinkProvider");
    }
    return context;
}
