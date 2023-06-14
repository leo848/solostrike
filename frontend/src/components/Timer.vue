<template>
  <v-card>
    <v-card-title class="timer" v-if="secondsLeft >= 0">
      <v-scroll-y-transition hide-on-leave>
        <span :key="minutes">
          {{minutes}}
        </span>
      </v-scroll-y-transition>
      <span class="mx-n12" ref="colon">
        :
      </span>
      <span :key="secondsLeft">
        {{seconds}}
      </span>
    </v-card-title>
    <v-card-title class="time-up" v-else>
      Zeit
      <br/>
      abgelaufen!
    </v-card-title>
  </v-card>
</template>

<script lang="ts">
import { type Timer, isTimer } from '@/game/state';

export default {
  data: () => ({
    secondsLeft: 60,
    updateSeconds: null as null | NodeJS.Timer,
  }),
  props: {
    timer: {
      type: Object,
      required: true,
      validator: isTimer,
    }
  },
  mounted() {
    this.updateSeconds = setInterval(() => {
      if (this.timer.paused) return;
      this.secondsLeft = (this.timer.end.getTime() - new Date().getTime()) / 1000;
      if (!this.$refs.colon) return;
      if (this.secondsLeft % 1 > 0.5) {
        this.$refs.colon.style.color = "#ccc";
      } else {
        this.$refs.colon.style.color = "#999";
      }
    }, 100);
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
  line-height: 175px;
}
.time-up {
  font-size: 80px;
  line-height: 80px;
}
</style>
