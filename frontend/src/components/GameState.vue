<template>
  <v-row>
    <v-col cols="5" v-if="state">
      <v-card>
          <v-scroll-y-transition hide-on-leave>
            <v-card-title :key="state.correct" class="large-number py-12 px-4">{{state.correct}}</v-card-title>
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
  </v-row>
</template>

<script lang="ts">
import {FenInfo, isFenInfo} from '@/game/loadFens';
import {State, isState} from '@/game/state';

export default {
  data: () => ({
    color: undefined as undefined | string,
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
  methods: {
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
  font-size: 70pt;
}
.semilarge {
  font-size: 30pt;
}
</style>
