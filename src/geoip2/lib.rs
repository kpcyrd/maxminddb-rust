#![crate_id = "geoip2#0.1.0-pre"]

#![comment = "MaxMind GeoIP2"]
#![license = "Apache 2"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

extern crate collections;
extern crate maxminddb;
extern crate serialize;

#[deriving(Decodable, Show)]
pub struct Names {
    en: StrBuf,
}

#[deriving(Decodable, Show)]
pub struct Continent {
    code: StrBuf,
    geoname_id: uint,
    names: Names,
}

#[deriving(Decodable, Show)]
pub struct Place {
    geoname_id: uint,
    iso_code: StrBuf,
    names: Names,
}

#[deriving(Decodable, Show)]
pub struct Traits {
    is_anonymous_proxy: bool,
    is_satellite_provider: bool,
}

#[deriving(Decodable, Show)]
pub struct Country {
    continent: Continent,
    country: Place,
    registered_country: Place,
    represented_country: Place,
    traits: Traits,
}