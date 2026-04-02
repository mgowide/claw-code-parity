const CACHE_NAME = 'claw-v1'
const STATIC_CACHE = 'claw-static-v1'
const SESSION_CACHE = 'claw-sessions-v1'

// Assets to pre-cache on install
const PRECACHE_URLS = ['/', '/index.html']

self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(STATIC_CACHE).then((cache) => cache.addAll(PRECACHE_URLS)),
  )
  self.skipWaiting()
})

self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((keys) =>
      Promise.all(
        keys
          .filter((k) => k !== STATIC_CACHE && k !== SESSION_CACHE)
          .map((k) => caches.delete(k)),
      ),
    ),
  )
  self.clients.claim()
})

self.addEventListener('fetch', (event) => {
  const url = new URL(event.request.url)

  // Network-only for non-GET and WebSocket upgrades
  if (event.request.method !== 'GET') return

  // Cache sessions for offline browsing (stale-while-revalidate)
  if (url.pathname.startsWith('/api/sessions')) {
    event.respondWith(
      caches.open(SESSION_CACHE).then((cache) =>
        fetch(event.request)
          .then((response) => {
            cache.put(event.request, response.clone())
            return response
          })
          .catch(() => cache.match(event.request)),
      ),
    )
    return
  }

  // Cache-first for static JS/CSS/font assets
  if (
    url.pathname.startsWith('/assets/') ||
    url.pathname.endsWith('.css') ||
    url.pathname.endsWith('.js')
  ) {
    event.respondWith(
      caches.open(STATIC_CACHE).then((cache) =>
        cache.match(event.request).then(
          (cached) =>
            cached ||
            fetch(event.request).then((response) => {
              cache.put(event.request, response.clone())
              return response
            }),
        ),
      ),
    )
    return
  }

  // Network-first fallback for everything else (including /api/*)
  event.respondWith(
    fetch(event.request).catch(() => caches.match(event.request)),
  )
})
