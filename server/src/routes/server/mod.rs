pub mod index;

use index::index as index_handler;
use rocket::{fairing::AdHoc, routes};

pub fn stage() -> AdHoc {
    return AdHoc::on_ignite("Server Routes Stage", |rocket| async {
        rocket
            .mount("/api/v1/server", routes![index_handler])
    });
}
