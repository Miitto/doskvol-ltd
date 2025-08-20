use crate::description::Description;

data::blades!();

mod bitflag_count;
pub use bitflag_count::BitCount;
pub mod description;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character {
    pub id: usize,
    pub name: String,
    pub class: Class,
    pub look: Description<String>,
    pub heritage: Heritage,
    pub background: Background,
    pub vice: Vice,
    pub abilities: Vec<String>,
    pub stash: u8,
    pub stress: u8,
    pub trauma: TraumaFlags,
    pub coin: u8,
    pub xp: XP,
    pub dots: Dots,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct XP {
    pub playbook: u8,
    pub insight: u8,
    pub prowess: u8,
    pub resolve: u8,
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Heritage {
    Akoros,
    TheDaggerIsles,
    Iruvia,
    Severos,
    Skovlan,
    Tycsheros,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Background {
    Academic,
    Labor,
    Law,
    Trade,
    Military,
    Noble,
    Underworld,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Vice {
    Faith,
    Gambling,
    Luxury,
    Obligation,
    Pleasure,
    Stupor,
    Weird,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Trauma {
    Cold,
    Haunted,
    Obsessed,
    Paranoid,
    Relentless,
    Soft,
    Unstable,
    Vicious,
}

bitflags::bitflags! {
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct TraumaFlags: u8 {
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

impl Trauma {
    pub const ALL: [Trauma; 8] = [
        Trauma::Cold,
        Trauma::Haunted,
        Trauma::Obsessed,
        Trauma::Paranoid,
        Trauma::Relentless,
        Trauma::Soft,
        Trauma::Unstable,
        Trauma::Vicious,
    ];
}

impl From<Trauma> for TraumaFlags {
    fn from(trauma: Trauma) -> Self {
        match trauma {
            Trauma::Cold => TraumaFlags::COLD,
            Trauma::Haunted => TraumaFlags::HAUNTED,
            Trauma::Obsessed => TraumaFlags::OBSESSED,
            Trauma::Paranoid => TraumaFlags::PARANOID,
            Trauma::Relentless => TraumaFlags::RELENTLESS,
            Trauma::Soft => TraumaFlags::SOFT,
            Trauma::Unstable => TraumaFlags::UNSTABLE,
            Trauma::Vicious => TraumaFlags::VICIOUS,
        }
    }
}

impl std::fmt::Display for Trauma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Trauma::Cold => write!(f, "Cold"),
            Trauma::Haunted => write!(f, "Haunted"),
            Trauma::Obsessed => write!(f, "Obsessed"),
            Trauma::Paranoid => write!(f, "Paranoid"),
            Trauma::Relentless => write!(f, "Relentless"),
            Trauma::Soft => write!(f, "Soft"),
            Trauma::Unstable => write!(f, "Unstable"),
            Trauma::Vicious => write!(f, "Vicious"),
        }
    }
}
