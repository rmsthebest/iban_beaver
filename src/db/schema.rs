table! {
    blacklist (iban) {
        iban -> Text,
        blacklisted -> Bool,
    }
}

table! {
    t_at (code) {
        id -> Integer,
        code -> Integer,
        name -> Text,
        zip -> Integer,
        city -> Text,
        bic -> Nullable<Text>,
    }
}

table! {
    t_de (id) {
        id -> Integer,
        code -> Integer,
        name -> Text,
        zip -> Integer,
        city -> Text,
        bic -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    blacklist,
    t_at,
    t_de,
);
