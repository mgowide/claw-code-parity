<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { Bar, Doughnut } from 'vue-chartjs'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  ArcElement,
  Tooltip,
  Legend,
  Title,
} from 'chart.js'
import { getSessions } from '@/lib/api'
import type { SessionSummary } from '@/types/events'
import { useSessionStore } from '@/stores/sessionStore'

ChartJS.register(CategoryScale, LinearScale, BarElement, ArcElement, Tooltip, Legend, Title)

const store = useSessionStore()
const sessions = ref<SessionSummary[]>([])
const loading = ref(false)
const error = ref('')

async function load() {
  loading.value = true
  error.value = ''
  try {
    sessions.value = await getSessions()
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to load sessions'
  } finally {
    loading.value = false
  }
}

// Bar chart — cost per session (last 10)
const barData = computed(() => {
  const last10 = sessions.value.slice(-10)
  return {
    labels: last10.map((s) =>
      s.name.length > 14 ? s.name.slice(0, 14) + '…' : s.name,
    ),
    datasets: [
      {
        label: 'Cost (USD)',
        data: last10.map((s) => s.cost ?? 0),
        backgroundColor: 'rgba(99,102,241,0.6)',
        borderColor: 'rgba(99,102,241,1)',
        borderWidth: 1,
        borderRadius: 4,
      },
    ],
  }
})

const barOptions = {
  responsive: true,
  plugins: {
    legend: { display: false },
    title: { display: true, text: 'Cost per Session (last 10)', color: '#9ca3af', font: { size: 11 } },
  },
  scales: {
    x: { ticks: { color: '#6b7280', font: { size: 10 } }, grid: { color: '#374151' } },
    y: { ticks: { color: '#6b7280', font: { size: 10 } }, grid: { color: '#374151' } },
  },
}

// Doughnut — input vs output vs cache cost
const donutData = computed(() => {
  const inp = store.inputTokens * 0.000003
  const out = store.outputTokens * 0.000015
  const cache = store.cacheHits * 0.0000003
  return {
    labels: ['Input', 'Output', 'Cache'],
    datasets: [
      {
        data: [inp, out, cache],
        backgroundColor: ['rgba(59,130,246,0.7)', 'rgba(16,185,129,0.7)', 'rgba(245,158,11,0.7)'],
        borderColor: ['#1e40af', '#065f46', '#92400e'],
        borderWidth: 1,
      },
    ],
  }
})

const donutOptions = {
  responsive: true,
  plugins: {
    legend: { position: 'bottom' as const, labels: { color: '#9ca3af', font: { size: 10 } } },
    title: { display: true, text: 'Cost by Token Type', color: '#9ca3af', font: { size: 11 } },
  },
}

function exportCsv() {
  const rows = [['session_id', 'name', 'model', 'messages', 'cost']]
  sessions.value.forEach((s) => {
    rows.push([s.id, `"${s.name}"`, s.model, String(s.message_count), String(s.cost ?? 0)])
  })
  const csv = rows.map((r) => r.join(',')).join('\n')
  const blob = new Blob([csv], { type: 'text/csv' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'claw-costs.csv'
  a.click()
  URL.revokeObjectURL(url)
}

const totalCost = computed(() =>
  sessions.value.reduce((sum, s) => sum + (s.cost ?? 0), 0),
)

function usd(n: number) {
  return `$${n.toFixed(4)}`
}

onMounted(load)
</script>

<template>
  <div class="flex h-full flex-col">
    <!-- Header -->
    <div class="border-b border-(--border) bg-(--bg-secondary) px-6 py-4">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-sm font-semibold text-(--text-primary)">Cost Analytics</h1>
          <p class="mt-0.5 text-xs text-(--text-muted)">Usage and spend across all sessions</p>
        </div>
        <button
          class="rounded-md border border-(--border) px-3 py-1.5 text-xs text-(--text-secondary) transition hover:border-(--accent) hover:text-(--text-primary)"
          @click="exportCsv"
        >
          Export CSV
        </button>
      </div>
    </div>

    <div class="flex-1 overflow-y-auto p-6">
      <div v-if="loading" class="text-xs text-(--text-muted)">Loading…</div>
      <div v-else-if="error" class="text-xs text-red-400">{{ error }}</div>

      <div v-else class="flex flex-col gap-6">
        <!-- Summary cards -->
        <div class="grid grid-cols-3 gap-4">
          <div class="rounded-lg border border-(--border) bg-(--bg-secondary) p-4">
            <p class="text-[10px] text-(--text-muted)">Total Sessions</p>
            <p class="mt-1 text-2xl font-bold text-(--text-primary)">{{ sessions.length }}</p>
          </div>
          <div class="rounded-lg border border-(--border) bg-(--bg-secondary) p-4">
            <p class="text-[10px] text-(--text-muted)">Total Spend</p>
            <p class="mt-1 text-2xl font-bold text-(--accent)">{{ usd(totalCost) }}</p>
          </div>
          <div class="rounded-lg border border-(--border) bg-(--bg-secondary) p-4">
            <p class="text-[10px] text-(--text-muted)">Current Session</p>
            <p class="mt-1 text-2xl font-bold text-(--text-primary)">{{ usd(store.cost) }}</p>
          </div>
        </div>

        <!-- Charts -->
        <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
          <div class="rounded-lg border border-(--border) bg-(--bg-secondary) p-4">
            <Bar v-if="sessions.length" :data="barData" :options="barOptions" />
            <p v-else class="py-8 text-center text-xs text-(--text-muted)">No session data yet</p>
          </div>
          <div class="rounded-lg border border-(--border) bg-(--bg-secondary) p-4">
            <Doughnut
              v-if="store.inputTokens + store.outputTokens > 0"
              :data="donutData"
              :options="donutOptions"
            />
            <p v-else class="py-8 text-center text-xs text-(--text-muted)">
              Start a conversation to see cost breakdown
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
