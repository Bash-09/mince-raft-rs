pub struct State {
    pub mouse_grabbed: bool,
    pub mouse_visible: bool,

    pub options_visible: bool,
}

impl State {
    pub fn new() -> State {
        State {
            mouse_grabbed: false,
            mouse_visible: true,

            options_visible: false,
        }
    }
}
