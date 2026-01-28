use time::{Date, OffsetDateTime};

struct User {
    id:         i64,
    name:       String,
    created_at: Date,
    active:     bool,
}

impl User {
    fn new(id: i64, name: String, created_at: Date, active: bool) -> Self {
        Self {id, name, created_at, active}
    }

    // fn store() -> Row {
    //
    // }
}

fn store_and_retrieve_object() {
    let new_user = User::new(1, "Stan Swanborn".into(), OffsetDateTime::now_utc().date(), true);
}