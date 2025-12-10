use ratatui::Terminal;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::{CrosstermBackend};
use std::io::*;

mod app;
mod cli;
mod db;
mod pager;
mod btree;

use app::App;
use cli::{run};

/// Ponto de entrada da aplicação de banco de dados chave-valor
/// 
/// # Setup
/// 1. Habilita modo raw do terminal
/// 2. Entra em tela alternativa
/// 3. Configura backend Crossterm
/// 4. Executa loop principal
/// 5. Limpa e restaura terminal
/// 
/// # Returns
/// * `Ok(())` - Execução bem-sucedida
/// * `Err(e)` - Erro durante execução
fn main() -> Result<()> {
    // Configura terminal para modo TUI
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    // Cria terminal Ratatui com backend Crossterm
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Inicializa aplicação e executa loop principal
    let mut app = App::new();
    let res = run(&mut terminal, &mut app);

    // Restaura terminal ao estado original
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Processa resultado da execução
    if let Ok(do_print) = res {
        if do_print {
            println!("See ya!");  // Mensagem de despedida
        }
    } else if let Err(err) = res {
        println!("{err:?}");  // Exibe erro se ocorreu
    }

    Ok(())
}
