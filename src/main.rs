#[macro_use]
extern crate serde_derive;
extern crate bincode;

#[macro_use]
extern crate nom;

use bincode::{deserialize, serialize};

use nom::number::streaming::{le_u64};
use std::str::from_utf8;

//Generic
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Content{}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ConfigGeneric
{
    pub header : String,//[u8; 16],
    pub content : Content,
}

// ConfigA
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ContentA
{
    pub field_str : String,
    pub field_u32 : u32,
    pub field_u16 : u16,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ConfigA
{
    pub header : String,
    pub content : ContentA,
}


impl Default for ConfigA {
    fn default() -> Self {
        ConfigA{ header : "ConfigA".to_string(), content : ContentA::default() }
    }
}

// ConfigB
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ContentB
{
    pub field_str : String,
    pub field_i128 : i128,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ConfigB
{
    pub header : String,
    pub content : ContentB,
}

impl Default for ConfigB {
    fn default() -> Self {
        ConfigB{ header : "ConfigB".to_string(), content : ContentB::default() }
    }
}

//Conversions
impl From<ConfigA> for ConfigB {
    fn from(cfg: ConfigA) -> Self {
        return ConfigB{
            header : ConfigB::default().header,
            content : ContentB{ field_str : cfg.content.field_str, field_i128 : 0},
        };
    }
}

impl From<ConfigB> for ConfigA {
    fn from(cfg: ConfigB) -> Self {
        return ConfigA{
            header : ConfigA::default().header,
            content : ContentA{ field_str : cfg.content.field_str, field_u32 : 0, field_u16 : 0},
        };
    }
}

pub enum ConfigType {
    ConfigA,
    ConfigB,
}

named!(pub le_string<&str>, map_res!(length_data!(le_u64), from_utf8));

named!(pub parse_config<ConfigGeneric>,
  do_parse!(
    header: le_string  >>
    (ConfigGeneric {
        header: header.to_string(),
        content: Content::default()
    })
  )
);

fn decode_config(encoded_config : &Vec<u8>) -> Option<ConfigType>
{
    let generic_cfg = parse_config(&encoded_config);
    println!("generic_cfg : {:?}", generic_cfg);

    let (_, gen_config) = generic_cfg.unwrap();

    match gen_config.header.as_str()
    {
        "ConfigA" => {
            return Some(ConfigType::ConfigA);
        },
        "ConfigB" => {
            return Some(ConfigType::ConfigB);
        },
        _ => {
            println!("Unrecognized list : '{}'. Please, Update me!", gen_config.header);
            return None;
        }
    };
}

fn main() {
    //Encoding
    let config_a = ConfigA{header : "ConfigA".to_string() , content : ContentA::default() };
    let encoded_config = serialize(&config_a).unwrap();
    println!("encoded_config_a : {:?}", encoded_config);

    let config_type = decode_config(&encoded_config).unwrap();

    match config_type
    {
        ConfigType::ConfigA => {
            let decoded: ConfigA = deserialize(&encoded_config).unwrap();
            println!("decoded ConfigA : {:?}", decoded);

            //Conversion to B
            let converted_from_a = ConfigB::from(decoded);
            println!("converted_from_a ConfigB : {:?}", converted_from_a);
        },
        ConfigType::ConfigB => {
            let decoded: ConfigB = deserialize(&encoded_config).unwrap();
            println!("decoded ConfigB : {:?}", decoded);

            //Conversion to A
            let converted_from_b = ConfigA::from(decoded);
            println!("converted_from_b ConfigA : {:?}", converted_from_b);
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conv_a_to_b() {
        let config_a = ConfigA{header : ConfigA::default().header , content : ContentA{ field_str : "test".to_string(), field_u32 : 12, field_u16 : 43 }};
        let config_a_cloned = config_a.clone();
        let converted_from_a = ConfigB::from(config_a);

        assert_eq!(converted_from_a.header.as_str(), "ConfigB");
        assert_eq!(converted_from_a.content.field_str.as_str(), config_a_cloned.content.field_str.as_str());
        assert_eq!(converted_from_a.content.field_i128, 0);
    }

    #[test]
    fn test_conv_b_to_a() {
        let config_b = ConfigB{header : ConfigB::default().header , content : ContentB{ field_str : "test2".to_string(), field_i128 : 777}};
        let config_b_cloned = config_b.clone();
        let converted_from_b = ConfigA::from(config_b);

        assert_eq!(converted_from_b.header.as_str(), "ConfigA");
        assert_eq!(converted_from_b.content.field_str.as_str(), config_b_cloned.content.field_str.as_str());
        assert_eq!(converted_from_b.content.field_u32, 0);
        assert_eq!(converted_from_b.content.field_u16, 0);
    }
}