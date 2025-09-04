// @generated automatically by Diesel CLI.

diesel::table! {
    character_abilities (id) {
        id -> Integer,
        character_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    character_class_items (id) {
        id -> Integer,
        character_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    character_contacts (id) {
        id -> Integer,
        character_id -> Integer,
        name -> Text,
        friend -> Bool,
    }
}

diesel::table! {
    character_dots (character_id) {
        character_id -> Integer,
        hunt -> Integer,
        study -> Integer,
        survey -> Integer,
        tinker -> Integer,
        finesse -> Integer,
        prowl -> Integer,
        skirmish -> Integer,
        wreck -> Integer,
        attune -> Integer,
        command -> Integer,
        consort -> Integer,
        sway -> Integer,
    }
}

diesel::table! {
    character_harm (character_id) {
        character_id -> Integer,
        harm_1_1 -> Text,
        harm_1_2 -> Text,
        harm_2_1 -> Text,
        harm_2_2 -> Text,
        harm_3 -> Text,
    }
}

diesel::table! {
    character_xp (character_id) {
        character_id -> Integer,
        playbook -> Integer,
        insight -> Integer,
        prowess -> Integer,
        resolve -> Integer,
    }
}

diesel::table! {
    characters (id) {
        id -> Integer,
        user_id -> Text,
        crew_id -> Integer,
        name -> Text,
        look -> Text,
        heritage -> Text,
        background -> Text,
        vice -> Text,
        stress -> Integer,
        trauma -> Integer,
        healing -> Integer,
        armor -> Integer,
        notes -> Text,
        class -> Text,
        stash -> Integer,
        coin -> Integer,
        load -> Nullable<Integer>,
        items -> Integer,
    }
}

diesel::table! {
    crew_invites (code) {
        code -> Text,
        crew_id -> Integer,
        used -> Integer,
        max_uses -> Integer,
    }
}

diesel::table! {
    crew_members (user_id, crew_id) {
        user_id -> Text,
        crew_id -> Integer,
        display_name -> Text,
    }
}

diesel::table! {
    crews (id) {
        id -> Integer,
        name -> Text,
        specialty -> Text,
        dm_id -> Text,
    }
}

diesel::table! {
    sessions (id) {
        id -> Integer,
        token -> Text,
        user_id -> Text,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    users (username) {
        username -> Text,
        totp_secret -> Text,
    }
}

diesel::joinable!(character_abilities -> characters (character_id));
diesel::joinable!(character_class_items -> characters (character_id));
diesel::joinable!(character_contacts -> characters (character_id));
diesel::joinable!(character_dots -> characters (character_id));
diesel::joinable!(character_harm -> characters (character_id));
diesel::joinable!(character_xp -> characters (character_id));
diesel::joinable!(characters -> crews (crew_id));
diesel::joinable!(characters -> users (user_id));
diesel::joinable!(crew_invites -> crews (crew_id));
diesel::joinable!(crew_members -> crews (crew_id));
diesel::joinable!(crew_members -> users (user_id));
diesel::joinable!(crews -> users (dm_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    character_abilities,
    character_class_items,
    character_contacts,
    character_dots,
    character_harm,
    character_xp,
    characters,
    crew_invites,
    crew_members,
    crews,
    sessions,
    users,
);
