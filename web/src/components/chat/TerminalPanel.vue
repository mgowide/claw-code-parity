<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from 'vue'
import { Terminal } from '@xterm/xterm'
import { FitAddon } from '@xterm/addon-fit'
import '@xterm/xterm/css/xterm.css'

const props = withDefaults(
  defineProps<{
    output: string
    maxHeight?: number
  }>(),
  {
    maxHeight: 300,
  },
)

const containerRef = ref<HTMLElement | null>(null)
let term: Terminal | null = null
let fitAddon: FitAddon | null = null
let resizeObserver: ResizeObserver | null = null

onMounted(() => {
  if (!containerRef.value) return

  term = new Terminal({
    theme: {
      background: '#0d1117',
      foreground: '#e6edf3',
      cursor: '#e6edf3',
      selectionBackground: '#264f78',
    },
    fontSize: 13,
    fontFamily: "'JetBrains Mono', 'Fira Code', Consolas, monospace",
    cursorBlink: false,
    disableStdin: true,
    scrollback: 1000,
    convertEol: true,
  })

  fitAddon = new FitAddon()
  term.loadAddon(fitAddon)
  term.open(containerRef.value)
  fitAddon.fit()

  if (props.output) {
    term.write(props.output)
  }

  resizeObserver = new ResizeObserver(() => {
    fitAddon?.fit()
  })
  resizeObserver.observe(containerRef.value)
})

watch(
  () => props.output,
  (newOutput) => {
    if (term) {
      term.clear()
      term.write(newOutput)
    }
  },
)

onBeforeUnmount(() => {
  resizeObserver?.disconnect()
  term?.dispose()
})
</script>

<template>
  <div
    ref="containerRef"
    class="overflow-hidden rounded-lg border border-(--border)"
    :style="{ maxHeight: `${maxHeight}px` }"
  />
</template>
