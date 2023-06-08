<template>
    <highlightjs class="h-80 overflow-y-scroll" language="js" :code="diffText" />
</template>

<script setup lang="ts">
defineProps<{
    diffText: string
}>()
</script>

<style scoped></style>
