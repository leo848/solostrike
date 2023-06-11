export type State = {
  correct: number,
}

export function newState(): State {
  return {
    correct: 0,
  }
}
