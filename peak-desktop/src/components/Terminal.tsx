import { useEffect, useRef } from 'react';
import { Terminal as XTerm } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
import { WebLinksAddon } from 'xterm-addon-web-links';
import 'xterm/css/xterm.css';
import { useSystemLink } from '../hooks/useSystemLink';

export function Terminal() {
    const terminalRef = useRef<HTMLDivElement>(null);
    const xtermRef = useRef<XTerm | null>(null);
    const { callTool, onNotification, isConnected } = useSystemLink();

    useEffect(() => {
        if (!terminalRef.current || !isConnected) return;

        const term = new XTerm({
            cursorBlink: true,
            fontSize: 14,
            fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
            theme: {
                background: '#09090b', // zinc-950
                foreground: '#f4f4f5', // zinc-100
                cursor: '#f4f4f5',
            },
            allowProposedApi: true
        });

        const fitAddon = new FitAddon();
        term.loadAddon(fitAddon);
        term.loadAddon(new WebLinksAddon());

        term.open(terminalRef.current);
        xtermRef.current = term;

        // Start PTY Session
        const initTerminal = async () => {
            try {
                // Initial fit to get dimensions
                fitAddon.fit();
                const cols = term.cols;
                const rows = term.rows;

                await callTool('terminal_open', { rows: Math.max(1, rows), cols: Math.max(1, cols) });
                term.writeln('\x1b[1;32mPeakOS Terminal Connected\x1b[0m');
            } catch (err) {
                term.writeln(`\x1b[31mFailed to connect to shell: ${err}\x1b[0m`);
            }
        };

        initTerminal();

        // Handle Input
        const onDataDisposable = term.onData(data => {
            callTool('terminal_write', { data }).catch(e => console.error("PTY Write Error:", e));
        });

        // Handle Output
        const removeOutputListener = onNotification('terminal/output', (params) => {
            if (params?.data) {
                term.write(params.data);
            }
        });

        // Handle Resize with ResizeObserver for precise fitting during animations
        const resizeObserver = new ResizeObserver(() => {
            try {
                fitAddon.fit();
                callTool('terminal_resize', {
                    rows: Math.max(1, term.rows),
                    cols: Math.max(1, term.cols)
                }).catch(e => console.warn("PTY Resize Error:", e));
            } catch (e) {
                // Ignore resize errors if hidden
            }
        });

        if (terminalRef.current) {
            resizeObserver.observe(terminalRef.current);
        }

        return () => {
            resizeObserver.disconnect();
            onDataDisposable.dispose();
            removeOutputListener();
            term.dispose();
        };
    }, [isConnected]);

    return (
        <div className="w-full h-full bg-zinc-950 p-4 font-mono overflow-hidden">
            <div ref={terminalRef} className="w-full h-full" />
        </div>
    );
}
