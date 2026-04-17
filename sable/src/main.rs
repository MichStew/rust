use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEvenetKind};


// docs say ratatui apps usually 
// init terminal
// loop app till user closes 
// restore terminal to usual state 

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line,Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};i

fn main() {
    ratatui::run(|terminal| App::default().run(terminal))

}

#[derive(Debug, Default)]
pub struct App { // public struct to maintain app state 
    counter: u8,
    exit: bool,
}

impl App { // impl is used to define behavior of struct, which was declared above 
    // runs applications main loop until the user quits 
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Reslt<()>{
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        todo!()
    }

    fn handle_events(&mut self) -> io::Result<()>{
        todo!()
        }
    }

impl Widget for &App { // pass the address of the app struct  
  fn render(self, area: Rect, buf: &mut Buffer) {
      let title = Line::from(" Counter App Tutorial".bold());
      let instructions = Line::from(vec![  // creates a line to be stylized? 
        " Decrement ".into(),  // vectors are arrays that are immutable if i remember from c++??
        "<Left>".blue().bold(),
        " Increment".into(),
        "<Right>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
      ]);

      let block = Block::bordered()
          .title(title.centered())
          .title_bottom(instructions.centered())
          .border_set(border::THICK);

      let counter_text = Text::from(vec![Line::from(vec![
              "Value: ".into(),
              self.counter(to_string().yellow(),
      ])]);

      Paragraph::new(counter_text)
      .centered()
      .block(block)
      .render(area,buf);
      }
}






