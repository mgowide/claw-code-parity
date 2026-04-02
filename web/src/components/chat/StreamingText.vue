<script setup lang="ts">
defineProps<{
  text: string
}>()
</script>

<template>
  <span class="whitespace-pre-wrap">{{ text }}<span class="cursor" /></span>
</template>

<style scoped>
.cursor {
  display: inline-block;
  inline-size: 2px;
  block-size: 1.1em;
  margin-inline-start: 1px;
  vertical-align: text-bottom;
  background: var(--accent);
  animation: blink 1s step-end infinite;
}

@keyframes blink {
  0%, 50% { opacity: 1; }
  51%, 100% { opacity: 0; }
}
</style>
