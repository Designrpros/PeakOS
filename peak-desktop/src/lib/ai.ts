import { AISettings } from '../components/Inspector';

export interface Message {
    role: 'system' | 'user' | 'assistant' | 'tool';
    content: string | null;
    tool_calls?: ToolCall[];
    id?: string;
    tool_call_id?: string;
    name?: string; // For tool results
}

export interface ToolCall {
    id: string;
    type: 'function';
    function: {
        name: string;
        arguments: string; // JSON string
    };
}

const PEAK_TOOLS = [
    {
        type: "function",
        function: {
            name: "list_processes",
            description: "List all running processes with PID, CPU, and Memory usage.",
            parameters: {
                type: "object",
                properties: {},
            },
        },
    },
    {
        type: "function",
        function: {
            name: "read_file",
            description: "Read content of a file from the system. Use this to read code files, config files, etc.",
            parameters: {
                type: "object",
                properties: {
                    path: { type: "string", description: "Absolute path to file" }
                },
                required: ["path"],
            },
        },
    },
    {
        type: "function",
        function: {
            name: "write_file",
            description: "Write content to a file. Use this to create or update files.",
            parameters: {
                type: "object",
                properties: {
                    path: { type: "string", description: "Absolute path to file" },
                    content: { type: "string", description: "Content to write" }
                },
                required: ["path", "content"],
            },
        },
    },
    {
        type: "function",
        function: {
            name: "kill_process",
            description: "Terminate a system process by PID. WARNING: Use with caution.",
            parameters: {
                type: "object",
                properties: {
                    pid: { type: "string", description: "Process ID to kill" }
                },
                required: ["pid"],
            },
        },
    }
];

export async function chatCompletion(
    messages: Message[],
    settings: AISettings
): Promise<Message> {
    const isLocal = settings.provider === 'local';

    // For now, we only support OpenRouter fully for Tools. 
    // Ollama tool support varies by model, but we can try to use the same schema.

    const endpoint = isLocal
        ? 'http://localhost:11434/v1/chat/completions'
        : 'https://openrouter.ai/api/v1/chat/completions';

    const headers: Record<string, string> = {
        'Content-Type': 'application/json',
    };

    if (!isLocal) {
        headers['Authorization'] = `Bearer ${settings.apiKey}`;
        headers['HTTP-Referer'] = 'https://peakos.org'; // Required by OpenRouter
        headers['X-Title'] = 'PeakOS';
    }

    const payload = {
        model: settings.model,
        messages: messages.map(m => ({
            role: m.role,
            content: m.content,
            tool_calls: m.tool_calls,
            tool_call_id: m.tool_call_id,
            name: m.name
        })),
        tools: PEAK_TOOLS,
        tool_choice: "auto",
        temperature: 0.7
    };

    try {
        const response = await fetch(endpoint, {
            method: 'POST',
            headers: headers as any,
            body: JSON.stringify(payload)
        });

        if (!response.ok) {
            const err = await response.text();
            throw new Error(`AI Provider Error (${response.status}): ${err}`);
        }

        const data = await response.json();
        const choice = data.choices[0];
        return choice.message;

    } catch (error) {
        console.error("Chat Completion failed:", error);
        throw error;
    }
}
