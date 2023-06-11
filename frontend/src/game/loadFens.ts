let fens: null | string[] = null;

export async function randomFen() {
  if (fens == null) {
    fens = await loadFens();
  }
  assert(fens.length == 100000, `Length is not 100000, but ${fens.length}`);
  return fens[Math.floor(Math.random() * fens.length)];
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
