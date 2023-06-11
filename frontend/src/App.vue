<template>
  <v-app>
    <v-app-bar color="primary">
      <v-app-bar-title>{{ $route.meta.title }}</v-app-bar-title>
    </v-app-bar>
    <router-view />
  </v-app>
</template>

<script setup lang="ts">
</script>
