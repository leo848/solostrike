<template>
  <v-app>
    <v-app-bar color="primary">
      <v-app-bar-title>{{ $route.meta.title }}</v-app-bar-title>
    </v-app-bar>
    <v-main>
      <router-view />
    </v-main>
  </v-app>
</template>

<script lang="ts">
  export default {

  }
</script>

<style>
@font-face {
  font-family: "Foldit";
  src: url("fonts/Foldit-VariableFont_wght.ttf");
}

.foldit {
  font-family: Foldit;
}
</style>
