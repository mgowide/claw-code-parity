<script setup lang="ts">
import ChatThread from '@/components/chat/ChatThread.vue'
import InputComposer from '@/components/composer/InputComposer.vue'
</script>

<template>
  <div class="flex h-full flex-col">
    <ChatThread />
    <InputComposer />
  </div>
</template>
