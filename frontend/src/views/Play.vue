<template>
  <div>
    <v-row>
      <v-col cols="12" sm="9" md="8" lg="6">
        <div ref="chessground" id="chessground-main"></div>
        <v-btn size="x-large" @click="nextFen">next fen</v-btn>
      </v-col>
    </v-row>
  </div>
</template>

<script lang="ts">
import App from "../App.vue"
import { Chessground } from 'chessground';
import { Chess } from 'chess.js';
import type { Api as ChessgroundApi } from 'chessground/api'
import { Key, Piece as ChessgroundPiece } from "chessground/types";

import { randomFen } from '@/game/loadFens';

function getDestinations(game: Chess): Map<Key, Key[]> {
  const destinations: Map<Key, Key[]> = new Map();
  for (const move of game.moves({ verbose: true })) {
    if (typeof move === "string") throw new Error("move is string");
    if (destinations.get(move.from as Key) === undefined) {
      destinations.set(move.from as Key, []);
    }
    destinations.get(move.from as Key)!.push(move.to as Key);
  }
  return destinations;
}

export default {
  components: { App },
  data: () => ({
    game: null as null | Chess,
    ground: null as null | ChessgroundApi,
    currentFen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
  }),
  mounted() {
    const config = {
      autoCastle: true,
      movable: {
        free: false,
      },
      events: {
        move: (orig: Key, dest: Key, capturedPiece: ChessgroundPiece | undefined) => {
          console.error("todo");
        }
      },
      highlight: {
        lastMove: true,
        check: true,
      },
      animation: {
        enabled: true,
        duration: 500,
      }
    };

    this.ground = Chessground(this.$refs.chessground, config);
    this.game = new Chess(this.currentFen);

    this.nextFen();
  },
  methods: {
    async nextFen() {
      let newFen = await randomFen();
      this.currentFen = newFen.fen;
      this.game.load(this.currentFen);

      const destinations = getDestinations(this.game);

      this.ground.set({
        fen: this.currentFen,
        orientation: this.game.turn() == "w" ? "white" : "black",
        movable: {
          destinations,
        }
      });
    }
  }
}
</script>

<style>
.piece-letter {
  height: 1.5em;
  width: 1.5em;
  margin-top: 0.25em;
  transform: scale(1.5);
  margin-left: 0.5em;
  margin-right: 0.5em;
}

.moves-move,
.moves-enter-active,
.moves-leave-active {
  transition: all 0.25s ease;
}
.moves-enter-from,
.moves-leave-to {
  opacity: 0;
  transform: translateX(30px);
}

.moves-leave-active {
  position: absolute;
  z-index: -1;
}

#chessground-main {
  width: 100%;
  max-height: 80vh;
  max-width: 80vh;
  aspect-ratio: 1 / 1 !important;
  position: relative;
  overflow: hidden;
}

cg-board {
  background-color: #bfcfdd;
}
</style>
