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

#[cfg(feature = "server")]
mod server {}
