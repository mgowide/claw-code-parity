<script setup lang="ts">
import { CodeDiff } from 'v-code-diff'
import { ref } from 'vue'

const props = withDefaults(
  defineProps<{
    filePath: string
    oldContent: string
    newContent: string
    outputFormat?: 'side-by-side' | 'line-by-line'
  }>(),
  {
    outputFormat: 'line-by-line',
  },
)

const format = ref(props.outputFormat)

function toggleFormat() {
  format.value = format.value === 'side-by-side' ? 'line-by-line' : 'side-by-side'
}
</script>

<template>
  <div class="overflow-hidden rounded-lg border border-(--border)">
    <!-- Header -->
    <div class="flex items-center justify-between bg-(--bg-tertiary) px-3 py-1.5 text-xs text-(--text-secondary)">
      <span class="font-medium text-(--text-primary)">{{ filePath }}</span>
      <button
        class="rounded px-2 py-0.5 transition hover:bg-(--bg-primary) hover:text-(--text-primary)"
        @click="toggleFormat"
      >
        {{ format === 'side-by-side' ? 'Unified' : 'Side-by-side' }}
      </button>
    </div>

    <!-- Diff -->
    <div class="max-h-[500px] overflow-auto">
      <CodeDiff
        :old-string="oldContent"
        :new-string="newContent"
        :file-name="filePath"
        :output-format="format"
        theme="dark"
      />
    </div>
  </div>
</template>
