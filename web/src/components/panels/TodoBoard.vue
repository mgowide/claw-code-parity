<script setup lang="ts">
import { useTodoStore } from '@/stores/todoStore'

const todoStore = useTodoStore()
</script>

<template>
  <div class="flex flex-col gap-3 p-4 text-xs text-(--text-primary)">
    <h3 class="text-[10px] font-semibold uppercase tracking-widest text-(--text-muted)">
      Todo Board
    </h3>

    <div v-if="todoStore.todos.length === 0" class="py-6 text-center text-(--text-muted)">
      No tasks yet
    </div>

    <div v-else class="flex flex-col gap-3">
      <!-- In Progress -->
      <div v-if="todoStore.inProgress.length > 0">
        <div class="mb-1 text-[10px] font-semibold uppercase tracking-wider text-amber-400">
          In Progress ({{ todoStore.inProgress.length }})
        </div>
        <div class="flex flex-col gap-1">
          <div
            v-for="t in todoStore.inProgress"
            :key="t.id"
            class="flex items-start gap-2 rounded-md border border-amber-500/30 bg-amber-500/5 px-2.5 py-1.5"
          >
            <span class="mt-0.5 inline-block h-2 w-2 flex-shrink-0 rounded-full bg-amber-400" />
            <span class="leading-snug text-(--text-primary)">{{ t.title }}</span>
          </div>
        </div>
      </div>

      <!-- Pending -->
      <div v-if="todoStore.pending.length > 0">
        <div class="mb-1 text-[10px] font-semibold uppercase tracking-wider text-(--text-muted)">
          Pending ({{ todoStore.pending.length }})
        </div>
        <div class="flex flex-col gap-1">
          <div
            v-for="t in todoStore.pending"
            :key="t.id"
            class="flex items-start gap-2 rounded-md border border-(--border) px-2.5 py-1.5"
          >
            <span class="mt-0.5 inline-block h-2 w-2 flex-shrink-0 rounded-full bg-(--border)" />
            <span class="leading-snug text-(--text-secondary)">{{ t.title }}</span>
          </div>
        </div>
      </div>

      <!-- Completed -->
      <div v-if="todoStore.completed.length > 0">
        <div class="mb-1 text-[10px] font-semibold uppercase tracking-wider text-emerald-400">
          Done ({{ todoStore.completed.length }})
        </div>
        <div class="flex flex-col gap-1">
          <div
            v-for="t in todoStore.completed"
            :key="t.id"
            class="flex items-start gap-2 rounded-md border border-emerald-500/30 bg-emerald-500/5 px-2.5 py-1.5"
          >
            <span class="mt-0.5 inline-block h-2 w-2 flex-shrink-0 rounded-full bg-emerald-500" />
            <span class="leading-snug text-(--text-secondary) line-through">{{ t.title }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
