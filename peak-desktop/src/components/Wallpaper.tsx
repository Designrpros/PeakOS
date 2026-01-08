import { VectorBackground } from './VectorBackground';

export function Wallpaper() {
    return (
        <div className="absolute inset-0 w-full h-full z-0 bg-background transition-colors duration-500 pointer-events-none">
            <VectorBackground particleCount={60} color="100, 100, 100" />

            {/* Subtle Noise Texture for "Paper" feel */}
            <div className="absolute inset-0 opacity-[0.03] bg-[url('https://grainy-gradients.vercel.app/noise.svg')]" />
        </div>
    );
}
