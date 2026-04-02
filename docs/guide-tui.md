# Building a Full-Screen TUI with ratatui

This guide walks through upgrading the current inline scroll output into a proper terminal UI using [ratatui](https://ratatui.rs). The TUI-ENHANCEMENT-PLAN.md in the repo covers the full 6-phase roadmap — this document focuses on implementation details.

---

## Current State

The existing CLI already has solid terminal primitives:

| Component | File | What It Does |
|---|---|---|
| `render.rs` (641 lines) | Markdown → terminal rendering: headings, tables, code blocks with `syntect` highlighting, `Spinner` widget |
| `input.rs` (269 lines) | `rustyline`-based line editor with slash-command tab completion, Shift+Enter multiline |
| `main.rs` (~3,159 lines) | `LiveCli` — the monolithic REPL loop, streaming output, tool call display, permission prompting |

All terminal control uses `crossterm`, which is also the backend for `ratatui`. No conflicting terminal libraries.

---

## Target Layout

```
┌──────────────────────────────────────┬────────────────────┐
│                                      │  🔧 Tool Status     │
│  Conversation View (scrollable)      │  ───────────────    │
│                                      │  ✓ bash (0.3s)      │
│  User: fix the failing test          │  ✓ read_file        │
│                                      │  ⟳ edit_file...     │
│  🦀 Running cargo test...            │                     │
│  ╭─ bash ──────────────╮             │  📋 Todos            │
│  │ cargo test          │             │  ───────────────    │
│  ╰─────────────────────╯             │  ✓ Find failing test│
│  ✓ bash (exit 0)                     │  ⟳ Fix the error    │
│                                      │  ○ Run tests again  │
├──────────────────────────────────────┴────────────────────┤
│  model: opus | tokens: 12,451 in / 3,200 out | $0.42 | main │
├───────────────────────────────────────────────────────────┤
│  > _                                                       │
└───────────────────────────────────────────────────────────┘
```

**Regions:**

| Region | Widget | Description |
|---|---|---|
| Conversation | Scrollable `Paragraph` | Full message history, Markdown rendered |
| Sidebar (optional) | `List` | Tool calls + todo list |
| Status bar | `Paragraph` / `Line` | Model, tokens, cost, git branch, elapsed |
| Input | Text area with cursor | User typing area |

---

## Prerequisites

Add `ratatui` behind a feature flag so it doesn't affect the default inline REPL build:

```toml
# rust/crates/rusty-claude-cli/Cargo.toml

[features]
default = []
full-tui = ["ratatui"]

[dependencies]
ratatui = { version = "0.29", optional = true, features = ["crossterm"] }
```

Activate with:

```bash
cargo build --release --features full-tui
```

Launch with:

```bash
claw --tui
```

---

## Module Structure

```
crates/rusty-claude-cli/src/
├── main.rs              # Entrypoint — dispatch to REPL or TUI based on --tui flag
├── app.rs               # LiveCli (existing inline REPL — unchanged)
├── tui/
│   ├── mod.rs           # TuiApp struct — owns the ratatui Terminal + event loop
│   ├── layout.rs        # Constraint-based layout (conversation / sidebar / status / input)
│   ├── conversation.rs  # ScrollableConversation widget — renders messages as styled spans
│   ├── status_bar.rs    # StatusBar widget — model, tokens, cost, branch, timer
│   ├── tool_panel.rs    # ToolPanel widget — sidebar with tool call history + todos
│   ├── input_area.rs    # InputArea widget — multiline text editor with cursor
│   ├── diff_view.rs     # Colored unified diff renderer
│   ├── pager.rs         # Internal pager for long outputs (PgUp/PgDn/q)
│   ├── theme.rs         # Named color themes (dark, light, solarized, catppuccin)
│   └── events.rs        # Crossterm event reader + app-level key dispatch
```

---

## Step-by-Step Implementation

### Step 1: TuiApp Skeleton

```rust
// tui/mod.rs
use std::io;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::execute;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

mod conversation;
mod events;
mod input_area;
mod layout;
mod status_bar;
mod theme;
mod tool_panel;

pub struct TuiApp {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    // ... ConversationRuntime handle, shared state
    should_quit: bool,
}

impl TuiApp {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self {
            terminal,
            should_quit: false,
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            self.terminal.draw(|frame| {
                layout::render(frame, &self.state);
            })?;

            if let Event::Key(key) = event::read()? {
                events::handle_key(key, &mut self.state);
            }

            if self.should_quit {
                break;
            }
        }
        Ok(())
    }
}

impl Drop for TuiApp {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
    }
}
```

### Step 2: Layout

```rust
// tui/layout.rs
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

pub fn render(frame: &mut Frame, state: &AppState) {
    // Main horizontal split: conversation (75%) | sidebar (25%)
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(frame.area());

    // Left: vertical split: conversation | status bar | input
    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),       // conversation (fills remaining)
            Constraint::Length(1),    // status bar
            Constraint::Length(3),    // input area
        ])
        .split(horizontal[0]);

    conversation::render(frame, left[0], state);
    status_bar::render(frame, left[1], state);
    input_area::render(frame, left[2], state);

    // Right sidebar: tool panel + todos
    tool_panel::render(frame, horizontal[1], state);
}
```

### Step 3: Status Bar

```rust
// tui/status_bar.rs
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let spans = vec![
        Span::styled(
            format!(" {} ", state.model),
            Style::default().fg(Color::Black).bg(Color::Cyan),
        ),
        Span::raw(" │ "),
        Span::styled(
            format!("{}↓ {}↑", state.input_tokens, state.output_tokens),
            Style::default().fg(Color::Green),
        ),
        Span::raw(" │ "),
        Span::styled(
            format!("${:.2}", state.cost),
            Style::default().fg(Color::Yellow),
        ),
        Span::raw(" │ "),
        Span::styled(
            format!(" {}", state.git_branch),
            Style::default().fg(Color::Magenta),
        ),
        Span::raw(" │ "),
        Span::styled(
            format!("{}s", state.turn_elapsed.as_secs()),
            Style::default().fg(Color::DarkGray),
        ),
    ];
    let bar = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(Color::DarkGray));
    frame.render_widget(bar, area);
}
```

### Step 4: Conversation View with Scrolling

```rust
// tui/conversation.rs
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let mut lines: Vec<Line> = Vec::new();

    for message in &state.messages {
        match message.role {
            Role::User => {
                lines.push(Line::from(Span::styled(
                    format!("You: {}", message.text),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                )));
            }
            Role::Assistant => {
                // Render markdown spans (bold, code, headings)
                lines.extend(markdown_to_lines(&message.text));
            }
            Role::ToolUse => {
                lines.push(Line::from(vec![
                    Span::styled("╭─ ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&message.tool_name, Style::default().fg(Color::Cyan)),
                    Span::styled(" ─╮", Style::default().fg(Color::DarkGray)),
                ]));
                // ... tool input + result
            }
        }
        lines.push(Line::default()); // blank separator
    }

    let paragraph = Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::NONE))
        .wrap(Wrap { trim: false })
        .scroll((state.scroll_offset, 0)); // PgUp/PgDn controlled

    frame.render_widget(paragraph, area);
}
```

### Step 5: Wiring to ConversationRuntime

The key integration point — `ConversationRuntime` uses trait boundaries (`ApiClient`, `ToolExecutor`), not stdout. You bridge via channels:

```rust
use std::sync::mpsc;
use runtime::{AssistantEvent, ConversationRuntime};

// In the TUI event loop:
let (event_tx, event_rx) = mpsc::channel::<AssistantEvent>();

// Spawn the runtime on a background thread/task:
std::thread::spawn(move || {
    // ConversationRuntime sends AssistantEvents back via the channel
    let events = runtime.run_turn(&user_message);
    for event in events {
        event_tx.send(event).ok();
    }
});

// In the TUI render loop:
while let Ok(event) = event_rx.try_recv() {
    match event {
        AssistantEvent::TextDelta(text) => state.append_text(&text),
        AssistantEvent::ToolUse { name, input, .. } => state.add_tool_call(&name, &input),
        AssistantEvent::Usage(usage) => state.update_usage(usage),
        AssistantEvent::MessageStop => state.mark_turn_complete(),
        _ => {}
    }
}
```

---

## Key Bindings

| Key | Action |
|---|---|
| `Enter` | Send message |
| `Shift+Enter` | Newline in input |
| `PgUp` / `PgDn` | Scroll conversation |
| `Ctrl+C` | Cancel current turn / quit |
| `Ctrl+L` | Clear and redraw |
| `Tab` | Autocomplete slash commands |
| `?` or `F1` | Toggle help overlay |
| `Esc` | Close sidebar / pager |

---

## Color Themes

```rust
// tui/theme.rs
pub enum ThemeName {
    Dark,      // default
    Light,
    Solarized,
    Catppuccin,
}

pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
    pub user_msg: Color,
    pub assistant_msg: Color,
    pub tool_border: Color,
    pub status_bg: Color,
    pub error: Color,
    pub success: Color,
}
```

Select via config:

```json
{ "theme": "catppuccin" }
```

Or at runtime:

```
/config theme solarized
```

---

## Testing Without a Terminal

All widgets take pure state structs and return ratatui render primitives. Test rendering logic without a real terminal:

```rust
#[test]
fn status_bar_shows_model_and_cost() {
    let state = AppState {
        model: "opus".into(),
        cost: 0.42,
        input_tokens: 12451,
        output_tokens: 3200,
        ..Default::default()
    };
    let buffer = render_status_bar_to_buffer(&state, 80, 1);
    assert!(buffer.contains("opus"));
    assert!(buffer.contains("$0.42"));
}
```

---

## Migration Path

| Phase | What Happens | Breaks Anything? |
|---|---|---|
| 0 | Extract `main.rs` monolith into `app.rs`, `format.rs`, `session_mgr.rs` | No — same behavior |
| 1 | Add status bar below the inline REPL (not full-screen yet) | No — additive |
| 2 | Improve streaming output (live markdown, thinking indicator) | No — additive |
| 3 | Add collapsible tool output + tool timeline | No — additive |
| 4 | Enhanced slash commands (colored diff, pager, undo) | No — additive |
| 5 | Color themes | No — additive |
| 6 | Full `ratatui` alternate-screen mode behind `--tui` flag | No — opt-in |

Every phase ships independently. The inline REPL remains the default throughout.
