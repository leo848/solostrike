import {Square} from "chess.js";

export type State = {
  correct: number,
  temp?: {
    lastMove?: { from: Square, to: Square, promotion ?: string, san: string },
  }
}

export function newState(): State {
  return {
    correct: 0,
    temp: { },
  }
}

export function isState(obj: any): obj is State {
  return obj
  && typeof obj.correct == "number";
}
