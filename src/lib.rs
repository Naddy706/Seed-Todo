use seed::{*, prelude::*};

struct Model {
    id: String,
    name:String,
    Message: String
}

#[derive(Clone)]
enum Msg{

    Add,
    id(String),
    name(String),
    Message(String),

}



impl Default for Model {
    fn default() -> Self {
        Self {
            id: "".into(),
            name:"".into(),
            Message: "".into()
        }
    }
}



fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    // match msg {
    //    // Msg::Add => model.id = id
    // }
}

/// The top-level component we pass to the virtual dom.
fn view(model: &Model) -> impl View<Msg> {
     div![
         class!["container","jumbotron"],
     
        h3![ "MyForm" ],
       h3![ format!("{} {}{} ", model.id, model.name, model.Message) ],
        input![ class!["form-control"], input_ev(Ev::Input, Msg::id) ],
        input![ class!["form-control"], input_ev(Ev::Input, Msg::name) ],
        input![ class!["form-control"], input_ev(Ev::Input, Msg::Message) ],
        button![ class!["btn","btn-primary"],simple_ev(Ev::Click, Msg::Add), "Save" ],
       
     
        ]
       // success_level(model.count),
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .build_and_start();
}

