<script setup lang="ts">
import { ref, computed, onMounted, inject } from 'vue'
import { useRouter } from 'vue-router'
import { useSession } from '@/composables/useSession'
import { useSessionStore } from '@/stores/sessionStore'
import type { useWebSocket } from '@/composables/useWebSocket'

const router = useRouter()
const store = useSessionStore()
const { sessions, sessionsLoading, loadSessions } = useSession()
const ws = inject<ReturnType<typeof useWebSocket>>('ws')!

const query = ref('')

const filtered = computed(() => {
  const q = query.value.trim().toLowerCase()
  // Show the 10 most recent sessions, filtered by search
  return sessions.value
    .filter((s) => !q || s.name.toLowerCase().includes(q) || s.model.toLowerCase().includes(q))
    .slice(0, 10)
})

function isActive(id: string) {
  return store.sessionId === id
}

function handleResume(id: string) {
  ws.send({ type: 'resume_session', session_id: id })
  router.push('/')
}

function handleNew() {
  store.reset()
  router.push('/')
}

onMounted(() => {
  loadSessions()
})
</script>

<template>
  <div class="flex flex-col h-full">
    <!-- Header -->
    <div class="flex items-center gap-2 border-b border-(--border) px-4 py-3">
      <div class="h-6 w-6 rounded-md bg-(--accent)" />
      <span class="text-sm font-semibold text-(--text-primary)">Claw</span>
    </div>

    <!-- New Chat -->
    <div class="p-3">
      <button
        class="w-full rounded-md border border-(--border) px-3 py-1.5 text-xs text-(--text-secondary) transition hover:border-(--accent) hover:text-(--text-primary)"
        @click="handleNew"
      >
        + New Chat
      </button>
    </div>

    <!-- Search -->
    <div class="px-3 pb-2">
      <input
        v-model="query"
        placeholder="Search sessions…"
        class="w-full rounded-md border border-(--border) bg-(--bg-primary) px-2 py-1 text-xs text-(--text-primary) outline-none placeholder:text-(--text-muted) focus:border-(--accent)"
      />
    </div>

    <!-- Session list -->
    <nav class="flex-1 overflow-y-auto px-2 pb-2">
      <div v-if="sessionsLoading" class="py-4 text-center text-xs text-(--text-muted)">
        Loading…
      </div>
      <div v-else-if="filtered.length === 0" class="py-4 text-center text-xs text-(--text-muted)">
        No sessions yet
      </div>
      <button
        v-for="s in filtered"
        :key="s.id"
        class="group mb-0.5 flex w-full flex-col rounded-md px-2.5 py-2 text-left transition"
        :class="[
          isActive(s.id)
            ? 'bg-(--accent)/15 text-(--accent)'
            : 'text-(--text-secondary) hover:bg-(--bg-primary) hover:text-(--text-primary)',
        ]"
        @click="handleResume(s.id)"
      >
        <span class="truncate text-xs font-medium leading-tight">{{ s.name }}</span>
        <span class="mt-0.5 text-[10px] text-(--text-muted)">
          {{ s.model.replace('claude-', '') }} · {{ s.message_count }} msgs
        </span>
      </button>
    </nav>

    <!-- View all link -->
    <div class="border-t border-(--border) px-3 py-2">
      <router-link
        to="/sessions"
        class="block text-center text-xs text-(--text-muted) transition hover:text-(--text-primary)"
      >
        View all sessions →
      </router-link>
    </div>
  </div>
</template>
