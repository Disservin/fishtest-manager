<template>
  <CenterLayout>
    <div>
      <h1 class="text-xl font-bold">Stockfish Repository</h1>
    </div>
    <div class="mt-5">
      <div>
        <span> Selected: {{ selected }} </span>
      </div>
      <button
        type="button"
        class="file-input w-full max-w-xs bg-stone-800 outline-none mt-1"
        @click="selectPath"
      >
        Change
      </button>
    </div>
  </CenterLayout>
</template>

<script setup lang="ts">
import CenterLayout from '@/layouts/CenterLayout.vue'
import { open } from '@tauri-apps/api/dialog'
import { isArray } from '@vue/shared'
import { ref } from 'vue'

const selected = ref<string>('None')

const selectPath = async () => {
  const path = await open({
    directory: true
  })

  if (path != null && !isArray(path)) {
    selected.value = path
  }
}
</script>

<style scoped></style>
