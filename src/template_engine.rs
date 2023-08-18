use crate::error::OneBlogError;
use handlebars::handlebars_helper;
use sea_orm::prelude::DateTimeUtc;

pub fn init() -> Result<handlebars::Handlebars<'static>, OneBlogError> {
    let mut handlebars = handlebars::Handlebars::new();
    handlebars.register_templates_directory(".html", "./src/view/")?;
    // time helper
    handlebars_helper!(time: |t: DateTimeUtc| t.format("%Y/%m/%d/ %H:%M").to_string());
    handlebars.register_helper("time", Box::new(time));

    Ok(handlebars)
}
