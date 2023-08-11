use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;

// Return an opaque 500 while preserving the error root's cause for logging.
pub fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

// Return a 400 with the user-representation of the validation error as body.
// The error root cause is preserved for logging purposes.
pub fn e400<T: std::fmt::Debug + std::fmt::Display>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorBadRequest(e)
}

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

#[derive(Debug, serde::Serialize)]
pub struct Page {
    page: i32,
    display: String, // '<', '>', 1, 2, ....100 etc
    active: bool,
    disabled: bool,
}

// 1-indexed
pub fn paginate(
    mut total: usize,
    per_page: usize,
    active: usize,
    previous: Option<String>,
    next: Option<String>,
) -> Vec<Page> {
    if total == 0 {
        return vec![];
    }
    let total_pages = (total + per_page - 1) / per_page;
    if total_pages < active {
        return vec![];
    }
    let mut pages = (1..=total_pages)
        .into_iter()
        .map(|index| Page {
            page: index as i32,
            display: index.to_string(),
            active: false,
            disabled: false,
        })
        .collect::<Vec<_>>();

    pages[active - 1].active = true;

    // add Previous
    if let Some(symbol) = previous {
        pages.insert(
            0,
            Page {
                page: if active == 1 { -1 } else { active as i32 - 1 },
                display: symbol,
                active: false,
                disabled: active == 1,
            },
        );
    }

    // add Next
    if let Some(symbol) = next {
        pages.push(Page {
            page: if active == total_pages {
                -1
            } else {
                active as i32 + 1
            },
            display: symbol,
            active: false,
            disabled: active == total_pages,
        });
    }

    pages
}
