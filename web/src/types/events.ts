export type ServerEvent =
  | { type: 'text_delta'; text: string }
  | { type: 'thinking_start' }
  | { type: 'thinking_end' }
  | { type: 'tool_use_start'; id: string; name: string; input: unknown }
  | { type: 'tool_result'; id: string; output: string; is_error: boolean }
  | { type: 'diff'; path: string; old_content: string; new_content: string }
  | { type: 'usage'; input_tokens: number; output_tokens: number; cache_hits: number; cost: number }
  | { type: 'turn_complete'; turn_index: number }
  | { type: 'permission_request'; id: string; tool: string; description: string }
  | { type: 'session_compacted'; removed_messages: number }
  | { type: 'error'; message: string }
  | { type: 'connected'; session_id: string; model: string }

export type ClientCommand =
  | { type: 'send_message'; session_id: string; text: string }
  | { type: 'cancel_turn'; session_id: string }
  | { type: 'approve_permission'; request_id: string }
  | { type: 'deny_permission'; request_id: string }
  | { type: 'compact'; session_id: string }
  | { type: 'switch_model'; model: string }
  | { type: 'resume_session'; session_id: string }

export interface Message {
  id: string
  role: 'user' | 'assistant'
  content: string
  isStreaming: boolean
  timestamp: number
}

export interface ToolCall {
  id: string
  name: string
  input: Record<string, unknown>
  output?: string
  isError?: boolean
  status: 'running' | 'success' | 'error'
  startTime: number
  endTime?: number
  diff?: { path: string; oldContent: string; newContent: string }
}

export interface PermissionRequest {
  id: string
  tool: string
  description: string
}
