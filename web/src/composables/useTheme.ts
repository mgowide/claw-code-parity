import { useStorage } from '@vueuse/core'
import { onMounted, watch } from 'vue'

export type ThemeName = 'dark' | 'light' | 'solarized' | 'high-contrast'

const THEMES: ThemeName[] = ['dark', 'light', 'solarized', 'high-contrast']

export function useTheme() {
  const theme = useStorage<ThemeName>('claw-theme', 'dark')

  function apply(name: ThemeName) {
    document.documentElement.setAttribute('data-theme', name)
  }

  function setTheme(name: ThemeName) {
    theme.value = name
    apply(name)
  }

  function cycleTheme() {
    const idx = THEMES.indexOf(theme.value)
    setTheme(THEMES[(idx + 1) % THEMES.length])
  }

  onMounted(() => apply(theme.value))

  return { theme, themes: THEMES, setTheme, cycleTheme }
}
