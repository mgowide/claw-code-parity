# Phase 6 — Theme System

> Dark, light, solarized, and high-contrast themes via CSS custom properties.

---

## Architecture

Themes are implemented as CSS custom property sets. Switching themes changes a `data-theme` attribute on `<html>`, which activates the corresponding variable set.

### CSS Variables

```css
:root, [data-theme="dark"] {
  --bg-primary: #0d1117;
  --bg-secondary: #161b22;
  --bg-tertiary: #21262d;
  --bg-input: #0d1117;
  
  --text-primary: #e6edf3;
  --text-secondary: #8b949e;
  --text-muted: #484f58;
  
  --border: #30363d;
  --border-active: #58a6ff;
  
  --accent-blue: #58a6ff;
  --accent-green: #3fb950;
  --accent-red: #f85149;
  --accent-yellow: #d29922;
  --accent-purple: #bc8cff;
  
  --tool-running: #58a6ff;
  --tool-success: #3fb950;
  --tool-error: #f85149;
  
  --diff-added-bg: rgba(63, 185, 80, 0.15);
  --diff-removed-bg: rgba(248, 81, 73, 0.15);
  --diff-added-text: #3fb950;
  --diff-removed-text: #f85149;
  
  --message-user: #1c3a5f;
  --message-assistant: #161b22;
  
  --scrollbar-track: #0d1117;
  --scrollbar-thumb: #30363d;
  
  --shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  --radius: 8px;
}

[data-theme="light"] {
  --bg-primary: #ffffff;
  --bg-secondary: #f6f8fa;
  --bg-tertiary: #eaeef2;
  --text-primary: #1f2328;
  --text-secondary: #656d76;
  --border: #d0d7de;
  /* ... */
}

[data-theme="solarized"] {
  --bg-primary: #002b36;
  --bg-secondary: #073642;
  --text-primary: #839496;
  --accent-blue: #268bd2;
  /* ... */
}

[data-theme="high-contrast"] {
  --bg-primary: #000000;
  --text-primary: #ffffff;
  --border: #ffffff;
  --accent-blue: #00ffff;
  /* ... */
}
```

### useTheme Composable

```typescript
import { useStorage } from '@vueuse/core'

export function useTheme() {
  const theme = useStorage('claw-theme', 'dark')

  function setTheme(name: 'dark' | 'light' | 'solarized' | 'high-contrast') {
    document.documentElement.setAttribute('data-theme', name)
    theme.value = name
  }

  // Apply stored theme on load
  onMounted(() => {
    document.documentElement.setAttribute('data-theme', theme.value)
  })

  return { theme, setTheme }
}
```

### Naive UI Theme Integration

```typescript
// In App.vue
import { darkTheme, lightTheme } from 'naive-ui'

const naiveTheme = computed(() => {
  return theme.value === 'light' ? lightTheme : darkTheme
})
```

```vue
<template>
  <n-config-provider :theme="naiveTheme">
    <AppShell />
  </n-config-provider>
</template>
```

---

## Theme Picker

In `SettingsView.vue`:

```
┌──────────────────────────────┐
│  Theme                       │
│                              │
│  ● Dark (default)            │
│  ○ Light                     │
│  ○ Solarized                 │
│  ○ High Contrast             │
│                              │
│  [Preview] ← live preview    │
└──────────────────────────────┘
```

Immediate visual switch — no page reload.

---

*Back: [README](README.md) | Animations: [animations.md](animations.md) | PWA: [pwa.md](pwa.md)*
