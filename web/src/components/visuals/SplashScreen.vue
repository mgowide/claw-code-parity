<script setup lang="ts">
import { ref, onMounted } from 'vue'
import AmbientParticles from './AmbientParticles.vue'

const emit = defineEmits<{ done: [] }>()
const visible = ref(true)
const fading = ref(false)

onMounted(() => {
  // Auto-dismiss after 2.8 s
  setTimeout(() => {
    fading.value = true
    setTimeout(() => {
      visible.value = false
      emit('done')
    }, 500)
  }, 2800)
})

function dismiss() {
  fading.value = true
  setTimeout(() => {
    visible.value = false
    emit('done')
  }, 400)
}
</script>

<template>
  <Transition name="fade">
    <div
      v-if="visible"
      class="fixed inset-0 z-50 flex items-center justify-center bg-(--bg-primary)"
      :class="fading ? 'opacity-0 transition-opacity duration-500' : ''"
      @click="dismiss"
    >
      <!-- Three.js particle background -->
      <div class="absolute inset-0 overflow-hidden">
        <AmbientParticles />
      </div>

      <!-- Content -->
      <div class="relative z-10 flex flex-col items-center gap-4 select-none">
        <!-- Logo mark -->
        <div
          class="flex h-20 w-20 items-center justify-center rounded-2xl bg-(--accent)/10 ring-1 ring-(--accent)/30"
        >
          <span class="text-4xl">🦀</span>
        </div>
        <div class="text-center">
          <h1 class="text-3xl font-bold tracking-tight text-(--text-primary)">Claw</h1>
          <p class="mt-1 text-sm text-(--text-muted)">AI Agent Interface</p>
        </div>
        <!-- Loading indicator -->
        <div class="mt-2 flex gap-1.5">
          <span
            v-for="i in 3"
            :key="i"
            class="h-1.5 w-1.5 rounded-full bg-(--accent)"
            :style="{ animationDelay: `${(i - 1) * 0.18}s` }"
            style="animation: pulse 1s infinite"
          />
        </div>
        <p class="text-[10px] text-(--text-muted)">Click to continue</p>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
@keyframes pulse {
  0%, 100% { opacity: 0.2; transform: scale(0.8); }
  50% { opacity: 1; transform: scale(1); }
}
</style>
