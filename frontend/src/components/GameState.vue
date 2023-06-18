<template>
  <v-row>
    <v-col cols="5" v-if="state">
      <v-card height="100%">
          <v-scroll-y-transition hide-on-leave>
            <v-card-title :key="state.correct" class="large-number foldit text-center py-16 px-4">{{state.correct}}</v-card-title>
          </v-scroll-y-transition>
        </v-card>
    </v-col>
    <v-col cols="7" v-if="puzzle">
      <v-scale-transition hide-on-leave>
        <v-card :key="puzzle.index" :color="color" :loading="!color" ref="puzzleCard">
          <template v-slot:loader="{ isActive }">
            <v-progress-linear
              :active="isActive"
              color="#333"
              height="4"
              v-model="puzzle.index"
              max="100000"
             ></v-progress-linear>
          </template>
          <v-card-title>#{{leftPad("" + puzzle.index, "0", 5)}}<br/>{{puzzle.color === "white" ? "Weiß" : "Schwarz"}} am Zug</v-card-title>
          <v-card-text class="semilarge py-8">{{puzzleMove}}</v-card-text>
        </v-card>
      </v-scale-transition>
    </v-col>
    <v-col cols="4" v-if="state && state.gameResults[0]">
      <v-card>
        <v-card-title class="mid-number streak">
          <span>
            <v-icon size="60pt" class="power-icon" color="yellow-lighten-4">mdi-lightning-bolt</v-icon>
          </span>
          <span class="streak">
            {{streakNumber}}
          </span>
        </v-card-title>
      </v-card>
    </v-col>
    <v-col cols="4">
      <v-card height="100%">
        <v-slide-x-reverse-transition hide-on-leave>
          <v-card-title :key="Number(skipMillisLeft != 0)">
            <v-progress-linear v-if="skipMillisLeft != 0" class="rounded-xl" v-model="skipMillisLeft" max="5000" height="100%" @click="skip(false)"></v-progress-linear>
            <v-btn v-else block flat size="x-large" class="skip-btn rounded-xl py-12 my-0" @click="skip(true)">skip</v-btn>
          </v-card-title>
        </v-slide-x-reverse-transition>
      </v-card>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import {FenInfo, isFenInfo} from '@/game/loadFens';
import {State, isState, streak} from '@/game/state';

export default {
  data: () => ({
    color: undefined as undefined | string,
    skipInterval: null as null | NodeJS.Timeout,
    skipMillisLeft: 0,
  }),
  props: {
    state: {
      required: true,
      validator: (obj: any): obj is State => {
        return isState(obj);
      },
      type: Object as () => State,
    },
    puzzle: {
      required: true,
      validator: (obj: any): obj is FenInfo => {
        return isFenInfo(obj);
      },
      type: Object as () => FenInfo,
    }
  },
  emits: [ 'skip' ],
  methods: {
    skip(start: boolean) {
      if (!start) {
        clearInterval(this.skipMillisLeft);
        this.skipMillisLeft = 0;
        return;
      }
      const timeout = 5000;
      const frame = 100;
      this.skipMillisLeft = timeout;
      this.skipInterval = setInterval(() => {
        if (this.skipMillisLeft! > 0) {
          this.skipMillisLeft! -= frame;
        } else {
          clearInterval(this.skipInterval!);
          this.skipMillisLeft = 0;
          this.$emit('skip');
        }
      }, frame)
    },
    input(result: "right" | "wrong") {
      const ev = new Event("mousedown") as MouseEventInit;
      const puzzleElt = this.$refs.puzzleCard as any;
      if (!puzzleElt || !(puzzleElt.$el)) throw new Error("no puzzle element");
      const el = puzzleElt.$el;
      const offset = el.getBoundingClientRect();
      ev.clientX = offset.left + offset.width / 2;
      ev.clientY = offset.top + offset.height / 2;
      el.dispatchEvent(ev);

      this.color = result === "right" ? "green" : "red";

      setTimeout(() => {
        el.dispatchEvent(new Event("mouseup"));
        this.color = undefined;
        this.state.temp.outcome = undefined;
        this.state.temp.lastMove = undefined;
      }, 750)
    },
    leftPad(str: string, prefix: string, length: number): string {
      while (str.length < length) {
        str = prefix + str;
      }
      return str;
    }
  },
  watch: {
    "state.temp.outcome": function(value, old) {
      if (value === "right" || value === "wrong") {
        this.input(value);
      }
    }
  },
  computed: {
    streakNumber(): number {
      return streak(this.state);
    },
    puzzleMove(): string {
      const suffix = this.puzzle.color === "white" ? ". " : "... ";
      const lastMove = this.state.temp?.lastMove?.san ?? '';
      return `${this.puzzle.moveNumber}${suffix}${lastMove}`
    }
  }
}
</script>

<style scoped>
.large-number {
  font-size: 90pt;
}
.mid-number {
  font-size: 60pt;
}
.semilarge {
  font-size: 30pt;
}
.skip-btn {
  font-size: 40pt;
}
.streak {
  line-height: 70pt;
  font-weight: 400;
}
</style>
