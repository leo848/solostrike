<template>
  <div>
    <v-row>
      <v-col cols="12" sm="9" md="8" lg="6">
        <div ref="chessground" id="chessground-main"></div>
      </v-col>
    </v-row>
  </div>
</template>

<script lang="ts">
import App from "../App.vue"
import { Chessground } from 'chessground';

export default {
  components: { App },
  mounted() {
    const config = {};
    const ground = Chessground(this.$refs.chessground, config);
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
  aspect-ratio: 1 / 1;
  position: relative;
  overflow: hidden;
}

cg-board {
  background-color: #bfcfdd;
}
</style>
