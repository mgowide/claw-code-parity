import { ref, onMounted } from 'vue'

export function useServiceWorker() {
  const isInstallable = ref(false)
  const isOffline = ref(!navigator.onLine)
  let deferredPrompt: BeforeInstallPromptEvent | null = null

  interface BeforeInstallPromptEvent extends Event {
    prompt(): Promise<void>
    userChoice: Promise<{ outcome: 'accepted' | 'dismissed' }>
  }

  onMounted(() => {
    // Register service worker
    if ('serviceWorker' in navigator) {
      navigator.serviceWorker
        .register('/sw.js')
        .catch((err) => console.warn('[SW] Registration failed:', err))
    }

    // PWA install prompt
    window.addEventListener('beforeinstallprompt', (e) => {
      e.preventDefault()
      deferredPrompt = e as BeforeInstallPromptEvent
      isInstallable.value = true
    })

    // Offline detection
    window.addEventListener('online', () => (isOffline.value = false))
    window.addEventListener('offline', () => (isOffline.value = true))
  })

  async function install() {
    if (!deferredPrompt) return
    await deferredPrompt.prompt()
    const { outcome } = await deferredPrompt.userChoice
    if (outcome === 'accepted') isInstallable.value = false
    deferredPrompt = null
  }

  return { isInstallable, isOffline, install }
}
