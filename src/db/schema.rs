table! {
    t_de (id) {
        id -> Integer,
        code -> Integer,
        name -> Text,
        zip -> Integer,
        city -> Text,
        bic -> Nullable<Text>,
        blacklisted -> Bool,
    }
}
