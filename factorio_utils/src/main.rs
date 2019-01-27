use base64::decode;
use flate2::read::ZlibDecoder;
use serde_derive::{Serialize, Deserialize};
use serde_json::Result;
use std::collections::HashMap;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
enum BlueprintObject {
    #[serde(rename = "blueprint")]
    Blueprint(Blueprint),

    #[serde(rename = "blueprint_book")]
    BlueprintBook(BlueprintBook)
}

#[derive(Serialize, Deserialize, Debug)]
struct BlueprintBook {
    item: String,
    label: String,
    blueprints: Vec<BlueprintBookEntry>,
    active_index: u8,
    version: u64
}

#[derive(Serialize, Deserialize, Debug)]
struct BlueprintBookEntry {
    index: u8,
    blueprint: Blueprint
}

#[derive(Serialize, Deserialize, Debug)]
struct Blueprint {
    item: String,
    label: String,
    entities: Vec<Entity>,

    #[serde(default)]
    tiles: Vec<Tile>,

    #[serde(default)]
    icons: Vec<Icon>,

    version: u64
}

#[derive(Serialize, Deserialize, Debug)]
struct Entity {
    entity_number: u8,
    name: String,
    position: Position,
    direction: Option<u8>,

    #[serde(default)]
    connections: HashMap<u8, Connection>,

    #[serde(default)]
    items: HashMap<String, u32>,

    recipe: Option<String>,
    bar: Option<u8>,
    infinity_settings: Option<InfinitySettings>,

    #[serde(rename = "type")]
    underground_type: Option<UndergroundBeltOrLoaderType>,

    input_priority: Option<SplitterDirection>,
    output_priority: Option<SplitterDirection>,
    filter: Option<String>,

    #[serde(default)]
    filters: Vec<ItemFilter>,

    override_stack_size: Option<u8>,
    drop_position: Option<Position>,
    pickup_position: Option<Position>,
    request_filters: Option<LogisticFilter>,
    request_from_buffers: Option<bool>,
    parameters: Option<SpeakerParameter>,
    alert_parameters: Option<SpeakerAlertParameter>,
    auto_launch: Option<bool>,
    color: Option<Color>,
    station: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Tile {
    name: String,
    position: Position
}

#[derive(Serialize, Deserialize, Debug)]
struct Icon {
    index: u8,
    signal: SignalID
}

#[derive(Serialize, Deserialize, Debug)]
struct SignalID {
    name: String,

    #[serde(rename = "type")]
    signal_type: SignalType
}

#[derive(Serialize, Deserialize, Debug)]
enum SignalType {
    #[serde(rename = "item")]
    Item,

    #[serde(rename = "fluid")]
    Fluid,

    #[serde(rename = "virtual")]
    Virtual
}

#[derive(Serialize, Deserialize, Debug)]
struct Position {
    x: f64,
    y: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct Connection {
    first_point: ConnectionPoint,
    second_point: ConnectionPoint
}

#[derive(Serialize, Deserialize, Debug)]
struct ConnectionPoint {
    red: Vec<ConnectionData>,
    green: Vec<ConnectionData>
}

#[derive(Serialize, Deserialize, Debug)]
struct ConnectionData {
    entity_id: u8,
    circuit_id: u8
}

#[derive(Serialize, Deserialize, Debug)]
struct InfinitySettings {

}

#[derive(Serialize, Deserialize, Debug)]
enum UndergroundBeltOrLoaderType {
    #[serde(rename = "input")]
    Input,

    #[serde(rename = "output")]
    Output
}

#[derive(Serialize, Deserialize, Debug)]
struct ItemFilter {

}

#[derive(Serialize, Deserialize, Debug)]
struct LogisticFilter {

}

#[derive(Serialize, Deserialize, Debug)]
struct SpeakerParameter {

}

#[derive(Serialize, Deserialize, Debug)]
struct SpeakerAlertParameter {

}

#[derive(Serialize, Deserialize, Debug)]
struct Color {

}

#[derive(Serialize, Deserialize, Debug)]
enum SplitterDirection {
    #[serde(rename = "left")]
    Left,

