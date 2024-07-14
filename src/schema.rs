// @generated automatically by Diesel CLI.

diesel::table! {
    camisas (id) {
        id -> Integer,
        nome -> Text,
        data_fabricacao -> Date,
    }
}

diesel::table! {
    calcas (id) {
        id -> Integer,
        is_visually_malnourished -> Bool,
        has_gastro_intestinal_inconsistency -> Bool,
        has_malnutrition_patology -> Bool,
        camisa_id -> Integer,
    }
}

diesel::joinable!(calcas -> camisas (camisa_id));

diesel::allow_tables_to_appear_in_same_query!(
    camisas,
    calcas,
);
