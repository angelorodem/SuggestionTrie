#![feature(round_char_boundary)]
extern crate suggestion_trie;
use argh::FromArgs;
use suggestion_trie::Suggestion;
use suggestion_trie::{TrieInputData, TrieRoot};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;
use std::time::Instant;
use std::{error::Error, time::Duration};
use tui::{
    backend::Backend,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame, Terminal,
};
use walkdir::WalkDir;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{List, ListItem, Paragraph},
};
use unicode_width::UnicodeWidthStr;

static BEFORE_FUZZING: usize = 1;
static MAX_NODE_RESULTS: usize = 30;

enum InputMode {
    Normal,
    Editing,
}

fn trie_data_file(title: String, path: String) -> TrieInputData<HashMap<String,String>> {
    let data = HashMap::from([("path".to_string(), path)]);
    let suggestion = Suggestion::new(title.clone(), Some(data));
    TrieInputData {
        keywords: vec![title.to_lowercase()], // lowercase for ease of matching
        suggestion,
    }
}

fn trie_data_words(title: String) -> TrieInputData<HashMap<String,String>> {
    let suggestion = Suggestion::new(title.clone(), None);
    TrieInputData {
        keywords: vec![title.to_lowercase()], // lowercase for ease of matching
        suggestion,
    }
}

/// App holds the state of the application
struct App {
    words: bool,
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// Trie results
    suggestions: Option<Vec<Suggestion<HashMap<String,String>>>>,
}

#[derive(FromArgs)]
#[argh(description = "...")]
struct Cli {
    /// folder to list all files inside
    #[argh(option, short = 'f', default = "String::from(\".\")")]
    folder: String,
    /// instead of file read a list of words
    #[argh(switch, short = 'w')]
    words: bool,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            suggestions: None,
            words: false,
        }
    }
}

fn list_folder(folder: &str) -> Result<Vec<(String, String)>, std::io::Error> {
    let dir_iterator = WalkDir::new(folder); //fs::read_dir(folder)?;
    let files: Vec<(String, String)> = dir_iterator
        .into_iter()
        .map(|x| {
            let x = x.unwrap();
            (
                String::from(x.file_name().to_str().unwrap()),
                String::from(x.path().to_str().unwrap()),
            )
        })
        .collect();
    Ok(files)
}

fn read_file_lines(path: &str) -> Result<Vec<String>, std::io::Error> {
    Ok(read_to_string(path)?.lines().map(String::from).collect())
}

fn setup_trie_words(words: Vec<String>) -> (TrieRoot<HashMap<String,String>>, usize) {
    let mut root = TrieRoot::new(BEFORE_FUZZING, Some(MAX_NODE_RESULTS));
    let mut entries: Vec<TrieInputData<HashMap<String,String>>> = vec![];

    let number_words = words.len();
    for file in words {
        entries.push(trie_data_words(file));
    }

    root.build(entries.as_slice());

    (root, number_words)
}

fn setup_trie_files(files: Vec<(String, String)>) -> (TrieRoot<HashMap<String,String>>, usize) {
    let mut root = TrieRoot::new(BEFORE_FUZZING,Some(MAX_NODE_RESULTS));
    let mut entries: Vec<TrieInputData<HashMap<String,String>>> = vec![];

    let number_files = files.len();
    for (file, path) in files {
        entries.push(trie_data_file(file, path));
    }

    root.build(entries.as_slice());

    (root, number_files)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Cli = argh::from_env();

    let trie_setup = if !args.words {
        let all_files = list_folder(&args.folder)?;
        setup_trie_files(all_files)
    } else {
        let all_words = read_file_lines(&args.folder)?;
        setup_trie_words(all_words)
    };


    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App { words: args.words, ..Default::default() };
    let res = run_app(&mut terminal, app, trie_setup);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    trie_data: (TrieRoot<HashMap<String,String>>, usize),
) -> io::Result<()> {
    let mut last_latency = Duration::from_millis(0);
    loop {
        terminal.draw(|f| ui(f, &app, trie_data.1, last_latency))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Char(c) => {
                        app.input.push(c);
                        let measure_start = Instant::now();
                        app.suggestions = trie_data.0.get_suggestions(&app.input.to_lowercase()); // lowercase for ease of matching
                        last_latency = measure_start.elapsed();
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                        let measure_start = Instant::now();
                        app.suggestions = trie_data.0.get_suggestions(&app.input); // lowercase for ease of matching
                        last_latency = measure_start.elapsed();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App, number_files: usize, last_latency: Duration) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start typing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop typing."),
            ],
            Style::default(),
        ),
    };

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);

    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }
    let suggestion_type = if app.words {
        "Words"
    } else {
        "Files"
    };
    if let Some(sug) = &app.suggestions {
        let messages: Vec<ListItem> = sug
            .iter()
            .map(|m| {
                let style = Style::default().fg(Color::LightGreen);
                let path = if let Some(v) = m.data.as_ref() {
                    Span::raw(format!(
                        "    -    Path: {}",
                            v                           
                            .get(&"path".to_string())
                            .unwrap_or(&"No path".to_string())
                    ))
                } else {
                    Span::raw("")
                };

                let content = vec![Spans::from(vec![
                    Span::styled(format!("{}", m.title), style),
                    path,
                ])];
                ListItem::new(content)
            })
            .collect();        
        let messages =
            List::new(messages).block(Block::default().borders(Borders::ALL).title(format!(
                "{} suggestions {}/{} in {:?}",
                suggestion_type,
                sug.len(),
                number_files,
                last_latency
            )));
        f.render_widget(messages, chunks[2]);
    } else {
        let messages = List::new(vec![ListItem::new(vec![Spans::from(Span::raw(
            "No suggestions",
        ))])])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("{} suggestions 0/{}",suggestion_type,  number_files)),
        );
        f.render_widget(messages, chunks[2]);
    }
}