    #[serde(rename = "right")]
    Right
}

fn main() -> Result<()> {
    let blueprint: EncodedBlueprint = EncodedBlueprint("0eNrtmj1vo0AQhv9KtDVE+8mC6yuuuCrtKYqwjRJ0DlgYRxdF/u8Hxk5ie5y8g+GKKFUEmMe7OzP78cQvYrpYZ8sqL+q7aVn+EZOXtzsrMfn97rJ9ls/Koru9yu+LdNHeq5+XmZiIvM4eRSCK9LG9qtJ8ITaByIt59ldM1OY2EFlR53Wede9vL57vivXjNKuaD7y+uaqbd+8f6nCLCMSyXDVvlUX7VQ0ptIF4bv6oaLMJTjAaxeiPKOaVMltXT9n8XFPMDqKajs7zKpt1Dy2BtNyGWaphjkvRFCU6iFI4e0jzItzF87ST8tp1sOTaHfbTEGzPYR+gCVjM7a6kKAkWTdUxjvroCKCSaLtkx4wBpmIyo0OmopjcaoiAdhom0wLttJykMfukcUg+KrhkmrmAbLGmqBE4Q6hdVhmgpR5jGpIYUUS4fizZczL+CZOpP4+/husphiOk4XpSEofCBaU0DoUrSlkcCq83YTemEgiTA5O+q08FtDKCW6nJbKLqSHveTHrUc08hY9aKqfczlD6eochhhSsqIseAqlIjsVAlVKSoETCKMwL+7ABQ05TRvdgKmf6NYUUu2cHlMZvKXmN7scMTODng+LoVUWlMZkXEZAJzt/GsvbIBiHGfLeTJoJIDkHA3A/rzNLASh1oYip+HdksNAtU4VMJQwzy5AdOXtUwmsMm0jrnMIn3nnaTcLlcNMgFYz1wbgc2bjZlM4JBhWcerhDzVSma4Y5LCWqLa5WPL8kgwnO61AThhU2bAcQuIPuU6bs3QbsA5LoYWFRHvmNhSboPOIU3eKadALNJptmju3ZTrYp5Oy3V9tV7eV+k8u/r180fzgaesWnV7i9hrHatEev0mnmTbuG+F9XUV1iVmyPfyDglS1T1PDNBs9C20vpTQ8meFlv4WWueEFjCu+xU+hPZbbLEF7LdwscWQZWoEWabHkGVmDFlmx5BljjVZn7cb5jLHhZs4P7iJi4eXZskIcs/IEWSZGlqW6RHcjRnB3djB3Q2rktw5d+Mvk1e4uzF+BHdj4hHcDUddyRHUFe5ueMfw/a4asrkMgYV7ITOCF+pnhg1iy3GPxfBC0QheyF/shWwvEwz9W/idteKYO0//TmFwgXWRCNLDiCAzjAiy/0EE3QAiSFHYcPubp4/YzcO0yaOn7G7POfNFm39bmz2t".to_string());
    println!("{:?}", blueprint.decode()?);
    Ok(())
}

struct EncodedBlueprint(String);

impl EncodedBlueprint {

    fn get_base64(&self) -> &str {
        &self.0[1..]
    }

    fn get_version_byte(&self) -> u8 {
        self.0.as_bytes()[0]
    }

    fn as_string(&self) -> &String {
        &self.0
    }

    fn decode(&self) -> Result<BlueprintObject> {
        let bytes: Vec<u8> = decode(self.get_base64()).unwrap();
        let mut deflater = ZlibDecoder::new(&bytes[..]);
        let mut decompressed = String::new();
        deflater.read_to_string(&mut decompressed).unwrap();
        let bp: BlueprintObject = serde_json::from_str(&decompressed)?;
        Ok(bp)
    }
}
