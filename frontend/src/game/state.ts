import {Square} from "chess.js";

export type State = {
  correct: number,
  temp?: {
    lastMove?: { from: Square, to: Square, promotion ?: string, san: string },
    outcome?: "right" | "wrong";
  }
}

export type Timer = {
  paused: boolean,
  started: Date,
  secondsLeft: number,
  update: number,
}

export function isTimer(obj: any): obj is Timer {
  return obj && typeof obj.secondsLeft === "number";
}

export function newTimer(): Timer {
  let timer = {
    paused: true,
    started: new Date(),
    secondsLeft: 60,
    update: 0,
  };
  timer.update = setTimeout(() => updateTimer(timer), 1000);
  return timer;
}

function updateTimer(timer: Timer): void {
  if (!timer.paused) {
    timer.secondsLeft--;
  }
  setTimeout(() => updateTimer(timer), 1000);
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
