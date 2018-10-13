#![recursion_limit = "1024"]

extern crate stdweb;
#[macro_use]
extern crate yew;
extern crate unit_splitter_core as core;

use core::inventory::{self, InventoryParseResult};
use core::requests::{self, RequestsParseResult};
use core::split::{self, Split, SplitResult};
use core::group::Group;
use yew::prelude::*;

const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn get_version() -> &'static str {
    VERSION.unwrap_or("unknown")
}

pub struct Model {
    inventory_string: String,
    requests_string: String,
    inventory: InventoryParseResult,
    requests: RequestsParseResult,
    split: SplitResult,
}

pub enum Msg {
    GotInventoryString(String),
    GotRequestString(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            inventory_string: "".into(),
            requests_string: "".into(),
            inventory: inventory::parse(""),
            requests: requests::parse(""),
            split: Ok(core::split::Split {
                filled_requests: std::collections::HashMap::new(),
                leftover_ranges: Vec::new(),
            }),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInventoryString(value) => {
                self.inventory_string = value;
                self.inventory = inventory::parse(&self.inventory_string);
            }
            Msg::GotRequestString(value) => {
                self.requests_string = value;
                self.requests = requests::parse(&self.requests_string);
            }
        }
        match (&self.inventory, &self.requests) {
            (Ok(inventory), Ok(requests)) => {
                self.split = split::split(&inventory, &requests);
            }
            _ => {
                // TODO: Make it apparent when output and input are desynchronized?
            }
        };
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="body",>
                <div>
                    <h1>{ "Units" }</h1>
                    <textarea class=("indent", "inventory-input"),
                        value=&self.inventory_string,
                        oninput=|e| Msg::GotInventoryString(e.value),
                        placeholder="enter inventory",>
                    </textarea>
                </div>
                <div>
                    <h1>{ "Requests" }</h1>
                    <textarea class=("indent", "requests-input"),
                        value=&self.requests_string,
                        oninput=|e| Msg::GotRequestString(e.value),
                        placeholder="enter requests",>
                    </textarea>
                </div>
                { self.view_output() }
            </div>
            <hr/>
            <div class="footer",>
                { PKG_NAME }{" "}<a href="changelog.html",>{"v"}{ get_version() }</a>
                {" by "}{ AUTHORS }
            </div>
        }
    }
}

impl Model {
    fn view_output(&self) -> Html<Model> {
        match &self.split {
            Ok(Split {
                filled_requests,
                leftover_ranges: _,
            }) => html! {
                <div>
                    <h1>{ "Output" }</h1>
                    <div class="indent",>
                        <div>{ for filled_requests.iter().map(view_filled_request) }</div>
                        <div>{ "Unused" }</div>
                    </div>
                </div>
            },
            Err(e) => html! {
                <div>
                    <h1>{ "Output" }</h1>
                    <div class="indent",>
                        { format!("{}", e) }
                    </div>
                </div>
            },
        }
    }

}

fn view_filled_request((request_name, groups): (&String, &Vec<Group>)) -> Html<Model> {
    html! {
        <div>
            <h1>{ "Output" }</h1>
            <div class="indent",>
                { format!("{}, {:?}", request_name, groups) }
            </div>
        </div>
    }
}
