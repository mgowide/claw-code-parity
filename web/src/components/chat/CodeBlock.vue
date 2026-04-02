<script setup lang="ts">
import { ref, computed } from 'vue'
import hljs from 'highlight.js/lib/core'

import javascript from 'highlight.js/lib/languages/javascript'
import typescript from 'highlight.js/lib/languages/typescript'
import python from 'highlight.js/lib/languages/python'
import rust from 'highlight.js/lib/languages/rust'
import json from 'highlight.js/lib/languages/json'
import bash from 'highlight.js/lib/languages/bash'
import css from 'highlight.js/lib/languages/css'
import xml from 'highlight.js/lib/languages/xml'
import markdown from 'highlight.js/lib/languages/markdown'
import yaml from 'highlight.js/lib/languages/yaml'
import sql from 'highlight.js/lib/languages/sql'
import go from 'highlight.js/lib/languages/go'
import java from 'highlight.js/lib/languages/java'
import cpp from 'highlight.js/lib/languages/cpp'
import c from 'highlight.js/lib/languages/c'
import ruby from 'highlight.js/lib/languages/ruby'
import php from 'highlight.js/lib/languages/php'
import swift from 'highlight.js/lib/languages/swift'
import kotlin from 'highlight.js/lib/languages/kotlin'
import dockerfile from 'highlight.js/lib/languages/dockerfile'

hljs.registerLanguage('javascript', javascript)
hljs.registerLanguage('typescript', typescript)
hljs.registerLanguage('python', python)
hljs.registerLanguage('rust', rust)
hljs.registerLanguage('json', json)
hljs.registerLanguage('bash', bash)
hljs.registerLanguage('css', css)
hljs.registerLanguage('xml', xml)
hljs.registerLanguage('html', xml)
hljs.registerLanguage('markdown', markdown)
hljs.registerLanguage('yaml', yaml)
hljs.registerLanguage('sql', sql)
hljs.registerLanguage('go', go)
hljs.registerLanguage('java', java)
hljs.registerLanguage('cpp', cpp)
hljs.registerLanguage('c', c)
hljs.registerLanguage('ruby', ruby)
hljs.registerLanguage('php', php)
hljs.registerLanguage('swift', swift)
hljs.registerLanguage('kotlin', kotlin)
hljs.registerLanguage('dockerfile', dockerfile)

const props = withDefaults(
  defineProps<{
    code: string
    language?: string
    showLineNumbers?: boolean
    maxHeight?: number
    fileName?: string
  }>(),
  {
    language: undefined,
    showLineNumbers: true,
    maxHeight: 400,
    fileName: undefined,
  },
)

const copied = ref(false)
const expanded = ref(false)

const highlighted = computed(() => {
  if (props.language && hljs.getLanguage(props.language)) {
    return hljs.highlight(props.code, { language: props.language })
  }
  return hljs.highlightAuto(props.code)
})

const detectedLang = computed(() => props.language ?? highlighted.value.language ?? 'text')

const highlightedLines = computed(() => highlighted.value.value.split('\n'))

const lines = computed(() => props.code.split('\n'))
const needsExpand = computed(() => !expanded.value && lines.value.length > 20)

async function copyCode() {
  await navigator.clipboard.writeText(props.code)
  copied.value = true
  setTimeout(() => { copied.value = false }, 2000)
}
</script>

<template>
  <div class="overflow-hidden rounded-lg border border-(--border) text-xs">
    <!-- Header -->
    <div class="flex items-center justify-between bg-(--bg-tertiary) px-3 py-1.5 text-(--text-secondary)">
      <span class="flex items-center gap-2">
        <span v-if="fileName" class="font-medium text-(--text-primary)">{{ fileName }}</span>
        <span class="rounded bg-(--bg-primary) px-1.5 py-0.5 text-[10px] uppercase">{{ detectedLang }}</span>
      </span>
      <button
        class="rounded px-2 py-0.5 transition hover:bg-(--bg-primary) hover:text-(--text-primary)"
        @click="copyCode"
      >
        {{ copied ? '✓ Copied' : 'Copy' }}
      </button>
    </div>

    <!-- Code body -->
    <div
      class="overflow-auto bg-(--bg-primary)"
      :style="{ maxHeight: expanded ? 'none' : `${maxHeight}px` }"
    >
      <table class="w-full border-collapse">
        <tbody>
          <tr v-for="(_, i) in lines" :key="i">
            <td
              v-if="showLineNumbers && lines.length > 5"
              class="select-none border-r border-(--border) px-2 text-right text-(--text-muted)"
              style="min-inline-size: 2.5rem"
            >
              {{ i + 1 }}
            </td>
            <!-- eslint-disable-next-line vue/no-v-html -->
            <td class="whitespace-pre-wrap px-3 py-0" v-html="highlightedLines[i] ?? ''" />
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Expand button -->
    <button
      v-if="needsExpand"
      class="block w-full border-t border-(--border) bg-(--bg-tertiary) py-1 text-center text-xs text-(--text-secondary) transition hover:text-(--text-primary)"
      @click="expanded = true"
    >
      Show all {{ lines.length }} lines
    </button>
  </div>
</template>
