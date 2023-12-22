/*!
    A very simple application that show your name in a message box.
    See `basic` for the version without the derive macro
*/
extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct CreateNoteGui {
    #[nwg_control(size: (300, 270), position: (300, 300), title: "Create a note", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [CreateNoteGui::exit] )]
    window: nwg::Window,

    #[nwg_control(text: "Title", size: (280, 35), position: (10, 10), focus: true)]
    title_edit: nwg::TextInput,

    #[nwg_control(text: "Description", size: (280, 35), position: (10, 55), focus: true)]
    description_edit: nwg::TextInput,

    #[nwg_control(text: "Create the new note", size: (280, 70), position: (10, 100))]
    #[nwg_events( OnButtonClick: [CreateNoteGui::create_new_note] )]
    hello_button: nwg::Button,
}

impl CreateNoteGui {
    fn create_new_note(&self) {
        nwg::modal_info_message(&self.window, "New note created!", &format!("Your note has been created!"));
    }
    
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

pub fn run_create_note_gui() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = CreateNoteGui::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
