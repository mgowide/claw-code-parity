# Phase 6 — Animations & Three.js

> Smooth transitions, micro-animations, and strategic 3D visuals.

---

## Vue Transitions

### Message Slide-In

```vue
<!-- ChatThread.vue -->
<TransitionGroup name="message" tag="div">
  <MessageBubble v-for="msg in messages" :key="msg.id" />
</TransitionGroup>

<style>
.message-enter-from { opacity: 0; transform: translateY(20px); }
.message-enter-active { transition: all 0.3s ease-out; }
.message-leave-active { transition: all 0.2s ease-in; }
.message-leave-to { opacity: 0; transform: translateX(-20px); }
</style>
```

### Tool Card Expand

```vue
<!-- ToolCallCard.vue body -->
<Transition name="expand">
  <div v-if="isExpanded" class="tool-body">
    <!-- input, output, diff -->
  </div>
</Transition>

<style>
.expand-enter-from, .expand-leave-to {
  max-height: 0; opacity: 0; overflow: hidden;
}
.expand-enter-active, .expand-leave-active {
  transition: max-height 0.3s ease, opacity 0.2s ease;
}
.expand-enter-to {
  max-height: 500px; opacity: 1;
}
</style>
```

### Page Transitions (Router)

```vue
<!-- App.vue -->
<router-view v-slot="{ Component }">
  <Transition name="page" mode="out-in">
    <component :is="Component" />
  </Transition>
</router-view>

<style>
.page-enter-from { opacity: 0; transform: translateX(10px); }
.page-enter-active { transition: all 0.2s ease-out; }
.page-leave-active { transition: all 0.15s ease-in; }
.page-leave-to { opacity: 0; }
</style>
```

---

## @vueuse/motion

For more complex animations beyond Vue's built-in `<Transition>`:

```vue
<script setup>
import { useMotion } from '@vueuse/motion'
const target = ref(null)
useMotion(target, {
  initial: { opacity: 0, y: 50 },
  enter: { opacity: 1, y: 0, transition: { duration: 400 } },
})
</script>
```

Use cases:
- Sidebar items stagger-fade on load
- Status bar elements slide in
- Modal overlay fade + scale

---

## Lottie Micro-Animations

Small animated illustrations for:

| Where | Animation |
|---|---|
| Tool success | Green checkmark burst |
| Tool error | Red X shake |
| Loading | Dots or spinner |
| Empty state | Illustrated "no sessions yet" |
| Send button | Morphs from arrow → stop icon while streaming |

```vue
<script setup>
import lottie from 'lottie-web'
onMounted(() => {
  lottie.loadAnimation({
    container: animRef.value,
    path: '/animations/success.json',
    loop: false,
    autoplay: true,
  })
})
</script>
```

---

## Three.js — Strategic Visuals

### Philosophy

Three.js is used for **atmosphere and delight**, not as the UI framework. Three.js components run in a `<canvas>` behind or beside the main UI and are **always optional** (togglable in settings).

### 1. Splash Screen (`SplashScreen.vue`)

Shown on first app load for ~2 seconds:
- Dark background with floating particle field
- Particles form the Claw logo briefly
- Dissolve into the app
- Skip on subsequent visits (stored in localStorage)

### 2. Ambient Particles (`AmbientParticles.vue`)

Optional background behind the chat:
- Very subtle, low-opacity floating nodes
- Density increases slightly when AI is thinking
- Performance: limited to 200 particles, `requestAnimationFrame` throttled
- Togglable in Settings → Appearance → "Ambient background"

### 3. Token Flow (`TokenFlow.vue`)

Visualization on the Cost page:
- Nodes represent tokens flowing from left (input) to right (output)
- Node color: blue (input), green (output), yellow (cached)
- Speed represents current throughput
- Only renders when CostView is active

### Performance Rules

- All canvases use `{ alpha: true, antialias: false, powerPreference: 'low-power' }`
- Max 200 particles / 1000 vertices
- FPS capped at 30 for ambient (60 for interactive)
- Automatically disabled if `navigator.hardwareConcurrency < 4`
- Disabled on mobile
- WebGL context lost → graceful fallback (hide canvas)

---

## Animation Performance Budget

| Location | Max Duration | Easing |
|---|---|---|
| Message slide-in | 300ms | ease-out |
| Tool card expand | 300ms | ease |
| Page transition | 200ms | ease-out |
| Modal overlay | 200ms | ease-out |
| Thinking pulse | 1500ms loop | ease-in-out |
| Toast slide-in | 250ms | ease-out |
| Three.js splash | 2000ms | custom |

**Rule**: No animation over 300ms in normal UI flow. Users should never feel like they're waiting for animation to finish.

---

*Back: [README](README.md) | Theming: [theming.md](theming.md) | PWA: [pwa.md](pwa.md)*
