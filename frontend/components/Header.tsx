"use client";

export default function Header() {
  return (
    <header className="w-full py-12 flex justify-center items-center bg-transparent relative z-50 overflow-hidden">
      {/* Spotlight Background */}
      <div className="absolute top-0 left-1/2 -translate-x-1/2 w-[600px] h-[300px] bg-purple-500/20 rounded-full blur-[100px] animate-spotlight pointer-events-none" />

      <div className="relative group">
        {/* Glitch Layers */}
        <h1 className="absolute top-0 left-0 text-5xl md:text-7xl font-black tracking-tighter text-blue-500 opacity-50 animate-glitch translate-x-1 translate-y-1 pointer-events-none select-none">
          Knotorious Offline
        </h1>
        <h1 className="absolute top-0 left-0 text-5xl md:text-7xl font-black tracking-tighter text-pink-500 opacity-50 animate-glitch -translate-x-1 -translate-y-1 pointer-events-none select-none" style={{ animationDirection: 'reverse' }}>
          Knotorious Offline
        </h1>

        {/* Main Text */}
        <h1 className="relative text-5xl md:text-7xl font-black tracking-tighter text-transparent bg-clip-text bg-gradient-to-r from-white via-purple-200 to-white drop-shadow-[0_0_15px_rgba(168,85,247,0.5)] z-10">
          Knotorious Offline
        </h1>
      </div>
    </header>
  );
}
