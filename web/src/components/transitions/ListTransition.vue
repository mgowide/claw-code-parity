<script setup lang="ts">
defineProps<{ tag?: string }>()
</script>

<template>
  <TransitionGroup :tag="tag ?? 'div'" name="message" class="contents">
    <slot />
  </TransitionGroup>
</template>
