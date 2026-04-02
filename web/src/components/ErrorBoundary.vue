<script lang="ts">
import { defineComponent, ref, onErrorCaptured } from 'vue'

export default defineComponent({
  name: 'ErrorBoundary',
  setup(_, { slots }) {
    const error = ref<string | null>(null)

    onErrorCaptured((err) => {
      error.value = err instanceof Error ? err.message : String(err)
      console.error('[ErrorBoundary]', err)
      return false // prevent propagation
    })

    return () => {
      if (error.value) {
        return (
          <div class="flex flex-col items-center justify-center gap-3 p-6 text-center">
            <span class="text-2xl">⚠️</span>
            <p class="text-sm font-medium text-(--text-primary)">Something went wrong</p>
            <p class="max-w-sm text-xs text-(--text-muted)">{error.value}</p>
            <button
              class="mt-2 rounded-md border border-(--border) px-4 py-1.5 text-xs text-(--text-secondary) transition hover:text-(--text-primary)"
              onClick={() => (error.value = null)}
            >
              Retry
            </button>
          </div>
        )
      }
      return slots.default?.()
    }
  },
})
</script>
