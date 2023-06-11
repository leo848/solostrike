<template>
  <v-card>
    <v-card-title>{{state.correct}}</v-card-title>
  </v-card>
  <v-card>
    <v-card-title>#{{puzzle.index}}</v-card-title>
  </v-card>
</template>

<script lang="ts">
export default {
  props: {
    state: {
      required: true,
    },
    puzzle: {
      required: true,
    }
  }
}
</script>
