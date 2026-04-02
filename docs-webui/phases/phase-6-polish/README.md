# Phase 6 — Polish & Visual Excellence

> **Goal**: Make it not just functional but genuinely beautiful and delightful.

---

## What This Phase Delivers

- **4 themes**: dark (default), light, solarized, high-contrast
- **Smooth animations**: message slide-in, tool card expand, page transitions
- **Three.js visuals**: splash screen, token flow, ambient particles (strategic)
- **Resizable panels**: drag-to-resize all panels, persisted
- **PWA**: installable, offline session reading, service worker
- **Reliability**: error boundaries, WebSocket reconnect, toast notifications

## Prerequisites

- All previous phases complete (UI fully functional)

## Steps

| # | Task | Detail Doc |
|---|---|---|
| 6.1 | Theme system | [theming.md](theming.md) |
| 6.2 | Animations | [animations.md](animations.md) |
| 6.3 | Three.js strategic visuals | [animations.md](animations.md) |
| 6.4 | Resizable panels + responsive | [pwa.md](pwa.md) |
| 6.5 | PWA + service worker | [pwa.md](pwa.md) |
| 6.6 | Error boundaries + reconnect + toasts | [pwa.md](pwa.md) |

## New Dependencies

```bash
npm install three @types/three lottie-web @vueuse/motion
```

## Acceptance Criteria

- [ ] 4 themes available and working (dark, light, solarized, high-contrast)
- [ ] Theme persisted in localStorage
- [ ] Messages animate in (slide from bottom)
- [ ] Tool cards expand/collapse with smooth animation
- [ ] Page transitions use fade/slide
- [ ] Three.js splash screen renders on first load
- [ ] All panels draggable to resize
- [ ] Panel sizes persisted across page reloads
- [ ] Layout collapses sidebar on screens < 768px
- [ ] PWA install prompt appears
- [ ] Offline mode: can browse past sessions without server
- [ ] WebSocket reconnects automatically with backoff
- [ ] Connection status indicator updates (green → yellow → red)
- [ ] Toast notifications for: error, compacted, permission denied, disconnected
- [ ] Component-level error boundaries (one broken panel doesn't crash page)

## Files Created

```
# Theming
src/styles/themes/dark.css
src/styles/themes/light.css
src/styles/themes/solarized.css
src/styles/themes/high-contrast.css
src/composables/useTheme.ts

# Animations
src/components/transitions/PageTransition.vue
src/components/transitions/ListTransition.vue

# Three.js
src/components/visuals/SplashScreen.vue
src/components/visuals/AmbientParticles.vue
src/components/visuals/TokenFlow.vue

# PWA
public/manifest.json
public/sw.js
src/composables/useServiceWorker.ts

# Error handling
src/components/ErrorBoundary.vue
src/components/ToastContainer.vue
src/composables/useToast.ts
```

---

*Detail docs: [theming.md](theming.md) | [animations.md](animations.md) | [pwa.md](pwa.md)*
