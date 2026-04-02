<script setup lang="ts">
export interface AttachedFile {
  name: string
  content: string
  size: number
}

interface Props {
  files: AttachedFile[]
}

defineProps<Props>()
const emit = defineEmits<{ remove: [index: number] }>()

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes}B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)}KB`
  return `${(bytes / 1024 / 1024).toFixed(1)}MB`
}
</script>

<template>
  <div
    v-if="files.length > 0"
    class="flex flex-wrap gap-1.5 border-b border-(--border) px-4 py-2"
  >
    <div
      v-for="(file, i) in files"
      :key="file.name"
      class="flex items-center gap-1.5 rounded-full border border-(--border) bg-(--bg-primary) px-2.5 py-1 text-xs text-(--text-secondary)"
    >
      <!-- Paperclip icon -->
      <svg width="11" height="11" viewBox="0 0 12 12" fill="none" class="shrink-0 text-(--text-muted)">
        <path
          d="M10.5 5.5L5.5 10.5C4.4 11.6 2.6 11.6 1.5 10.5C0.4 9.4 0.4 7.6 1.5 6.5L7 1C7.8 0.2 9 0.2 9.8 1C10.6 1.8 10.6 3 9.8 3.8L4.5 9.1C4.1 9.5 3.5 9.5 3.1 9.1C2.7 8.7 2.7 8.1 3.1 7.7L7.5 3.3"
          stroke="currentColor"
          stroke-width="1.2"
          stroke-linecap="round"
        />
      </svg>
      <span class="max-w-32 truncate">{{ file.name }}</span>
      <span class="text-(--text-muted)">{{ formatSize(file.size) }}</span>
      <button
        class="ml-0.5 rounded-full p-0.5 hover:bg-(--border) hover:text-(--text-primary)"
        :aria-label="`Remove ${file.name}`"
        @click="emit('remove', i)"
      >
        <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
          <path d="M2 2L8 8M8 2L2 8" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" />
        </svg>
      </button>
    </div>
  </div>
</template>
