import { useState, useRef, useEffect } from 'react';
import { Play, Pause, Volume2, VolumeX } from 'lucide-react';
import { convertFileSrc } from '@tauri-apps/api/core';

interface MediaPlayerProps {
    src: string; // Local file path or URL
    type: 'audio' | 'video';
    autoPlay?: boolean;
}

export function MediaPlayer({ src, type, autoPlay = false }: MediaPlayerProps) {
    const mediaRef = useRef<HTMLVideoElement | HTMLAudioElement>(null);
    const [isPlaying, setIsPlaying] = useState(autoPlay);
    const [progress, setProgress] = useState(0);
    const [duration, setDuration] = useState(0);
    const [isMuted, setIsMuted] = useState(false);
    const [assetUrl, setAssetUrl] = useState('');

    useEffect(() => {
        // Convert local path to Tauri asset URL
        let url = convertFileSrc(src);

        // Manual fallback for Tauri v2 on macOS if convertFileSrc fails to convert
        if (url === src) {
            console.log("MediaPlayer: convertFileSrc returned raw path, using manual fallback.");
            // Encode the path segments but keep the slashes
            const encodedPath = src.split('/').map(encodeURIComponent).join('/');
            url = `http://asset.localhost${encodedPath}`;
        }

        console.log("MediaPlayer: Final URL:", url);
        setAssetUrl(url);
    }, [src]);

    useEffect(() => {
        const media = mediaRef.current;
        if (!media) return;

        const updateProgress = () => setProgress(media.currentTime);
        const updateDuration = () => setDuration(media.duration);
        const onEnd = () => setIsPlaying(false);

        media.addEventListener('timeupdate', updateProgress);
        media.addEventListener('loadedmetadata', updateDuration);
        media.addEventListener('ended', onEnd);

        if (autoPlay) media.play().catch(e => console.error("Autoplay failed", e));

        return () => {
            media.removeEventListener('timeupdate', updateProgress);
            media.removeEventListener('loadedmetadata', updateDuration);
            media.removeEventListener('ended', onEnd);
        };
    }, [assetUrl, autoPlay]);

    const togglePlay = () => {
        if (!mediaRef.current) return;
        if (isPlaying) {
            mediaRef.current.pause();
        } else {
            mediaRef.current.play();
        }
        setIsPlaying(!isPlaying);
    };

    const toggleMute = () => {
        if (!mediaRef.current) return;
        mediaRef.current.muted = !isMuted;
        setIsMuted(!isMuted);
    };

    const handleSeek = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (!mediaRef.current) return;
        const time = Number(e.target.value);
        mediaRef.current.currentTime = time;
        setProgress(time);
    };

    const formatTime = (seconds: number) => {
        if (isNaN(seconds)) return "00:00";
        const mins = Math.floor(seconds / 60);
        const secs = Math.floor(seconds % 60);
        return `${mins}:${secs.toString().padStart(2, '0')}`;
    };

    return (
        <div className="flex flex-col w-full h-full bg-black text-white overflow-hidden">
            {/* Media Area */}
            <div className="flex-1 relative flex items-center justify-center bg-zinc-950">
                {assetUrl && (
                    type === 'video' ? (
                        <video
                            ref={mediaRef as React.RefObject<HTMLVideoElement>}
                            src={assetUrl}
                            className="w-full h-full object-contain"
                            onClick={togglePlay}
                        />
                    ) : (
                        <div className="flex flex-col items-center gap-4">
                            <div className="w-32 h-32 rounded-full bg-zinc-800 flex items-center justify-center animate-pulse-slow">
                                <span className="text-4xl">🎵</span>
                            </div>
                            <audio ref={mediaRef as React.RefObject<HTMLAudioElement>} src={assetUrl} />
                        </div>
                    )
                )}
            </div>

            {/* Controls */}
            <div className="h-16 bg-zinc-900 border-t border-zinc-800 px-4 flex items-center gap-4">
                <button onClick={togglePlay} className="w-10 h-10 rounded-full bg-white text-black flex items-center justify-center hover:scale-105 transition-transform">
                    {isPlaying ? <Pause size={20} fill="currentColor" /> : <Play size={20} fill="currentColor" className="ml-0.5" />}
                </button>

                <div className="flex-1 flex flex-col gap-1">
                    <input
                        type="range"
                        min={0}
                        max={duration || 100}
                        value={progress}
                        onChange={handleSeek}
                        className="w-full h-1 bg-zinc-700 rounded-lg appearance-none cursor-pointer accent-amber-500 hover:h-2 transition-all"
                    />
                    <div className="flex justify-between text-xs text-zinc-400 font-mono">
                        <span>{formatTime(progress)}</span>
                        <span>{formatTime(duration)}</span>
                    </div>
                </div>

                <div className="flex items-center gap-2">
                    <button onClick={toggleMute} className="text-zinc-400 hover:text-white">
                        {isMuted ? <VolumeX size={18} /> : <Volume2 size={18} />}
                    </button>
                </div>
            </div>
        </div>
    );
}
