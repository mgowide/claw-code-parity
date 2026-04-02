# Phase 6 — PWA, Responsive, Error Handling

> Service worker, offline support, responsive layout, error boundaries, reconnect, toasts.

---

## PWA Setup

### `public/manifest.json`

```json
{
  "name": "Claw Code",
  "short_name": "Claw",
  "description": "AI Agent Web Interface",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#0d1117",
  "theme_color": "#58a6ff",
  "icons": [
    { "src": "/icons/icon-192.png", "sizes": "192x192", "type": "image/png" },
    { "src": "/icons/icon-512.png", "sizes": "512x512", "type": "image/png" }
  ]
}
```

### Service Worker Strategy

- **Cache-first** for static assets (JS, CSS, fonts, icons)
- **Network-first** for API calls
- **Stale-while-revalidate** for session data (offline browsing of past sessions)

```javascript
// public/sw.js (simplified)
self.addEventListener('fetch', (event) => {
  const url = new URL(event.request.url)
  
  if (url.pathname.startsWith('/api/sessions/')) {
    // Cache session data for offline browsing
    event.respondWith(
      caches.open('sessions').then(cache =>
        fetch(event.request)
          .then(response => { cache.put(event.request, response.clone()); return response })
          .catch(() => cache.match(event.request))
      )
    )
  }
})
```

### What Works Offline

| Feature | Offline | Notes |
|---|---|---|
| Browse past sessions | ✅ | Cached on first view |
| Read session transcripts | ✅ | Full message history cached |
| Start new chat | ❌ | Needs server + API key |
| Send messages | ❌ | Needs server |
| View files | ❌ | Needs server |
| Settings | ✅ | Cached config |

---

## Responsive Layout

### Breakpoints

| Width | Layout |
|---|---|
| ≥ 1200px | Full: sidebar + main + right panel |
| 768–1199px | Main + right panel (sidebar collapsed, hamburger menu) |
| < 768px | Main only (sidebar + right panel as overlays) |

### Implementation

```css
/* Sidebar */
.sidebar {
  width: 260px;
  transition: width 0.2s ease;
}

@media (max-width: 1199px) {
  .sidebar {
    position: fixed; left: -260px; z-index: 50;
  }
  .sidebar.open { left: 0; }
}

/* Right panel */
@media (max-width: 767px) {
  .right-panel {
    position: fixed; bottom: 0; width: 100%; height: 40vh;
    transform: translateY(100%);
  }
  .right-panel.open { transform: translateY(0); }
}
```

### Resizable Panels

```typescript
import { useDraggable } from '@vueuse/core'

// Sidebar resize handle
const sidebarWidth = useStorage('sidebar-width', 260)
// Right panel resize handle
const rightPanelWidth = useStorage('right-panel-width', 320)
// Terminal panel resize handle
const terminalHeight = useStorage('terminal-height', 200)
```

Drag handles appear on hover between panels. Min/max constraints enforced.

---

## Error Boundaries

### Component-Level

```vue
<!-- ErrorBoundary.vue -->
<script setup>
import { onErrorCaptured, ref } from 'vue'

const error = ref<Error | null>(null)

onErrorCaptured((err) => {
  error.value = err
  return false // prevent propagation
})
</script>

<template>
  <div v-if="error" class="error-fallback">
    <p>Something went wrong in this panel.</p>
    <button @click="error = null">Retry</button>
  </div>
  <slot v-else />
</template>
```

Wrap each major section:

```vue
<ErrorBoundary>
  <ChatThread />
</ErrorBoundary>
<ErrorBoundary>
  <RightPanel />
</ErrorBoundary>
```

If one panel errors, others keep working.

---

## WebSocket Reconnect

```typescript
// In useWebSocket.ts
const BACKOFF = [1000, 2000, 4000, 8000, 16000, 30000]
let attempt = 0

function reconnect() {
  status.value = 'connecting'
  const delay = BACKOFF[Math.min(attempt, BACKOFF.length - 1)]
  
  setTimeout(() => {
    try {
      connect()
      attempt = 0
      status.value = 'connected'
    } catch {
      attempt++
      reconnect()
    }
  }, delay)
}

// On disconnect:
ws.onclose = () => {
  status.value = 'disconnected'
  reconnect()
}
```

### Visual Indicator (StatusBar)

| Status | Visual |
|---|---|
| Connected | 🟢 green dot |
| Connecting | 🟡 yellow dot + "Reconnecting..." text |
| Disconnected | 🔴 red dot + "Disconnected" text |

---

## Toast Notifications

### useToast Composable

```typescript
interface Toast {
  id: string
  type: 'info' | 'success' | 'warning' | 'error'
  message: string
  duration: number  // ms, 0 = persistent
}

export function useToast() {
  const toasts = ref<Toast[]>([])

  function show(type, message, duration = 4000) {
    const toast = { id: uuid(), type, message, duration }
    toasts.value.push(toast)
    if (duration > 0) {
      setTimeout(() => dismiss(toast.id), duration)
    }
  }

  return { toasts, show, dismiss }
}
```

### Toast Events

| Event | Toast |
|---|---|
| WebSocket disconnected | ⚠️ "Connection lost. Reconnecting..." |
| WebSocket reconnected | ✅ "Connected" |
| Session compacted | ℹ️ "Session compacted: 12 messages removed" |
| Permission denied | ⚠️ "Permission denied for bash" |
| Export complete | ✅ "Session exported as Markdown" |
| Config saved | ✅ "Settings saved" |
| Error from server | ❌ error.message |

### Visual

```
┌─────────────────────────────────────┐
│                                     │ ← main app
│                                     │
│                                     │
│                                     │
│              ┌──────────────────┐   │
│              │ ✅ Connected      │   │ ← toast (bottom-right)
│              └──────────────────┘   │
└─────────────────────────────────────┘
```

Toasts stack vertically from bottom-right, slide in, auto-dismiss.

---

## Loading Skeletons

Use Naive UI's `<n-skeleton>` for loading states:

```vue
<template>
  <div v-if="loading">
    <n-skeleton text :repeat="3" />
    <n-skeleton text style="width: 60%" />
  </div>
  <div v-else>
    <!-- actual content -->
  </div>
</template>
```

Applied to:
- Session list while loading
- File tree while loading
- Settings while loading config
- Tool list while loading

---

*Back: [README](README.md) | Theming: [theming.md](theming.md) | Animations: [animations.md](animations.md)*
