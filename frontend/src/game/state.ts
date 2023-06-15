import {Square} from "chess.js";
import { reactive, Ref } from "vue";

export type State = {
  correct: number,
  temp: {
    lastMove?: { from: Square, to: Square, promotion ?: string, san: string },
    outcome?: "right" | "wrong";
  }
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
  timer.value.update = setTimeout(() => updateTimer(timer), 1000);
  return timer;
}

function updateTimer(timer: Timer): void {
  if (timer.value.paused) timer.value.end.setSeconds(timer.value.end.getSeconds() + 1);
  timer.value.update = setTimeout(() => updateTimer(timer), 1000);
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
