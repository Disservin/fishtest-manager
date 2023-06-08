<script setup lang="ts">
import { RouterView } from 'vue-router'

import VVerticalNavbar from '@/components/VVerticalNavbar.vue'
</script>

<template>
  <VVerticalNavbar />

  <div class="sm:pl-48 pl-40 h-screen">
    <RouterView />
  </div>
</template>

<style scoped></style>
