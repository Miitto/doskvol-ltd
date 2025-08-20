data::blades!();

#[derive(Debug, Clone, PartialEq)]
pub struct Character {
    pub name: String,
    pub class: Class,
    pub look: String,
    pub heritage: Heritage,
    pub background: Background,
    pub vice: Vice,
    pub abilities: Vec<String>,
    pub stash: u8,
    pub stress: u8,
    pub trauma: u8,
    pub traumas: Trauma,
    pub coin: u8,
    pub xp: XP,
    pub dots: Dots,
}

#[derive(Debug, Clone, PartialEq)]
pub struct XP {
    pub playbook: u8,
    pub insight: u8,
    pub prowess: u8,
    pub resolve: u8,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Dots {
    pub hunt: u8,
    pub study: u8,
    pub survey: u8,
    pub tinker: u8,
    pub finesse: u8,
    pub prowl: u8,
    pub skirmish: u8,
    pub wreck: u8,
    pub attune: u8,
    pub command: u8,
    pub consort: u8,
    pub sway: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Heritage {
    Akoros,
    TheDaggerIsles,
    Iruvia,
    Severos,
    Skovlan,
    Tycsheros,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Background {
    Academic,
    Labor,
    Law,
    Trade,
    Military,
    Noble,
    Underworld,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Vice {
    Faith,
    Gambling,
    Luxury,
    Obligation,
    Pleasure,
    Stupor,
    Weird,
}

bitflags::bitflags! {
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Trauma: u8 {
    const COLD = 0b00000001;
    const HAUNTED = 0b00000010;
    const OBSESSED = 0b00000100;
    const PARANOID = 0b00001000;
    const RELENTLESS = 0b00010000;
    const SOFT = 0b00100000;
    const UNSTABLE = 0b01000000;
    const VICIOUS = 0b10000000;
}
}

impl Heritage {
    pub const ALL: [Heritage; 6] = [
        Heritage::Akoros,
        Heritage::TheDaggerIsles,
        Heritage::Iruvia,
        Heritage::Severos,
        Heritage::Skovlan,
        Heritage::Tycsheros,
    ];
}

impl std::fmt::Display for Heritage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heritage::Akoros => write!(f, "Akoros"),
            Heritage::TheDaggerIsles => write!(f, "The Dagger Isles"),
            Heritage::Iruvia => write!(f, "Iruvia"),
            Heritage::Severos => write!(f, "Severos"),
            Heritage::Skovlan => write!(f, "Skovlan"),
            Heritage::Tycsheros => write!(f, "Tycheros"),
        }
    }
}

impl From<String> for Heritage {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Akoros" => Heritage::Akoros,
            "The Dagger Isles" => Heritage::TheDaggerIsles,
            "Iruvia" => Heritage::Iruvia,
            "Severos" => Heritage::Severos,
            "Skovlan" => Heritage::Skovlan,
            "Tycheros" => Heritage::Tycsheros,
            _ => panic!("Unknown heritage: {s}"),
        }
    }
}

impl Background {
    pub const ALL: [Background; 7] = [
        Background::Academic,
        Background::Labor,
        Background::Law,
        Background::Trade,
        Background::Military,
        Background::Noble,
        Background::Underworld,
    ];
}

impl std::fmt::Display for Background {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Background::Academic => write!(f, "Academic"),
            Background::Labor => write!(f, "Labor"),
            Background::Law => write!(f, "Law"),
            Background::Trade => write!(f, "Trade"),
            Background::Military => write!(f, "Military"),
            Background::Noble => write!(f, "Noble"),
            Background::Underworld => write!(f, "Underworld"),
        }
    }
}

impl From<String> for Background {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Academic" => Background::Academic,
            "Labor" => Background::Labor,
            "Law" => Background::Law,
            "Trade" => Background::Trade,
            "Military" => Background::Military,
            "Noble" => Background::Noble,
            "Underworld" => Background::Underworld,
            _ => panic!("Unknown background: {s}"),
        }
    }
}

impl Vice {
    pub const ALL: [Vice; 7] = [
        Vice::Faith,
        Vice::Gambling,
        Vice::Luxury,
        Vice::Obligation,
        Vice::Pleasure,
        Vice::Stupor,
        Vice::Weird,
    ];
}

impl std::fmt::Display for Vice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vice::Faith => write!(f, "Faith"),
            Vice::Gambling => write!(f, "Gambling"),
            Vice::Luxury => write!(f, "Luxury"),
            Vice::Obligation => write!(f, "Obligation"),
            Vice::Pleasure => write!(f, "Pleasure"),
            Vice::Stupor => write!(f, "Stupor"),
            Vice::Weird => write!(f, "Weird"),
        }
    }
}

impl From<String> for Vice {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Faith" => Vice::Faith,
            "Gambling" => Vice::Gambling,
            "Luxury" => Vice::Luxury,
            "Obligation" => Vice::Obligation,
            "Pleasure" => Vice::Pleasure,
            "Stupor" => Vice::Stupor,
            "Weird" => Vice::Weird,
            _ => panic!("Unknown vice: {s}"),
        }
    }
}

