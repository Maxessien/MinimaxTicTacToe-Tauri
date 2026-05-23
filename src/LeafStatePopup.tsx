

interface LeafStatePopupProps {
  status: 'win' | 'lose' | 'draw';
  onRestart: () => void;
  hideFn: () => void;
}

const LeafStatePopup = ({ status, onRestart, hideFn }: LeafStatePopupProps) => {
  const content = {
    win: {
      title: 'Victory!',
      message: 'You have defeated the Minimax AI.',
      color: 'text-(--color-player-x)',
      shadow: 'drop-shadow-(--shadow-player-x)'
    },
    lose: {
      title: 'Defeat!',
      message: 'The Minimax AI has outmaneuvered you.',
      color: 'text-(--color-player-o)',
      shadow: 'drop-shadow-(--shadow-player-o)'
    },
    draw: {
      title: 'Draw!',
      message: 'A perfectly played game by both sides.',
      color: 'text-indigo-200',
      shadow: 'drop-shadow-md'
    }
  }[status];

  return (
    <div onClick={hideFn} className="fixed flex-col gap-2 inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-md p-4 transition-all duration-300">
      <div onClick={(e)=> e.stopPropagation()} className="flex w-full max-w-120 justify-end px-3 py-4">
        <button onClick={hideFn} className="font-medium text-(--text-primary) cursor-pointer text-3xl">X</button>
      </div>
      {/* Modal Card */}
      <div onClick={(e)=> e.stopPropagation()} className="bg-slate-900/90 backdrop-blur-xl border border-white/10 p-8 md:p-10 rounded-3xl shadow-2xl flex flex-col items-center text-center gap-6 max-w-sm w-full transform transition-all animate-in zoom-in-95 duration-500">
        
        {/* Decorative Top Accent */}
        <div className="w-16 h-1 rounded-full bg-white/20 mb-2"></div>

        {/* Status Title */}
        <h2 className={`text-4xl md:text-5xl font-black uppercase tracking-widest ${content.color} ${content.shadow} transition-all duration-300 hover:scale-105`}>
          {content.title}
        </h2>
        
        {/* Status Message */}
        <p className="text-(--text-primary) font-medium text-base md:text-lg px-2">
          {content.message}
        </p>

        {/* Action Button */}
        <button 
          onClick={onRestart}
          className="mt-4 w-full py-4 bg-linear-to-r from-(--btn-from) hover:from-(--btn-hover-from) to-(--btn-to) hover:to-(--btn-hover-to) text-white font-bold rounded-full shadow-(--btn-shadow) transition-all duration-300 active:scale-95 tracking-wider uppercase text-sm border border-white/10 group"
        >
          <span className="flex items-center justify-center gap-2">
            Play Again
            <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4 group-hover:rotate-180 transition-transform duration-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={3}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
          </span>
        </button>

      </div>
    </div>
  );
}

export default LeafStatePopup;