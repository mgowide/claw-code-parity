import { defineStore } from 'pinia'
import { ref } from 'vue'
import { useStorage } from '@vueuse/core'

/**
 * UI state that is not session-specific:
 * sidebar visibility, modal flags, and focus triggers.
 */
export const useUiStore = defineStore('ui', () => {
  const sidebarOpen = ref(true)
  const modelSelectorOpen = ref(false)
  /** Increment to trigger focus on the InputComposer textarea (Ctrl+/). */
  const composerFocusTrigger = ref(0)

  // Right panel (Phase 5)
  const rightPanelOpen = useStorage('claw-right-panel-open', true)
  const rightPanelTab = useStorage<'tokens' | 'cost' | 'todos'>('claw-right-panel-tab', 'tokens')
  const rightPanelWidth = useStorage('claw-right-panel-width', 288)

  return {
    sidebarOpen,
    modelSelectorOpen,
    composerFocusTrigger,
    rightPanelOpen,
    rightPanelTab,
    rightPanelWidth,
  }
})
