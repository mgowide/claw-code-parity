import { defineStore } from 'pinia'
import { ref } from 'vue'

/**
 * UI state that is not session-specific:
 * sidebar visibility, modal flags, and focus triggers.
 */
export const useUiStore = defineStore('ui', () => {
  const sidebarOpen = ref(true)
  const modelSelectorOpen = ref(false)
  /** Increment to trigger focus on the InputComposer textarea (Ctrl+/). */
  const composerFocusTrigger = ref(0)

  return { sidebarOpen, modelSelectorOpen, composerFocusTrigger }
})
