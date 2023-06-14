<template>
  <v-card>
    <v-card-title class="timer">
      <span :key="secondsLeft">
        {{minutes}}
      </span>:<span>
        {{seconds}}
      </span>
    </v-card-title>
  </v-card>
</template>

<script lang="ts">
import { type Timer, isTimer } from '@/game/state';

export default {
  data: () => ({
    secondsLeft: null as null | number,
  }),
  props: {
    timer: {
      required: true,
      validator: isTimer,
    }
  },
  created() {
    this.updateSeconds = setInterval(() => {
      this.secondsLeft = Math.floor((this.timer.end.getTime() - new Date().getTime()) / 1000);
    }, 500);
  },
  computed: {
    minutes() {
      let raw = Math.floor(this.secondsLeft / 60).toString();
      return raw;
    },
    seconds() {
      let raw = Math.floor(this.secondsLeft % 60).toString();
      while (raw.length < 2) {
        raw = "0" + raw;
      }
      return raw;
    }
  }
}
</script>

<style scoped>
.timer {
  font-size: 175px;
  font-weight: 300;
  padding-top: 96px !important;
  padding-bottom: 96px !important;
}
</style>
