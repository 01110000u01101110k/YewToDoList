use yew::prelude::*;
use yew::services::{ConsoleService, DialogService};

enum Msg {
    HandleChangeInputValue(String),
    AddNewNote,
    MarkNoteAsDone(String)
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: String,
    notes: Vec<String>,
    done_notes: Vec<String>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: String::from(""),
            notes: vec![],
            done_notes: vec![]
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
                    self.notes.push(self.value.to_string());
                } else {
                    DialogService::alert("note is empty");
                }
                true
            },
            Msg::MarkNoteAsDone(value) => {
                ConsoleService::log(&value.to_string());
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
                            <p>{self.done_notes.len()}{" / "}{self.notes.len()}</p>
                        </div>
                        <div class="notesSection">
                            {for (0..self.notes.len()).into_iter().map(|note| {
                                html! {
                                    <>
                                        <div class="note">
                                            {self.notes[note].to_string()}
                                            /*<input type="checkbox" onclick=self.link.callback(|_| Msg::MarkNoteAsDone(self.notes[note])) />*/
                                        </div>
                                        
                                    </>
                                }
                            }) }
                        </div>
                    </div>
                </main>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}