table! {
    blacklist (iban) {
        iban -> Text,
        blacklisted -> Bool,
    }
}

table! {
    t_at (code) {
        id -> Integer,
        code -> Text,
        name -> Text,
        zip -> Integer,
        city -> Text,
        bic -> Nullable<Text>,
    }
}

table! {
    t_be (id) {
        id -> Text,
        name -> Text,
        bic -> Text,
    }
}

table! {
    t_de (id) {
        id -> Integer,
        code -> Text,
        name -> Text,
        zip -> Integer,
        city -> Text,
        bic -> Nullable<Text>,
    }
}

table! {
    t_nl (code) {
        code -> Text,
        name -> Text,
        bic -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    blacklist,
    t_at,
    t_be,
    t_de,
    t_nl,
);
