use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use profilectl_cli::commands;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;
use ratatui::Terminal;
use std::io::{self, BufRead, Write};

/// Top-level interactive menu actions.
///
/// The menu is the canonical TUI surface defined in `tool_workflow.md` (§3).
/// Order is fixed and is asserted in tests so that doc and code do not drift.
/// Destructive surface (`unlink`, `--force`, `bootstrap --remove`) is
/// deliberately omitted from the TUI; CLI users can still reach it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Init,
    Sync,
    Link,
    Bootstrap,
    Status,
    Profiles,
    Exit,
}

impl Action {
    const ALL: &'static [Action] = &[
        Self::Init,
        Self::Sync,
        Self::Link,
        Self::Bootstrap,
        Self::Status,
        Self::Profiles,
        Self::Exit,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::Sync => "sync",
            Self::Link => "link",
            Self::Bootstrap => "bootstrap",
            Self::Status => "status",
            Self::Profiles => "profiles",
            Self::Exit => "exit",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::Init => "first-time setup wizard",
            Self::Sync => "apply symlinks + install tools",
            Self::Link => "create dotfile symlinks",
            Self::Bootstrap => "wire shell sourcing + rendered tree",
            Self::Status => "show profile, drift, and verification",
            Self::Profiles => "list, show, and switch profiles",
            Self::Exit => "leave interactive mode",
        }
    }
}

/// Outcome of a single render/event loop.
enum LoopOutcome {
    /// User picked an action with Enter.
    Selected(Action),
    /// User pressed q / Esc / Ctrl-C to leave interactive mode.
    Quit,
}

struct App {
    profile: String,
    selected: usize,
}

impl App {
    fn new(profile: String) -> Self {
        Self {
            profile,
            selected: 0,
        }
    }

    fn next(&mut self) {
        self.selected = (self.selected + 1) % Action::ALL.len();
    }

    fn prev(&mut self) {
        self.selected = if self.selected == 0 {
            Action::ALL.len() - 1
        } else {
            self.selected - 1
        };
    }

    fn current(&self) -> Action {
        Action::ALL[self.selected]
    }
}

type Tui = Terminal<CrosstermBackend<io::Stdout>>;

/// Entry point for full interactive mode (no subcommand given).
pub fn run_interactive(profile: &str) -> Result<()> {
    let mut app = App::new(profile.to_string());

    loop {
        let mut terminal = setup_terminal()?;
        let outcome = run_menu_loop(&mut terminal, &mut app);
        restore_terminal(&mut terminal)?;

        match outcome? {
            LoopOutcome::Quit => return Ok(()),
            LoopOutcome::Selected(Action::Exit) => return Ok(()),
            LoopOutcome::Selected(action) => {
                dispatch(action)?;
                wait_for_enter()?;
            }
        }
    }
}

fn setup_terminal() -> Result<Tui> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn restore_terminal(terminal: &mut Tui) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_menu_loop(terminal: &mut Tui, app: &mut App) -> Result<LoopOutcome> {
    loop {
        terminal.draw(|frame| draw(frame, app))?;

        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(LoopOutcome::Quit),
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                return Ok(LoopOutcome::Quit);
            }
            KeyCode::Down | KeyCode::Char('j') => app.next(),
            KeyCode::Up | KeyCode::Char('k') => app.prev(),
            KeyCode::Enter => return Ok(LoopOutcome::Selected(app.current())),
            _ => {}
        }
    }
}

fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(4),
        Constraint::Min(5),
        Constraint::Length(3),
    ])
    .split(frame.area());

    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("profile: "),
            Span::styled(
                app.profile.as_str(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from("select an action to run"),
    ])
    .block(Block::default().borders(Borders::ALL).title(" profilectl "));
    frame.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = Action::ALL
        .iter()
        .enumerate()
        .map(|(index, action)| {
            let is_selected = index == app.selected;
            let indicator = if is_selected { "> " } else { "  " };
            let label = format!(
                "{indicator}{:<10} — {}",
                action.label(),
                action.description()
            );
            let style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(label).style(style)
        })
        .collect();
    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(" actions "));
    frame.render_widget(list, chunks[1]);

    let footer = Paragraph::new("↑/↓ or j/k: move   Enter: select   q/Esc/Ctrl-C: exit")
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[2]);
}

/// Run the recipe associated with a TUI action.
///
/// Recipes are the contract from `tool_workflow.md` §3.2: each TUI screen
/// composes one or more CLI invocations. While the underlying CLI subcommands
/// are still stubs, dispatching them in the documented order keeps the
/// surface honest and makes drift between doc and code visible.
fn dispatch(action: Action) -> Result<()> {
    match action {
        Action::Init => {
            commands::init::run(commands::init::InitArgs {
                force: false,
                non_interactive: false,
                from: None,
            })?;
            commands::bootstrap::run(commands::bootstrap::BootstrapArgs {
                shell: None,
                remove: false,
            })
        }
        Action::Sync => {
            commands::diff::run(commands::diff::DiffArgs {
                tools_only: false,
                links_only: false,
            })?;
            commands::sync::run(commands::sync::SyncArgs {
                tools_only: false,
                links_only: false,
                force: false,
            })
        }
        Action::Link => {
            commands::diff::run(commands::diff::DiffArgs {
                tools_only: false,
                links_only: true,
            })?;
            commands::link::run(commands::link::LinkArgs { force: false })
        }
        Action::Bootstrap => commands::bootstrap::run(commands::bootstrap::BootstrapArgs {
            shell: None,
            remove: false,
        }),
        Action::Status => {
            commands::status::run(commands::status::StatusArgs {})?;
            commands::check::run(commands::check::CheckArgs {
                tools_only: false,
                links_only: false,
            })?;
            commands::diff::run(commands::diff::DiffArgs {
                tools_only: false,
                links_only: false,
            })
        }
        Action::Profiles => commands::profile::run(commands::profile::ProfileArgs {
            command: commands::profile::ProfileCommand::List,
        }),
        Action::Exit => Ok(()),
    }
}

fn wait_for_enter() -> Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    writeln!(stdout, "\nPress Enter to return to the menu…")?;
    stdout.flush()?;
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_list_matches_workflow_doc_order() {
        let labels: Vec<&str> = Action::ALL.iter().map(|a| a.label()).collect();
        assert_eq!(
            labels,
            vec![
                "init",
                "sync",
                "link",
                "bootstrap",
                "status",
                "profiles",
                "exit"
            ],
        );
    }

    #[test]
    fn next_cycles_forward() {
        let mut app = App::new("default".to_string());
        assert_eq!(app.current(), Action::Init);
        app.next();
        assert_eq!(app.current(), Action::Sync);
        for _ in 0..(Action::ALL.len() - 1) {
            app.next();
        }
        assert_eq!(app.current(), Action::Init);
    }

    #[test]
    fn prev_wraps_to_last() {
        let mut app = App::new("default".to_string());
        app.prev();
        assert_eq!(app.current(), Action::Exit);
        app.prev();
        assert_eq!(app.current(), Action::Profiles);
    }

    #[test]
    fn every_action_has_label_and_description() {
        for action in Action::ALL {
            assert!(!action.label().is_empty());
            assert!(!action.description().is_empty());
        }
    }

    #[test]
    fn destructive_actions_are_not_in_tui() {
        let labels: Vec<&str> = Action::ALL.iter().map(|a| a.label()).collect();
        for forbidden in ["unlink", "scan"] {
            assert!(
                !labels.contains(&forbidden),
                "{forbidden} must not appear in the TUI top-level menu",
            );
        }
    }
}
