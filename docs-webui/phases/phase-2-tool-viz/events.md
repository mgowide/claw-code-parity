# Phase 2 — Tool Event WebSocket Handlers

> How tool execution events flow from Rust backend to Vue frontend.

---

## Backend Changes (Step 2.6)

The WebSocket handler in Phase 1 already sends `ServerEvent` variants. Phase 2 adds the tool-specific event mapping from `AssistantEvent` to `ServerEvent`.

### AssistantEvent → ServerEvent Mapping

```rust
// In ws_handler, when processing the ConversationRuntime stream:

match event {
    AssistantEvent::TextDelta(text) => {
        send(ServerEvent::TextDelta { text });
    }
    AssistantEvent::ToolUse { id, name, input } => {
        send(ServerEvent::ToolUseStart { id, name, input });
        
        // Execute the tool
        let result = tool_executor.execute(&name, &input).await;
        
        // Check if this was an edit_file → also send diff
        if name == "edit_file" || name == "write_file" {
            if let Some(diff) = extract_diff(&result) {
                send(ServerEvent::Diff {
                    path: diff.path,
                    old_content: diff.old,
                    new_content: diff.new,
                });
            }
        }
        
        send(ServerEvent::ToolResult {
            id,
            output: result.output,
            is_error: result.is_error,
        });
    }
    AssistantEvent::Usage(usage) => {
        send(ServerEvent::Usage {
            input_tokens: usage.input_tokens,
            output_tokens: usage.output_tokens,
            cache_hits: usage.cache_read_tokens,
            cost: calculate_cost(&usage),
        });
    }
    AssistantEvent::MessageStop => {
        send(ServerEvent::TurnComplete { turn_index });
    }
}
```

### Permission Interception

When a tool requires permission approval:

```rust
// Before executing tool:
if requires_permission(&tool_name, &config.permission_mode) {
    let request_id = uuid::Uuid::new_v4().to_string();
    
    // Send permission request to frontend
    send(ServerEvent::PermissionRequest {
        id: request_id.clone(),
        tool: tool_name.clone(),
        description: format_tool_description(&tool_name, &input),
    });
    
    // Wait for approval/denial from client
    match wait_for_permission_response(&request_id).await {
        PermissionResponse::Approved => { /* proceed with execution */ }
        PermissionResponse::Denied => {
            send(ServerEvent::ToolResult {
                id: tool_id,
                output: "Permission denied by user".into(),
                is_error: true,
            });
            return;
        }
    }
}
```

---

## Frontend Event Processing

### Updates to `useStream.ts`

```typescript
function processEvent(event: ServerEvent) {
  switch (event.type) {
    case 'text_delta':
      sessionStore.appendDelta(event.text)
      break

    // NEW in Phase 2:
    case 'tool_use_start':
      sessionStore.addToolCall({
        id: event.id,
        name: event.name,
        input: event.input,
        status: 'running',
        startTime: Date.now(),
      })
      break

    case 'tool_result':
      sessionStore.resolveToolCall(event.id, {
        output: event.output,
        isError: event.is_error,
        status: event.is_error ? 'error' : 'success',
        endTime: Date.now(),
      })
      break

    case 'diff':
      sessionStore.attachDiff(event.id, {
        path: event.path,
        oldContent: event.old_content,
        newContent: event.new_content,
      })
      break

    case 'permission_request':
      sessionStore.addPermissionRequest({
        id: event.id,
        tool: event.tool,
        description: event.description,
      })
      break

    // ... existing handlers
  }
}
```

### Updates to `sessionStore.ts`

New state additions:

```typescript
interface ToolCall {
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

interface PermissionRequest {
  id: string
  tool: string
  description: string
}

// Added to SessionState:
toolCalls: ToolCall[]
pendingPermissions: PermissionRequest[]
alwaysAllowedTools: Set<string>
```

---

## Event Sequence Examples

### Example: `read_file` Tool Call

```
Server → Client:
1. {"type": "tool_use_start", "id": "t1", "name": "read_file", "input": {"path": "src/main.rs"}}
2. {"type": "tool_result", "id": "t1", "output": "fn main() {...}", "is_error": false}

UI behavior:
1. ToolCallCard appears with spinner, title "read_file"
2. Card resolves: green border, output shown as CodeBlock
```

### Example: `edit_file` with Diff

```
Server → Client:
1. {"type": "tool_use_start", "id": "t2", "name": "edit_file", "input": {"path": "src/lib.rs", ...}}
2. {"type": "diff", "path": "src/lib.rs", "old_content": "...", "new_content": "..."}
3. {"type": "tool_result", "id": "t2", "output": "File edited successfully", "is_error": false}

UI behavior:
1. ToolCallCard appears with spinner
2. DiffViewer auto-opens inside the card showing colored diff
3. Card resolves: green border
```

### Example: `bash` Requiring Permission

```
Server → Client:
1. {"type": "permission_request", "id": "p1", "tool": "bash", "description": "Run: npm install"}

Client → Server (user clicks Allow):
2. {"type": "approve_permission", "request_id": "p1"}

Server → Client:
3. {"type": "tool_use_start", "id": "t3", "name": "bash", "input": {"command": "npm install"}}
4. {"type": "tool_result", "id": "t3", "output": "added 150 packages...", "is_error": false}

UI behavior:
1. PermissionModal appears: "bash wants to run: npm install" [Deny] [Allow]
2. User clicks Allow → modal closes
3. ToolCallCard appears with spinner
4. Card resolves with TerminalPanel showing colored output
```

---

*Back: [README](README.md) | Components: [components.md](components.md)*
