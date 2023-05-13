use crate::domain::controls::Controls;
use ggrs::Config;
use zoop_shared::player_id::PlayerId;

#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = Controls;
    // Docs say this can be left as u8 :shrugs:
    type State = u8;
    type Address = PlayerId;
}
