<template>
  <v-card>
    <v-card-title class="timer" v-if="secondsLeft >= 0">
      <v-row>
        <v-col cols="8">
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
          <span class="tenths rounded-lg px-5 my-0" v-if="secondsLeft <= 10">
            {{tenths}}
          </span>
        </v-col>
        <v-col cols="4">
          <span>
            <v-card class="delta" v-for="delta, id in deltas" :key="delta.amount+id" :color="color(delta)" max-width="75px">
              <v-card-title class="pl-0">
                <v-icon v-if="delta.amount > 0">mdi-plus</v-icon>
                <v-icon v-else-if="delta.amount < 0">mdi-minus</v-icon>
                <span class="pl-0 text-h5">{{Math.abs(delta.amount)}}</span>
              </v-card-title>
            </v-card>
          </span>
          </v-col>
      </v-row>
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
    deltas: [] as {amount: number, shown: number}[],
  }),
  props: {
    timer: {
      type: Object,
      required: true,
      validator: isTimer,
    }
  },
  emits: [ 'timeUp' ],
  mounted() {
    this.updateSeconds = setInterval(() => {
      if (this.timer.paused) return;

      this.secondsLeft = (this.timer.end.getTime() - new Date().getTime()) / 1000;
      if (this.secondsLeft < 0) {
        if (this.updateSeconds) clearInterval(this.updateSeconds);
        this.$emit("timeUp");
      }

      if (this.timer.deltas.length > 0) {
        this.deltas.push({
          amount: this.timer.deltas.pop(),
          shown: 20,
        });
      }
      this.deltas = this.deltas.filter(delta => delta.shown >= 0);
      for (const delta of this.deltas) {
        delta.shown--;
      }

      const colonElt = this.$refs.colon;
      if (!colonElt) return;
      if (!(colonElt instanceof HTMLElement)) return;
      if (this.secondsLeft % 1 > 0.5) {
        colonElt.style.color = "#ccc";
      } else {
        colonElt.style.color = "#999";
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
    },
    tenths() {
      let raw = Math.floor((this.secondsLeft % 1) * 10).toString();
      return raw;
    }
  },
  methods: {
    color(delta: { amount: number, shown: number }): string | undefined {
      const base = delta.amount > 0 ? [0, 200, 0] : delta.amount < 0 ? [ 200, 0, 0 ] : [ 50, 50, 50 ];
      return `rgba(${base[0]}, ${base[1]}, ${base[2]}, ${Math.sin(delta.shown/20*Math.PI)}`
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
.tenths {
  font-size: 75px;
  background-color: #666;
  color: #333;
  vertical-align: top;
}
</style>
