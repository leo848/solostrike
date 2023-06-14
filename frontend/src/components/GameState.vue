<template>
  <v-row>
    <v-col cols="5">
      <v-card>
        <v-card-title class="large-number py-12 px-4">{{state.correct}}</v-card-title>
      </v-card>
    </v-col>
    <v-col cols="7">
      <v-card :color="color" ref="puzzleCard">
        <v-card-title>#{{puzzle.index}}<br/>{{puzzle.color === "white" ? "Weiß" : "Schwarz"}} am Zug</v-card-title>
        <v-card-text class="semilarge py-8">{{puzzleMove}}</v-card-text>
      </v-card>
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
      }
    },
    puzzle: {
      required: true,
      validator: (obj: any): obj is FenInfo => {
        return isFenInfo(obj);
      }
    }
  },
  methods: {
    input(result: "right" | "wrong") {
      const ev = new Event("mousedown") as MouseEventInit;
      const el = this.$refs.puzzleCard.$el;
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
