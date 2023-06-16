import {Square} from "chess.js";
import { reactive, Ref } from "vue";
import {FenInfo} from "./loadFens";

export type State = {
  correct: number,
  gameResults: GameResult[],
  temp: {
    lastMove?: { from: Square, to: Square, promotion ?: string, san: string },
    outcome?: "right" | "wrong";
  }
}

export type GameResult = {
  fen: FenInfo,
  timeDisplayed: Date,
  solved: boolean,
}

export type Timer = Ref<{
  paused: boolean,
  start: Date,
  end: Date,
  update: NodeJS.Timeout,
  deltas: number[],
}>;

export function isTimer(obj: any): obj is Timer {
  return obj && typeof obj.paused === "boolean"
  && typeof obj.start === "object"
  && typeof obj.end === "object"
  && typeof obj.update
  && Array.isArray(obj.deltas);
}

export function newTimer(): Timer {
  const now = new Date();
  const inOneMinute = new Date(now);
  inOneMinute.setSeconds(now.getSeconds() + 61.5);
  let timer = reactive({
    paused: true,
    start: now,
    end: inOneMinute,
    update: 0 as unknown as NodeJS.Timeout,
    deltas: [] as number[],
  }) as unknown as Timer;
  // @ts-ignore
  timer.update = setTimeout(() => updateTimer(timer), 1000);
  return timer;
}

function updateTimer(timer: Timer): void {
  // @ts-ignore
  if (timer.paused) timer.end.setSeconds(timer.end.getSeconds() + 1);
  // @ts-ignore
  timer.update = setTimeout(() => updateTimer(timer), 1000);
}

export function newState(): State {
  return {
    correct: 0,
    gameResults: [],
    temp: { },
  }
}

export function isState(obj: any): obj is State {
  return obj
  && typeof obj.correct == "number";
}
