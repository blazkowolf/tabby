use std::io;
use std::path::PathBuf;
use structopt::StructOpt;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

#[derive(StructOpt, Debug)]
struct TabbyOpt {
    #[structopt(parse(from_os_str))]
    file: PathBuf,
}

fn main() -> Result<(), io::Error> {
    let args = TabbyOpt::from_args();
    // println!("{:#?}", args);
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;
    Ok(())
}
