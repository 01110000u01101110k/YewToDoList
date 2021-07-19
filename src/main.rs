use yew::prelude::*;
use yew::{InputData};
use yew::services::{ConsoleService, DialogService};
use web_sys::window;
use web_sys::Storage;
use yew::format::Json;

const NOTES_STORAGE: &str = "notes";

struct NoteData {
    note_text: String,
    is_done: bool
}

enum Msg {
    HandleChangeInputValue(String),
    AddNewNote,
    DeleteNote(usize),
    MarkNoteAsDone(usize)
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: String,
    notes: Vec<NoteData>,
    storage: String
}

fn set_local_storage_data(value: String, is_done: String) {
    // пока придумал такой костыльный метод, заполнения local_storage (временное решение)
    let set_local_storage = window().unwrap().local_storage().unwrap().unwrap();
    let note_text = String::from(value.to_string());
    let str_for_local_storage = "{ note_text: ".to_string() + &note_text + &", is_done: " + &is_done +  &"}";
    set_local_storage.set_item(NOTES_STORAGE, &str_for_local_storage).unwrap();
}

fn get_data_from_local_storage() -> String {
    let get_local_storage = window().unwrap().local_storage().unwrap().unwrap();
    let get_notes_from_local_storage_data = get_local_storage.get_item(NOTES_STORAGE).unwrap();
    let mut storage: String = "".to_string();
    match get_notes_from_local_storage_data {
        Some(data) => {
            storage = String::from(data);
        },
        None => println!("some went wrong"),
    }
    return storage;
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = get_data_from_local_storage();
        Self {
            link,
            value: String::from(""),
            notes: vec![],
            storage: storage
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HandleChangeInputValue(value) => {
                self.value = value;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            },
            Msg::AddNewNote => {
                if self.value.to_string() != ""{
                    let note_data = NoteData {
                        note_text: String::from(self.value.to_string()),
                        is_done: false
                    };
                    self.notes.push(note_data);

                    set_local_storage_data(self.value.to_string(), "false".to_string());
                    self.storage = get_data_from_local_storage();
                } else {
                    DialogService::alert("note is empty");
                }
                true
            },
            Msg::DeleteNote(index) => {
                self.notes.remove(index);
                true
            },
            Msg::MarkNoteAsDone(index) => {
                self.notes[index].is_done = !self.notes[index].is_done;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let mut count_done_notes:usize = 0;

        for note in self.notes.iter(){
            if note.is_done{
                count_done_notes += 1;
            }
        }

        html! {
            <>
                <header>
                    <nav>
                        <p class="logo">
                            {"Notes"}
                        </p>
                    </nav>
                </header>
                <main>
                    <div class="container">
                        <div class="inputSection">
                            <input 
                            type="text" 
                            class="input" 
                            placeholder="write note"
                            oninput=self.link.callback(|elem: InputData|Msg::HandleChangeInputValue(elem.value)) />
                            <button onclick=self.link.callback(|_| Msg::AddNewNote) class="btn">{"create note"}
                            </button>
                        </div>
                        <div><p>{count_done_notes}{" / "}{self.notes.len()}</p></div>
                        <div class="notesSection">
                            {for (0..self.notes.len()).into_iter().map(|note| {
                                html! {
                                    <>
                                        <div class="note">
                                            <div>
                                                <p class="indent-horizontal-15">{self.notes[note].note_text.to_string()}</p>
                                            </div>
                                            <div>
                                                <button class={if self.notes[note].is_done {"btn-done"} else {"btn-not-done"}} onclick=self.link.callback(move|_| Msg::MarkNoteAsDone(note))>
                                                {if self.notes[note].is_done{
                                                    "done"
                                                } else {
                                                    "not done"
                                                }}
                                                </button>
                                                <button class="btn-delete" onclick=self.link.callback(move|_| Msg::DeleteNote(note))>{"delete"}</button>
                                            </div>
                                        </div>
                                    </>
                                }
                            })}
                        </div>
                        <p>{"last data from local storage: "}{&self.storage}</p>
                    </div>
                </main>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}