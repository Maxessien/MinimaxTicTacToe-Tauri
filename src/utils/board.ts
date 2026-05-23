export type BoardRowsCols =
  | "a1"
  | "b1"
  | "c1"
  | "a2"
  | "b2"
  | "c2"
  | "a3"
  | "b3"
  | "c3";

export type BoardValue = "X" | "O";

export type BoardState = (PlayerType[] | null[])[];

export type BoardMap = Record<BoardRowsCols, BoardFieldVal>;

export type PlayerType = "Minimizer" | "Maximizer";

interface MethodResponse {
  success: boolean;
  message: string;
}

export interface BoardFieldVal {
  row: number;
  col: number;
}

const boardMap: BoardMap = {
  a1: { row: 0, col: 0 },
  b1: { row: 0, col: 1 },
  c1: { row: 0, col: 2 },
  a2: { row: 1, col: 0 },
  b2: { row: 1, col: 1 },
  c2: { row: 1, col: 2 },
  a3: { row: 2, col: 0 },
  b3: { row: 2, col: 1 },
  c3: { row: 2, col: 2 },
};

const getEmptyBoardState = (): BoardState => {
  return [
    [null, null, null],
    [null, null, null],
    [null, null, null],
  ];
};

class Player extends EventTarget {
  hasTurn: boolean;
  readonly type: PlayerType;
  readonly val: BoardValue;
  readonly board: Board;
  readonly goalScore: -1 | 1;

  constructor(type: PlayerType, val: BoardValue, board: Board) {
    super();
    this.type = type;
    this.val = val;
    this.board = board;
    this.goalScore = type === "Maximizer" ? 1 : -1;
    this.hasTurn = false;
  }

  play(position: BoardFieldVal): MethodResponse {
    if (!this.hasTurn) return { message: "Not Your Turn", success: false };

    const val = this.board.addValue(position, this);
    if (val.success) this.hasTurn = false;

    const event = new CustomEvent("just_played");
    this.dispatchEvent(event);
    return { message: val.message, success: val.success };
  }
}

class Board {
  protected state: BoardState;
  private boardMap: BoardMap;

  constructor(initState = getEmptyBoardState()) {
    this.state = initState;
    this.boardMap = boardMap;
  }

  getState(): BoardState {
    return structuredClone(this.state);
  }

  setState(newState: BoardState) {
    this.state = newState;
  }

  getMap(): BoardMap {
    return structuredClone(this.boardMap);
  }

  addValue(position: BoardFieldVal, val: Player): MethodResponse {
    const map = position;
    if (!map) return { message: "Position doesn't exists", success: false };
    if (this.state[map.row][map.col])
      return { message: "Position already has a value", success: false };
    this.state[map.row][map.col] = val.type;
    return { message: "Finished", success: true };
  }

  private getAllPossibleWinCom() {
    const rowWins = [this.state[0], this.state[1], this.state[2]];
    const colWins = [
      [this.state[0][0], this.state[1][0], this.state[2][0]],
      [this.state[0][1], this.state[1][1], this.state[2][1]],
      [this.state[0][2], this.state[1][2], this.state[2][2]],
    ];
    const diagonalWins = [
      [this.state[0][0], this.state[1][1], this.state[2][2]],
      [this.state[0][2], this.state[1][1], this.state[2][0]],
    ];

    return [...rowWins, ...colWins, ...diagonalWins];
  }

  private getGoalPlayer(): PlayerType | null {
    const allWins = this.getAllPossibleWinCom();
    for (const win of allWins) {
      if (win.some((w) => !w)) continue;
      else if (win.every((w) => w === win[0])) return win[0];
    }
    return null;
  }

  checkStateValue(): 0 | 1 | -1 | null {
    if (this.state.flat().every((v) => v)) return 0;
    const goalPl = this.getGoalPlayer();
    if (!goalPl) return null;
    else return goalPl === "Maximizer" ? 1 : -1;
  }
}

class BoardWithPlayers extends Board {
  private player1: Player;
  private player2: Player;
  private currentStarter: number;

  constructor() {
    super();
    this.player1 = new Player("Maximizer", "X", this);
    this.player2 = new Player("Minimizer", "O", this);
    this.currentStarter = 0;

    this.player1.hasTurn = true;

    this.player1.addEventListener("just_played", () => {
      this.player2.hasTurn = true;
    });
    this.player2.addEventListener("just_played", () => {
      this.player1.hasTurn = true;
    });
  }

  switchStarter() {
    if (this.currentStarter === 0) {
      this.player2.hasTurn = true;
      this.player1.hasTurn = false
      this.currentStarter = 1;
    } else {
      this.player1.hasTurn = true;
      this.player2.hasTurn = false
      this.currentStarter = 0;
    }
  }

  getPlayers(): Player[] {
    return [this.player1, this.player2];
  }

  resetBoard(switchSt?: boolean): PlayerType {
    this.state = getEmptyBoardState();
    if (switchSt) this.switchStarter();
    else {
      this.player1.hasTurn = true;
      this.player2.hasTurn = false;
      this.currentStarter = 0
    }
    return this.player1.hasTurn ? this.player1.type : this.player2.type
  }
}

export { Board, BoardWithPlayers, Player };

