use cursive::theme::{BorderStyle, Color, PaletteColor};
use cursive::traits::Scrollable;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, EditView, LinearLayout, SelectView};
use cursive::Cursive;
use static_init::dynamic;
use std::io::{self, BufRead, Write};

#[dynamic(lazy)]
static mut INPUT_LINES: Vec<String> = Vec::new();

fn print_result(ctx: &mut Cursive, result: &str) {
    ctx.quit();
    io::stdout().write_all(result.as_bytes()).unwrap();
}

fn on_submit(ctx: &mut Cursive, query: &str) {
    let matches = ctx.find_name::<SelectView>("matches").unwrap();
    if matches.is_empty() {
        print_result(ctx, query);
    } else {
        let matched = &*matches.selection().unwrap();
        print_result(ctx, matched);
    };
}

fn on_edit(ctx: &mut Cursive, query: &str, _cursor: usize) {
    let matches = search_fn(&INPUT_LINES.read(), query);
    ctx.call_on_name("matches", |v: &mut SelectView| {
        v.clear();
        v.add_all_str(matches);
    });
}

fn search_fn<'a>(items: &'a Vec<String>, query: &'a str) -> Vec<String> {
    let result: Vec<String> = items
        .into_iter()
        .filter_map(|item| {
            let item = item.to_lowercase();
            let query = query.to_lowercase();
            if item.contains(&query) {
                Some(item.to_owned())
            } else {
                None
            }
        })
        .collect();
    result
}

fn main() {
    let mut tui = cursive::default();

    tui.update_theme(|theme| {
        theme.shadow = false;
        theme.borders = BorderStyle::None;
        theme.palette.extend(vec![
            (PaletteColor::Background, Color::TerminalDefault),
            (PaletteColor::View, Color::TerminalDefault),
            (PaletteColor::Primary, Color::TerminalDefault),
            (PaletteColor::TitlePrimary, Color::TerminalDefault),
            (PaletteColor::TitleSecondary, Color::TerminalDefault),
        ])
    });

    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        // Read in all lines from stdin
        match stdin.lock().read_line(&mut buffer) {
            Ok(bytes) => {
                // 0 bytes read means stdin is empty (fully read)
                if bytes == 0 {
                    break;
                }
            }

            _ => panic!("Failed to read from STDIN"),
        }
    }

    buffer
        .split("\n")
        .for_each(|line| INPUT_LINES.write().push(line.to_owned()));

    tui.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(
                    EditView::new()
                        .on_edit(on_edit)
                        .on_submit(on_submit)
                        .with_name("query"),
                )
                .child(
                    SelectView::new()
                        .with_all_str(buffer.split("\n"))
                        .on_submit(print_result)
                        .with_name("matches")
                        .scrollable(),
                )
                .full_screen(),
        )
        .button("Quit", Cursive::quit),
    );

    tui.run();
}
