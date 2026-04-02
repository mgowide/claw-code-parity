<script setup lang="ts">
import { useToastStore } from '@/stores/toastStore'
import type { Toast } from '@/stores/toastStore'

const store = useToastStore()

const icons: Record<string, string> = {
  success: '✓',
  error: '✕',
  warning: '⚠',
  info: 'ℹ',
}

const colors: Record<string, string> = {
  success: 'border-emerald-500/40 bg-emerald-500/10 text-emerald-300',
  error: 'border-red-500/40 bg-red-500/10 text-red-300',
  warning: 'border-amber-500/40 bg-amber-500/10 text-amber-300',
  info: 'border-blue-500/40 bg-blue-500/10 text-blue-300',
}
</script>

<template>
  <Teleport to="body">
    <div class="pointer-events-none fixed bottom-6 right-4 z-50 flex flex-col gap-2">
      <TransitionGroup name="slide-up">
        <div
          v-for="toast in store.toasts"
          :key="toast.id"
          class="pointer-events-auto flex items-start gap-2.5 rounded-lg border px-3.5 py-2.5 text-sm shadow-lg backdrop-blur-sm"
          :class="colors[toast.type]"
          role="alert"
          @click="store.dismiss(toast.id)"
        >
          <span class="mt-0.5 font-bold">{{ icons[toast.type] }}</span>
          <span class="leading-snug">{{ toast.message }}</span>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>
