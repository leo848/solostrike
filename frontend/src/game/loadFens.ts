import {Color} from "chessground/types";

let fens: null | string[] = null;

export type FenInfo = {
  fen: string,
  index: number,
  moveNumber: number,
  color: Color,
}

export function isFenInfo(obj: any): obj is FenInfo {
  return obj
  && obj.fen && typeof obj.fen == "string"
  && typeof obj.index == "number"
  && typeof obj.moveNumber == "number"
  && obj.color == "white" || obj.color == "black";
}

export async function randomFen(): Promise<FenInfo> {
  if (fens == null) {
    fens = await loadFens();
  }
  assert(fens.length == 100000 || fens.length == 1000, `Length is not 100000 or 1000, but ${fens.length}`);
  const index = Math.floor(Math.random() * fens.length);
  const fen = fens[index];
  const components = fen.split(" ");
  const moveNumber = Number(components[5]);
  const colorToMove = components[1] == 'w' ? 'white' : 'black';
  return {
    fen,
    index,
    moveNumber,
    color: colorToMove,
  };
}

async function loadFens(): Promise<string[]> {
  let resp = await fetch("fens/fens_created_perfect_count.json");
  let json = await resp.json();
  return json;
}

function assert(condition: boolean, message?: string): (typeof condition extends true ? never : undefined) {
  if (!condition) {
    throw new Error(message);
  } else {
    return undefined;
  }
}
