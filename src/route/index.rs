use crate::controller;
use crate::error::OneBlogError;
use crate::utils;
use actix_web::web;

// GET /admin
pub async fn index(
    per_page: Option<web::Query<usize>>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> impl actix_web::Responder {
    let per_page = per_page.map(|inner| inner.into_inner()).unwrap_or(3);
    let counts = controller::post::count(&db).await?;
    let pages = utils::paginate(
        counts as usize,
        per_page,
        1,
        Some("<".to_string()),
        Some(">".to_string()),
    );
    let posts = controller::post::offset_and_limit(&db, 0, per_page as u64).await?;
    let categories = controller::category::posts_count(&db).await?;
    let html = hbs.render(
        "index",
        &serde_json::json!({
            "posts": posts,
            "pages": pages,
            "meta": serde_json::json!({"categories": categories}),
        }),
    )?;

    OneBlogError::ok(utils::html(html))
}

pub async fn posts(
    page: web::Path<i32>,
    per_page: Option<web::Query<usize>>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> impl actix_web::Responder {
    let per_page = per_page.map(|inner| inner.into_inner()).unwrap_or(3);
    let page = page.into_inner() as usize;
    let counts = controller::post::count(&db).await?;
    let pages = utils::paginate(
        counts as usize,
        per_page,
        page,
        Some("<".to_string()),
        Some(">".to_string()),
    );
    let posts =
        controller::post::offset_and_limit(&db, ((page - 1) * per_page) as u64, per_page as u64)
            .await?;
    let categories = controller::category::posts_count(&db).await?;
    let html = hbs.render(
        "index",
        &serde_json::json!({
            "posts": posts,
            "pages": pages,
            "categories": categories
        }),
    )?;

    OneBlogError::ok(utils::html(html))
}

pub async fn posts_with_category(
    path: web::Path<(i32, i32)>,
    per_page: Option<web::Query<usize>>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> impl actix_web::Responder {
    let (category_id, page_number) = path.into_inner();
    let per_page = per_page.map(|inner| inner.into_inner()).unwrap_or(3);
    let db: &sea_orm::DatabaseConnection = &db;
    let count = controller::category::find_posts_count(db, category_id).await?;
    let (category, posts) = controller::category::find_posts_with(
        db,
        category_id,
        Some((page_number as u64 - 1) * per_page as u64),
        Some(per_page as u64),
    )
    .await?
    .unzip();
    let pages = utils::paginate(
        count as usize,
        per_page,
        page_number as usize,
        Some("<".to_string()),
        Some(">".to_string()),
    );
    let categories = controller::category::posts_count(db).await?;
    let html = hbs.render(
        "posts_with_category",
        &serde_json::json!(
            {
                "categories": categories,
                "cur_category": category,
                "pages": pages,
                "posts": posts
            }
        ),
    )?;

    OneBlogError::ok(utils::html(html))
}

pub async fn post_id(
    post_id: web::Path<i32>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> impl actix_web::Responder {
    let post = controller::post::find(&db, *post_id).await?;
    let categories = controller::category::posts_count(&db).await?;
    let html = hbs.render(
        "post_id",
        &serde_json::json!(
        {
            "categories": categories,
            "post": post,
        }),
    )?;

    OneBlogError::ok(utils::html(html))
}

#[derive(serde::Deserialize)]
pub struct SearchFormData {
    pattern: String,
}

pub async fn search(
    form: web::Form<SearchFormData>,
    per_page: Option<web::Query<u64>>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> impl actix_web::Responder {
    search_with_page(form, 1.into(), per_page, db, hbs).await
}

pub async fn search_with_page(
    web::Form(form): web::Form<SearchFormData>,
    page_number: web::Path<i32>,
    per_page: Option<web::Query<u64>>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> impl actix_web::Responder {
    let page_number = page_number.into_inner();
    let per_page = per_page.map(web::Query::into_inner).unwrap_or(3);
    // Query database
    let (
        mut searched,
        sea_orm::ItemsAndPagesNumber {
            number_of_items,
            number_of_pages: _,
        },
    ) = controller::post::search(
        &db,
        form.pattern.clone(),
        page_number as u64,
        per_page as i32,
    )
    .await?;
    // Replace with <mark> tag
    searched.iter_mut().for_each(|post| {
        if let Some(index) = post.title.to_lowercase().find(&form.pattern) {
            let substr = &post.title[index..index + form.pattern.len()];
            post.title.replace_range(
                index..index + form.pattern.len(),
                &format!("<mark>{}</mark>", substr),
            );
        }
    });
    // Pagination
    let pages = utils::paginate(
        number_of_items as usize,
        per_page as usize,
        page_number as usize,
        Some("<".to_string()),
        Some(">".to_string()),
    );
    // Render html
    let categories = controller::category::posts_count(&db).await?;
    let json = &serde_json::json!(
    {
        "searched": searched,
        "pattern": form.pattern,
        "pages": pages,
        "meta": serde_json::json!({"categories": categories}),
    });
    let html = hbs.render("searched", json)?;

    OneBlogError::ok(utils::html(html))
}
