fn reedline_testing() -> Result<(), miette::ErrReport> {
    let commands = vec![
        "test".into(),
        "hello world".into(),
        "hello world reedline".into(),
        "this is the reedline crate".into(),
    ];
    let completer =
        Box::new(DefaultCompleter::new_with_wordlen(commands.clone(), 2));
    // Use the interactive menu to select options from the completer
    let completion_menu =
        Box::new(ColumnarMenu::default().with_name("completion_menu"));
    let mut line_editor = Reedline::create()
        .with_highlighter(Box::new(ExampleHighlighter::new(commands)))
        .with_completer(completer)
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu));
    let prompt = DefaultPrompt::default();
    loop {
        let sig = line_editor.read_line(&prompt).unwrap();
        match sig {
            Signal::Success(buffer) => {
                println!("We processed: {}", buffer);
            }
            Signal::CtrlD | Signal::CtrlC => {
                println!("\nAborted!");
                break Ok(());
            }
        }
    }
}

// pub struct BalsapopPrompt;

// impl Prompt for BalsapopPrompt {
//     fn render_prompt_left(&self) -> std::borrow::Cow<str> {
//         todo!()
//     }

//     fn render_prompt_right(&self) -> std::borrow::Cow<str> {
//         todo!()
//     }

//     fn render_prompt_indicator(
//         &self,
//         prompt_mode: reedline::PromptEditMode,
//     ) -> std::borrow::Cow<str> {
//         todo!()
//     }

//     fn render_prompt_multiline_indicator(&self) -> std::borrow::Cow<str> {
//         todo!()
//     }

//     fn render_prompt_history_search_indicator(
//         &self,
//         history_search: reedline::PromptHistorySearch,
//     ) -> std::borrow::Cow<str> {
//         todo!()
//     }
// }
