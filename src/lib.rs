pub mod result;

pub use result::*;

use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Record {
    Atom(Atom),
    Other(String),
}

impl FromStr for Record {
    type Err = ParseRecordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..6] {
            "ATOM  " => Ok(Record::Atom(s.parse()?)),
            _ => Ok(Record::Other(s.to_string()))
        }
    }
}

macro_rules! parse_data {
    ( $s: expr, String ) => {
        $s.to_string()
    };
    ( $s: expr,  $_: ident ) => {
        $s.trim().parse()?
    }
}

macro_rules! define_record {
    { $record: ident {$($name: ident : $type: ident $range: expr),*} } => {
        #[derive(Debug, PartialEq)]
        pub struct Atom {
            $(
                $name: $type,
            )*
        }

        impl FromStr for Atom {
            type Err = ParseRecordError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($record {
                    $(
                        $name: parse_data!(s[$range], $type),
                    )*
                })
            }
        }
    }
}

define_record! {
    Atom {
        serial:      i32     6..11,
        name:        String  12..16,
        alt_loc:     String  16..17,
        res_name:    String  17..20,
        chain_id:    String  21..22,
        res_seq:     i16     22..26,
        i_code:      String  26..27,
        x:           f32     30..38,
        y:           f32     38..46,
        z:           f32     46..54,
        occupancy:   f32     54..60,
        temp_factor: f32     60..66,
        element:     String  76..78,
        charge:      String  78..80
    }
}
