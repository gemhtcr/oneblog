use crate::error::OneBlogError;
use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};
use actix_web_flash_messages::FlashMessage;

pub async fn logout(session: TypedSession) -> impl actix_web::Responder {
    if session.get_user_id().map_err(e500)?.is_none() {
        OneBlogError::ok(see_other("/login"))
    } else {
        session.log_out();
        FlashMessage::info("You have successfully logged out.").send();
        OneBlogError::ok(see_other("/login"))
    }
}
