/**
 * REST API client for session management endpoints.
 * All requests go through the Vite dev-proxy (or same-origin in production).
 */

import type { SessionSummary, SessionDetail } from '@/types/events'

const BASE = '/api'

async function apiFetch<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE}${path}`, {
    headers: { 'Content-Type': 'application/json' },
    ...init,
  })
  if (!res.ok) {
    const text = await res.text().catch(() => res.statusText)
    throw new Error(`API ${res.status}: ${text}`)
  }
  // 204 No Content — return void cast as T
  if (res.status === 204) return undefined as T
  return res.json() as Promise<T>
}

// ── Sessions ──────────────────────────────────────────────────────────────────

export function getSessions(): Promise<SessionSummary[]> {
  return apiFetch('/sessions')
}

export function getSession(id: string): Promise<SessionDetail> {
  return apiFetch(`/sessions/${id}`)
}

export function createSession(name?: string): Promise<{ session_id: string }> {
  return apiFetch('/sessions', {
    method: 'POST',
    body: JSON.stringify({ name: name ?? null }),
  })
}

export function deleteSession(id: string): Promise<void> {
  return apiFetch(`/sessions/${id}`, { method: 'DELETE' })
}

/**
 * Triggers a file download for the session export.
 * format: 'md' (Markdown) | 'json' (raw JSON)
 */
export async function exportSession(id: string, format: 'md' | 'json' = 'md'): Promise<void> {
  const res = await fetch(`${BASE}/sessions/${id}/export?format=${format}`)
  if (!res.ok) throw new Error(`Export failed: ${res.statusText}`)

  const disposition = res.headers.get('content-disposition') ?? ''
  const match = disposition.match(/filename="?([^";]+)"?/)
  const filename = match?.[1] ?? `session-${id}.${format}`

  const blob = await res.blob()
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = filename
  a.click()
  URL.revokeObjectURL(url)
}
