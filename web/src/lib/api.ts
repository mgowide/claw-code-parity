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

// ── Status / Models ──────────────────────────────────────────────────────────────────────────────
const modelsCache: { data: ModelInfo[] | null; ts: number } = { data: null, ts: 0 }
const CACHE_TTL = 60_000 // 1 min

export interface ModelInfo {
  id: string
  provider: string
  active?: boolean
}

export interface StatusResponse {
  status: string
  model: string
  version: string
  models: ModelInfo[]
}

export function getStatus(): Promise<StatusResponse> {
  return apiFetch('/status')
}

export async function getModels(): Promise<ModelInfo[]> {
  if (modelsCache.data && Date.now() - modelsCache.ts < CACHE_TTL) return modelsCache.data
  const s = await getStatus()
  modelsCache.data = s.models
  modelsCache.ts = Date.now()
  return s.models
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
// ── Files ─────────────────────────────────────────────────────────────────────

export interface FileEntry {
  name: string
  path: string
  kind: 'file' | 'dir'
  size?: number
  lang?: string
}

export interface FileListResponse {
  path: string
  entries: FileEntry[]
}

export function getFiles(path = '.'): Promise<FileListResponse> {
  return apiFetch(`/files?path=${encodeURIComponent(path)}`)
}

export interface FileReadResponse {
  path: string
  content: string
  lang: string
  total_lines: number
  start_line: number
  end_line: number
}

export function readFile(path: string, start?: number, end?: number): Promise<FileReadResponse> {
  const params = new URLSearchParams({ path })
  if (start !== undefined) params.set('start', String(start))
  if (end !== undefined) params.set('end', String(end))
  return apiFetch(`/files/read?${params}`)
}

// ── Config ───────────────────────────────────────────────────────────────────

export function getConfig(): Promise<Record<string, unknown>> {
  return apiFetch('/config')
}

export function putConfig(config: Record<string, unknown>): Promise<{ ok: boolean }> {
  return apiFetch('/config', { method: 'PUT', body: JSON.stringify(config) })
}

// ── Tools ─────────────────────────────────────────────────────────────────────

export interface ToolEntry {
  name: string
  description: string
  permission: string
  source: string
  call_count: number
  enabled: boolean
}

export function getTools(): Promise<ToolEntry[]> {
  return apiFetch('/tools')
}

// ── MCP Servers ───────────────────────────────────────────────────────────────

export interface McpServer {
  id: string
  name: string
  transport: 'stdio' | 'sse'
  command?: string
  url?: string
  status: 'connected' | 'connecting' | 'error' | 'disconnected'
  error?: string
  tools: Array<{ name: string; description: string }>
  connected_at?: number
}

export function getMcpServers(): Promise<McpServer[]> {
  return apiFetch('/mcp/servers')
}

export function addMcpServer(body: {
  name: string
  transport: 'stdio' | 'sse'
  command?: string
  url?: string
}): Promise<McpServer> {
  return apiFetch('/mcp/servers', { method: 'POST', body: JSON.stringify(body) })
}

export function removeMcpServer(id: string): Promise<void> {
  return apiFetch(`/mcp/servers/${id}`, { method: 'DELETE' })
}

// ── Sessions ─────────────────────────────────────────────────────────────────

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
