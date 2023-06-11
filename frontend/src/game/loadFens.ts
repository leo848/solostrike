let fens: null | string[] = null;

export type FenInfo = {
  fen: string,
  index: number,
}

export async function randomFen(): Promise<FenInfo> {
  if (fens == null) {
    fens = await loadFens();
  }
  assert(fens.length == 100000, `Length is not 100000, but ${fens.length}`);
  const index = Math.floor(Math.random() * fens.length);
  return {
    fen: fens[index],
    index,
  };
}

async function loadFens(): Promise<string[]> {
  let resp = await fetch("fens/fens_l.json");
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
