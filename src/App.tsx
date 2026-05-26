import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { toast, ToastContainer } from "react-toastify";
import LeafStatePopup from "./LeafStatePopup";
import {
  BoardWithPlayers,
  Player,
  PlayerType,
  type BoardFieldVal,
} from "./utils/board";

const board = new BoardWithPlayers();

const App = () => {
  const [boardState, setBoardState] = useState({
    state: board.getState(),
    value: board.checkStateValue(),
    building: true,
  });

  const currentPlayer = {
    userPlayer: board.getPlayers()[1],
    botPlayer: board.getPlayers()[0],
  };

  const [hidePopup, setHidePopup] = useState(true);

  const updateState = () => {
    setBoardState((st) => ({
      ...st,
      state: board.getState(),
      value: board.checkStateValue(),
    }));
  };

  const userPlay = async (player: Player, position: BoardFieldVal) => {
    if (boardState.building) {
      toast.error("Game isn't ready, please wait");
      return;
    }
    const res = player.play(position);
    if (!res.success) toast.warn(res.message);
    await invoke("set_node", { board: board.getState() });
    updateState();
    if (currentPlayer.botPlayer.hasTurn) await botPlay();
  };

  const botPlay = async () => {
    const move: BoardFieldVal = await invoke("play_move");
    currentPlayer.botPlayer.play(move);
    await invoke("set_node", { board: board.getState() });
    updateState();
  };

  useEffect(() => {
    (async () => {
      const opts: { player: PlayerType } = {
        player: currentPlayer.botPlayer.hasTurn
          ? currentPlayer.botPlayer.type
          : currentPlayer.userPlayer.type,
      };
      await invoke("reset_bot", opts);
      if (currentPlayer.botPlayer.hasTurn) {
        await botPlay();
      }
      setBoardState((st) => ({ ...st, building: false }));
    })();

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const valueMap: { [key: string]: "win" | "lose" | "draw" } = {
    "-1": "win",
    "1": "lose",
    "0": "draw",
  };

  const resetGame = async () => {
    const type = board.resetBoard(true);
    const opts: { player: PlayerType } = { player: type };
    await invoke("reset_bot", opts);
    setBoardState((st) => ({ ...st, building: false }));
    updateState();
    if (currentPlayer.botPlayer.hasTurn) await botPlay();
  };

  useEffect(() => {
    const show = () => {
      if (Number.isFinite(boardState.value)) setHidePopup(false);
    };
    show();
  }, [boardState.value]);

  return (
    <>
      {Number.isFinite(boardState.value) && !hidePopup && (
        <LeafStatePopup
          hideFn={() => setHidePopup(true)}
          onRestart={resetGame}
          status={valueMap[boardState.value?.toString() || "0"]}
        />
      )}
      <div
        {...(Number.isFinite(boardState.value)
          ? {
              style: { cursor: "pointer", position: "relative" },
              onClick: () => setHidePopup(false),
              "aria-disabled": true,
            }
          : {})}
        className="min-h-screen bg-linear-to-br from-(--bg-app-from) via-(--bg-app-via) to-(--bg-app-to) flex items-center justify-center font-sans text-white p-4"
      >
        {Number.isFinite(boardState.value) && (
          <div className="absolute z-30 top-0 left-0 w-full h-full"></div>
        )}
        <div className="max-w-md w-full flex flex-col items-center gap-3">
          {/* Header Section */}
          <div className="text-center space-y-2">
            <h1 className="text-4xl md:text-5xl font-extrabold tracking-tight bg-clip-text text-transparent bg-linear-to-r from-(--text-header-from) to-(--text-header-to) drop-shadow-md">
              Tic Tac Toe
            </h1>
            <p className="text-(--text-primary) font-medium tracking-wide uppercase text-sm">
              Powered by Minimax
            </p>
          </div>

          {/* Turn Indicator / Scoreboard */}
          <div className="w-full flex justify-between items-center bg-white/10 backdrop-blur-md rounded-2xl p-5 shadow-2xl border border-white/10">
            <div className="flex flex-col items-center gap-1 w-1/3">
              <span className="text-xs text-(--text-primary) font-bold uppercase tracking-widest">
                Player
              </span>
              <span className="text-3xl font-black text-(--color-player-x) drop-shadow-md">
                X
              </span>
            </div>

            <div className="flex flex-col items-center justify-center w-1/3">
              <span className="text-[10px] uppercase tracking-widest text-(--text-primary-dim) mb-1">
                Status
              </span>
              <span className="px-3 py-1 rounded-full bg-(--badge-bg) text-(--badge-text) text-xs font-bold border border-(--badge-border)">
                {currentPlayer.userPlayer.hasTurn ? "Your Turn" : "AI turn"}
              </span>
            </div>

            <div className="flex flex-col items-center gap-1 w-1/3">
              <span className="text-xs text-(--text-primary) font-bold uppercase tracking-widest">
                Minimax AI
              </span>
              <span className="text-3xl font-black text-(--color-player-o) drop-shadow-md">
                O
              </span>
            </div>
          </div>

          <div className="px-10 disabled:opacity-65 py-4 rounded-lg text-center w-[95%] max-w-90 bg-linear-to-r from-(--btn-from) hover:from-(--btn-hover-from) to-(--btn-to) hover:to-(--btn-hover-to) text-white font-bold shadow-lg transition-all duration-300 active:scale-95 tracking-wider uppercase text-sm border border-white/10">
            {boardState.building ? "LOADING..." : "GAME HAS STARTED"}
          </div>

          {/* Board Container */}
          <div className="bg-white/5 backdrop-blur-xl p-4 md:p-6 rounded-3xl shadow-2xl border border-white/10">
            <div className="grid grid-cols-3 gap-3 md:gap-4">
              {boardState.state.map((row, rIndex) =>
                row.map((cell, cIndex) => (
                  <button
                    key={`${rIndex}-${cIndex}`}
                    style={{ opacity: boardState.building ? 0.6 : 1 }}
                    className="w-20 h-20 md:w-24 md:h-24 bg-white/5 hover:bg-white/15 transition-all duration-300 rounded-2xl flex items-center justify-center text-5xl md:text-6xl shadow-inner border border-white/5 group relative overflow-hidden"
                  >
                    {/* Hover effect highlight */}
                    <div
                      onClickCapture={() =>
                        userPlay(currentPlayer.userPlayer, {
                          col: cIndex,
                          row: rIndex,
                        })
                      }
                      className="absolute inset-0 bg-linear-to-tr from-white/0 to-white/5 opacity-0 group-hover:opacity-100 transition-opacity"
                    />

                    {cell === "Maximizer" && (
                      <span className="text-(--color-player-x) font-black drop-shadow-(--shadow-player-x) transform transition-transform duration-300 hover:scale-110">
                        X
                      </span>
                    )}
                    {cell === "Minimizer" && (
                      <span className="text-(--color-player-o) font-black drop-shadow-(--shadow-player-o) transform transition-transform duration-300 hover:scale-110">
                        O
                      </span>
                    )}
                  </button>
                )),
              )}
            </div>
          </div>
        </div>
      </div>

      <ToastContainer
        newestOnTop
        pauseOnHover
        position="top-center"
        theme="colored"
      />
    </>
  );
};

export default App;
