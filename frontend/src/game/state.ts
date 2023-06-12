export type State = {
  correct: number,
}

export function newState(): State {
  return {
    correct: 0,
  }
}

export function isState(obj: any): obj is State {
  return obj
  && typeof obj.correct == "number";
}
