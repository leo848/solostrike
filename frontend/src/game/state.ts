import {Square} from "chess.js";
import { reactive } from "vue";

export type State = {
  correct: number,
  temp?: {
    lastMove?: { from: Square, to: Square, promotion ?: string, san: string },
    outcome?: "right" | "wrong";
  }
}

export type Timer = Ref<{
  paused: boolean,
  start: Date,
  end: Date,
  update: number,
}>;

export function isTimer(obj: any): obj is Timer {
  return obj && typeof obj.paused === "boolean"
  && typeof obj.start === "object"
  && typeof obj.end === "object"
  && typeof obj.update === "number";
}

export function newTimer(): Timer {
  const now = new Date();
  const inOneMinute = new Date(now);
  inOneMinute.setSeconds(now.getSeconds() + 60);
  let timer = reactive({
    paused: true,
    start: now,
    end: inOneMinute,
    update: 0,
  });
  timer.update = setTimeout(() => updateTimer(timer), 1000);
  return timer;
}

function updateTimer(timer: Timer): void {
  timer.update = setTimeout(() => updateTimer(timer), 1000);
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
