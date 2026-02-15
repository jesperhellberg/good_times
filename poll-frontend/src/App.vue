<template>
  <LocaleToggle />
  <router-view v-slot="{ Component }">
    <transition name="fade" mode="out-in">
      <component :is="Component" />
    </transition>
  </router-view>
</template>

<script setup>
import LocaleToggle from './components/LocaleToggle.vue'
</script>
