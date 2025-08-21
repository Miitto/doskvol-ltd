//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;

/// Echo the user input on the server.
#[server(GetCharacter)]
pub async fn get_character(id: usize) -> Result<types::Character, ServerFnError> {
    let character = types::Character {
        id: 0,
        name: "Lord Theodophilus Dalmore".to_string(),
        class: types::Class::Slide,
        look: "Affable, handsome, long scarf, waistcoat".into(),
        abilities: vec![
            "Cloak & Dagger".into(),
            "Like looking into a mirror".into(),
            "A little something on the side".into(),
        ],
        heritage: types::Heritage::Akoros,
        background: types::Background::Noble,
        vice: types::Vice::Luxury,
        coin: 1,
        stash: 10,
        xp: types::XP {
            playbook: 0,
            insight: 0,
            prowess: 0,
            resolve: 0,
        },
        dots: types::Dots {
            hunt: 0,
            study: 0,
            survey: 0,
            tinker: 0,
            finesse: 3,
            prowl: 3,
            skirmish: 1,
            wreck: 0,
            attune: 0,
            command: 0,
            consort: 0,
            sway: 0,
        },
        stress: 3,
        trauma: types::TraumaFlags::empty(),
        harm: types::Harm::default(),
        healing: 1,
        armor: types::ArmorFlags::ARMOR,
        notes: types::Description::new("Nyryx scares me\nhelp".into()),
        contacts: types::Contacts {
            friends: vec!["Nyryx, a prostitute".to_string()],
            rivals: vec!["Bazso Baz, a gang leader".to_string()],
        },
        class_items: vec![],
        load: None,
        items: vec![],
    };

    Ok(character)
}
