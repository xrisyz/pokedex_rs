#[macro_use]
extern crate reqwest;
extern crate serde_json;
extern crate serde;
extern crate serde_derive;
extern crate tokio;


use serde::{Serialize, Deserialize};


mod pokeapi_endpoint;
mod poke_error;
use crate::pokeapi_endpoint::PokeAPIEndpoint;
use crate::poke_error::PokeError;
use std::fmt;
use std::future::Future;

use reqwest::Error;

enum PokeRegion {
    National,
    Kanto,
    Johto,
    Hoenn,
    Sinnoh,
    Unova,
    Kalos
}

impl PokeRegion {
    fn get_string(&self) -> String {
        match self {
            PokeRegion::National => "national".to_string(),
            PokeRegion::Kanto => "kanto".to_string(),
            PokeRegion::Johto => "johto".to_string(),
            PokeRegion::Hoenn => "hoenn".to_string(),
            PokeRegion::Unova => "unove".to_string(),
            PokeRegion::Kalos => "kalos".to_string(),
            PokeRegion::Sinnoh => "sinnoh".to_string()
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct Language {
    name: String,
    url: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Description {
    description: String
}

#[derive(Serialize, Deserialize, Debug)]
struct PokeDexName {
    name: String,
    language: Language
}

#[derive(Serialize, Deserialize, Debug)]
struct PokemonResource {
    name: String,
    url: String
}
#[derive(Serialize, Deserialize, Debug)]
struct PokeDexEntries {
    entry_number: i32,
    pokemon_species: PokemonResource
}

#[derive(Serialize, Deserialize, Debug)]
struct NamedResource {
    name: String,
    url: String
}

#[derive(Serialize, Deserialize, Debug)]
struct PokeDexResponse {
    id: i32,
    name: String,
    is_main_series: bool,
    names: Vec<PokeDexName>,
    descriptions: Vec<Description>,
    pokemon_entries: Vec<PokeDexEntries>,
    version_groups: Vec<NamedResource>
}

impl PokeDexResponse {
    fn get_entries(mut self) -> Vec<PokeDexEntries> {
        self.pokemon_entries
    }

    fn check_entries_count(mut self) -> Result<Vec<PokeDexEntries>, PokeError> {
        if self.get_entries().iter().count() > 0 {
            return Ok(self.pokemon_entries)
        } else {
            let error = PokeError {
                details: String::from("no pokemon entries found")
            };
            return Err(error)
        }
    }
}


#[tokio::main]
async fn main()   {
    println!("Hello World");
    let pokemon_name = String::from("pikachu");
    let endpoint_manager = PokeAPIEndpoint::build();
    let pokedex_response = get_pokedex(PokeRegion::National, endpoint_manager).await;
    let pokemon_count = pokedex_response.pokemon_entries.iter().count();
    let pokemon_entry = pokedex_response
        .check_entries_count()
        .unwrap()
        .iter()
        .filter(|&p| p.pokemon_species.name == pokemon_name)
        .next()
        .unwrap();


    println!("name: {} \n url: {}", pokemon_entry.pokemon_species.name, pokemon_entry.pokemon_species.url)
}

async fn get_pokemon(name: &str, endpoint: PokeAPIEndpoint) -> String {
    let endpoint = endpoint.get_pokemon(name).get_path();
    println!("Endpoint: {}", endpoint);
    let response = reqwest::get(endpoint.as_str()).await.unwrap().text().await.unwrap();
    response
}

fn transform_poke_respons_to_entries(response: PokeDexResponse) -> Vec<PokeDexEntries> {
    response.pokemon_entries
}


/// Gets the pokedex response from the poke api server
async fn get_pokedex(region: PokeRegion, endpoint_manager: PokeAPIEndpoint) -> PokeDexResponse {
    let endpoint_manager = endpoint_manager.construct_pokedex_entry_endpoint(region.get_string().as_str());
    let endpoint = endpoint_manager.get_path();
    let body = reqwest::get(endpoint.as_str()).await.unwrap().text().await.unwrap();
    let pokedex_response: PokeDexResponse = serde_json::from_str(body.as_str()).unwrap();
    pokedex_response
}