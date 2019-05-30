table! {
    dates (id) {
        id -> Integer,
        date -> Text,
    }
}

table! {
    regs (id) {
        id -> Integer,
        date -> Text,
        user -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    dates,
    regs,
    users,
);
