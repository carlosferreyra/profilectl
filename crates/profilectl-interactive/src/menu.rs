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
/// Mirrors `tool_workflow.md`: the TUI is a strict subset of the CLI. Each
/// entry composes one or more CLI invocations (the "recipe" model).
/// CLI-only verbs (`uninstall`, `check`, `scan`) are deliberately absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Apply,
    Status,
    Profiles,
    Exit,
}

impl Action {
    const ALL: &'static [Action] = &[Self::Apply, Self::Status, Self::Profiles, Self::Exit];

    fn label(self) -> &'static str {
        match self {
            Self::Apply => "apply",
            Self::Status => "status",
            Self::Profiles => "profiles",
            Self::Exit => "exit",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::Apply => "preview drift, then apply the active profile",
            Self::Status => "show drift between profile and machine",
            Self::Profiles => "list, view, switch, or publish profiles",
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
                "{indicator}{:<8} — {}",
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

/// Dispatch a TUI selection to the underlying CLI recipe.
///
/// Recipes (per `tool_workflow.md` §3.2):
/// * `Apply`    → `status` (preview)  then `apply`
/// * `Status`   → `status`
/// * `Profiles` → submenu (placeholder: prints not-yet-implemented)
fn dispatch(action: Action) -> Result<()> {
    match action {
        Action::Apply => {
            commands::status::run(commands::status::StatusArgs {
                scope: commands::scope::Scope::All,
            })?;
            commands::apply::run(commands::apply::ApplyArgs {
                scope: commands::scope::Scope::All,
                pull: false,
                force: false,
                strict: false,
            })
        }
        Action::Status => commands::status::run(commands::status::StatusArgs {
            scope: commands::scope::Scope::All,
        }),
        Action::Profiles => {
            println!("profilectl profiles submenu: not yet implemented");
            Ok(())
        }
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
        assert_eq!(labels, vec!["apply", "status", "profiles", "exit"]);
    }

    #[test]
    fn next_cycles_forward() {
        let mut app = App::new("default".to_string());
        assert_eq!(app.current(), Action::Apply);
        app.next();
        assert_eq!(app.current(), Action::Status);
        for _ in 0..(Action::ALL.len() - 1) {
            app.next();
        }
        assert_eq!(app.current(), Action::Apply);
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

    /// Guard: destructive or CI-only verbs must never be reachable from the
    /// main TUI menu. `publish` lives only inside the Profiles submenu.
    #[test]
    fn destructive_actions_are_not_in_tui() {
        let labels: Vec<&str> = Action::ALL.iter().map(|a| a.label()).collect();
        for forbidden in ["uninstall", "check", "scan", "publish", "init"] {
            assert!(
                !labels.contains(&forbidden),
                "TUI main menu must not expose `{forbidden}`",
            );
        }
    }
}
