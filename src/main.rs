use std::env;
use std::io;
use std::process;

use crossterm::{
    cursor,
    execute,
    queue,
    style,
    style::{Color},
    event::{read, Event, KeyCode}, terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    terminal::{size, self, ClearType}
};
use std::io::{stdout};

#[allow(non_snake_case)]
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut fg:Color = Color::White;
    let mut bg:Color = Color::Black;
    let mut hl:Color = Color::Green;
    let mut max = 0;
    let mut header:String = "Welcome to Lister!".to_string();
    let mut footer:String = "Arrow keys to move. Spacebar to select. Enter to submit.".to_string();
    let mut rawList:String = "".to_string();
    
    let mut flagger:bool = false;
    if args.len() == 1 {
        usage(None);
        process::exit(0x0);
    } else {
        for (i, arg) in args.iter().enumerate() {
            if flagger == true {
                match args[i-1].as_str() {
                    "--foreground"  => fg = colorChange(arg.to_string()),
                    "--background"  => bg = colorChange(arg.to_string()),
                    "--highlight"   => hl = colorChange(arg.to_string()),
                    "--max"         => max = arg.parse::<i32>().unwrap(),
                    "--header"      => header = arg.to_string(),
                    "--footer"      => footer = arg.to_string(),
                    _               => usage(Some(String::from("Unrecognized command flag")))
                }
                flagger = false;
            } else if i == 0 {
                continue;
            } else if arg.starts_with("--") {
                    flagger = true;
            } else if arg.contains(",") {
                println!("List: {arg}");
                rawList = arg.to_string();
            } else {
                usage(Some(String::from("Malformed parameter detected.")))
            }
        }  
    }

    let listSplit = rawList.split(",");

    let mut listIn:Vec<String> = Vec::new();
    for listItem in listSplit {
        listIn.push(listItem.trim().to_string());
    }


    lister(&listIn, fg, bg, hl, max, header, footer)?;
    
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}

#[allow(non_snake_case)]
fn lister(list:&Vec<String>, fg:Color, bg:Color, hl:Color, max:i32, header:String, footer:String) -> io::Result<(Vec<String>)> {
    let out:Vec<String> = Vec::new();
    //Prepwork
    let listLen = list.len();
    let looper:bool = true;
    let mut toggled:Vec<bool> = Vec::new();
    for _it in list {
        toggled.push(false);
    }
    let mut caret:usize= 0;
    let workspace = size().unwrap();
    let rows:u16 = workspace.1;
    let workHeight = rows - 4;
    let offset:usize = 0;

    let _handle = execute!(stdout(), EnterAlternateScreen);
    while looper {
        //Pre-input
        queue!(
            stdout(),
            terminal::Clear(ClearType::All),
            style::SetForegroundColor(fg),
            style::SetBackgroundColor(bg),
            cursor::Hide,
            cursor::MoveTo(0, 0),
            style::Print(header.to_string()),
            cursor::MoveToNextLine(1),
            style::Print("██████████████████████████"),
            cursor::MoveToNextLine(1)
        )?;
        let caretActual = caret + offset;
        for i in 1..workHeight {
            let iSized:usize = (i - 1).into();
            let targetIndex:usize = iSized + offset;
            if (targetIndex < listLen) {
                if iSized == caretActual {
                    queue!(stdout(), style::Print(" > "))?;
                } else {
                    queue!(stdout(), style::Print("   "))?;
                }
                if (toggled[targetIndex] == true) {
                    queue!(stdout(), style::SetForegroundColor(fg))?;
                } else {
                    queue!(stdout(), style::SetForegroundColor(hl))?;
                }
                queue!(stdout(), style::Print(list[targetIndex].to_string()), cursor::MoveToNextLine(1))?;
            } else {
                queue!(stdout(), cursor::MoveToNextLine(1))?;
            }
        }
        queue!(
            stdout(),
            style::Print("██████████████████████████ Rows: "),
            style::Print(rows.to_string()),
            cursor::MoveToNextLine(1),
            style::Print(footer.to_string())
        )?;

        //Input
        let keyIn = read()?;
        println!();
        if  keyIn == Event::Key(KeyCode::Up.into()) {
            caret = (caret - 1).clamp(0, rows.into());
        } else if keyIn == Event::Key(KeyCode::Down.into()) {
            caret = (caret + 1).clamp(0, rows.into());
        } else if keyIn == Event::Key(KeyCode::Char(' ').into()) {
            if toggled[caret] {
                toggled[caret] = false;
            } else {
                toggled[caret] = true;
            }
        } else if keyIn == Event::Key(KeyCode::Enter.into()) {
            break;
        }
    }
    Ok(out,)
}

#[allow(non_snake_case)]
fn colorChange(hue_arg:String) -> Color {

    let mut hue:Color = Color::Black;
    
    match hue_arg.as_str() {
        "DarkGrey" => hue = Color::DarkGrey,
        "Red" => hue = Color::Red,
        "Green" => hue = Color::Green,
        "Yellow" => hue = Color::Yellow,
        "Blue" => hue = Color::Blue,
        "Magenta" => hue = Color::Magenta,
        "Cyan" => hue = Color::Cyan,
        "White" => hue = Color::White,
        "Black" => hue = Color::Black,
        "DarkRed" => hue = Color::DarkRed,
        "DarkGreen" => hue = Color::DarkGreen,
        "DarkYellow" => hue = Color::DarkYellow,
        "DarkBlue" => hue = Color::DarkBlue,
        "DarkMagenta" => hue = Color::DarkMagenta,
        "DarkCyan" => hue = Color::DarkCyan,
        "Grey" => hue = Color::Grey,
        _ => usage(Some(String::from("Invalid color specified.")))
    }
    return hue;
}

fn usage(error: Option<String>) {
    let header = error.unwrap_or(String::from("Lister - Allow a user to select one or more items from a list"));
    println!("
{header}

Usage:
lister [flags] \"[list items]\"

Color flags:
    --foreground [Foreground Color]
        Default: White
    --background [Background Color]
        Default: Black
    --highlight [Highlight Color]
        Default: Green
    Available colors:
        DarkGrey	Black
        Red	        DarkRed
        Green	    DarkGreen
        Yellow	    DarkYellow
        Blue	    DarkBlue
        Magenta	    DarkMagenta
        Cyan	    DarkCyan
        White	    Grey
Technical flags:
    --header [Header text]
    --footer [Footer text]
    --max [Limit]
        Limit the number of options that can be highlighted by the user. 
        If '0' is specified, no limit is imposed.
        Default: 0

List items should be comma separated. White space between a comma and the next item will be removed automatically.

Example:
    lister --foreground Gray --highlight Blue --max 3 \"One, Two, Three, Four, Five, Six\"
");
}