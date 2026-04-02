<script setup lang="ts">
import { useTheme } from '@/composables/useTheme'
import type { ThemeName } from '@/composables/useTheme'

const { theme, themes, setTheme } = useTheme()

const labels: Record<ThemeName, string> = {
  dark: '🌙 Dark',
  light: '☀️ Light',
  solarized: '🌊 Solarized',
  'high-contrast': '⚡ High Contrast',
}
</script>

<template>
  <div class="flex items-center gap-1">
    <button
      v-for="t in themes"
      :key="t"
      :title="labels[t]"
      class="rounded-md px-2 py-1 text-[10px] transition"
      :class="
        theme === t
          ? 'bg-(--accent)/20 text-(--accent) font-semibold'
          : 'text-(--text-muted) hover:text-(--text-primary)'
      "
      @click="setTheme(t)"
    >
      {{ labels[t].split(' ')[0] }}
    </button>
  </div>
</template>
