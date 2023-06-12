<template>
  <v-card class="mt-8">
    <v-card-title class="large-number py-12 px-4">{{state.correct}}</v-card-title>
  </v-card>
  <v-card class="mt-8">
    <v-card-title>#{{puzzle.index}}</v-card-title>
    <v-card-text class="semilarge py-8">{{puzzleMove}}</v-card-text>
  </v-card>
</template>

<script lang="ts">
import {FenInfo, isFenInfo} from '@/game/loadFens';
import {State, isState} from '@/game/state';

export default {
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
  font-size: 75pt;
}
.semilarge {
  font-size: 40pt;
}
</style>
