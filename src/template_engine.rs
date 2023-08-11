use handlebars::handlebars_helper;
use sea_orm::prelude::DateTimeUtc;
use serde_json::Value as Json;

pub fn init() -> handlebars::Handlebars<'static> {
    let mut handlebars = handlebars::Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./src/view/")
        .unwrap();
    handlebars_helper!(time: |t: DateTimeUtc| t.format("%Y/%m/%d/ %H:%M").to_string());
    handlebars.register_helper("time", Box::new(time));
    handlebars
}
