<template>
  <div>
    <v-row>
      <v-col cols="12" sm="8" md="6" lg="5">
        <div ref="chessground" id="chessground-main"></div>
      </v-col>
      <v-col cols="12" md="6">
        <Timer v-if="timer" :timer="timer" class="mt-8"></Timer>
        <GameState v-if="state && fenInfo" :state="state" :puzzle="fenInfo" class="mt-4"></GameState>
      </v-col>
    </v-row>
  </div>
</template>

<script lang="ts">
import App from "../App.vue"
import GameState from "../components/GameState.vue";
import Timer from "../components/Timer.vue";

import { Chessground } from 'chessground';
import { Chess } from 'chess.js';
import type { Api as ChessgroundApi } from 'chessground/api'
import { Key, Piece as ChessgroundPiece } from "chessground/types";

import { randomFen, FenInfo } from '@/game/loadFens';
import { State, newState, type Timer as TimerType, newTimer } from '@/game/state';

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
  components: { App, GameState, Timer },
  data: () => ({
    game: new Chess() as Chess,
    ground: null as null | ChessgroundApi,
    fenInfo: null as null | FenInfo,
    state: newState(),
    timer: newTimer(),
  }),
  mounted() {
    const config = {
      autoCastle: true,
      movable: {
        free: false,
      },
      events: {
        move: (orig: Key, dest: Key, capturedPiece: ChessgroundPiece | undefined) => {
          this.timer.paused = false;

          const gameMoveInput = { from: orig, to: dest, promotion: 'q' };
          let move;
          try {
            move = this.game.move(gameMoveInput);
            this.state.temp.lastMove = move;
          } catch (e) {
            return;
          }
          if (this.game.isCheckmate()) {
            this.right();
            this.ground!.explode([dest]);
            setTimeout(() => {
              this.nextFen();
            }, 500);
          } else if (this.game.isGameOver()) {
            throw new Error("game over but no checkmate");
          } else {
            this.wrong();
            let moves = this.game.moves({ verbose: true });
            let capturedMoves = moves.filter(move => move.captured);
            let randomMove;
            if (capturedMoves.length) {
              randomMove = capturedMoves[Math.floor(Math.random() * capturedMoves.length)];
            } else {
              randomMove = moves[Math.floor(Math.random() * moves.length)];
            }
            this.game.move(randomMove);
            this.ground!.set({ fen: this.game.fen() })
            setTimeout(() => {
              if (!this.fenInfo) throw new Error("no previous fen");
              this.game.load(this.fenInfo.fen);
              this.ground!.set({ fen: this.fenInfo.fen });
              this.resetBoard();
            }, 1500);
          }
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

    const boardElt = this.$refs.chessground;
    if (!(boardElt instanceof HTMLElement)) {
      throw new Error("No board element");
    }
    this.ground = Chessground(boardElt, config);
    this.game = new Chess(this.fenInfo?.fen || undefined);

    this.nextFen();
  },
  methods: {
    right() {
      this.state.correct++;
      this.state.temp.outcome = "right";
      this.timer.deltas.push(+3)
      this.timer.end.setSeconds(this.timer.end.getSeconds() + 3);
    },
    wrong() {
      this.state.temp.outcome = "wrong";
      this.timer.deltas.push(-15);
      this.timer.end.setSeconds(this.timer.end.getSeconds() - 15);
    },
    async nextFen() {
      this.state.temp = {};

      let newFen = await randomFen();
      this.fenInfo = newFen;
      this.game.load(newFen.fen);

      this.resetBoard();
    },

    resetBoard() {
      const { game, ground } = this;
      if (!game || !ground) throw new Error("no game");
      const destinations = getDestinations(game as Chess);

      if (!this.fenInfo) throw new Error("no fen");
      ground.set({
        fen: this.fenInfo.fen,
        orientation: this.game.turn() == "w" ? "white" : "black",
        movable: {
          free: false,
          dests: destinations,
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
