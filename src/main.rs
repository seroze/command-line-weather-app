
use std::error::Error;
use std::env; 
use serde::{Deserialize};

use dotenv::dotenv;
use clap::Parser;
use reqwest;

const LAT: f32 = -41.2; 
const LON: f32 = 174.7;

#[derive(Parser)]
#[command(name="wet")]
#[command(about="Weather in your terminal", long_about=None)]
struct Args{


    #[arg(short, default_value_t=0)]
    days: u8,
}

#[derive(Debug, Deserialize)]
struct Coordinate{
    lat: f32,
    lon: f32, 
}

#[derive(Debug, Deserialize)]
struct Weather{
    id: i32, 
    main: String, 
    description: String,
    icon: String, 
}

#[derive(Debug, Deserialize)]
struct MainWeather{
    temp: f32,
    feels_like: f32,

}

#[derive(Debug, Deserialize)]
struct CurrentWeather{
    coord: Coordinate,
    weather: Vec<Weather>,
    base: String,
    main: MainWeather,
}

fn main() -> Result<(), Box<dyn Error>>{
    println!("Hello, world!");
    // dotenv::dotenv().unwrap(); //read from local dotenv files 
    dotenv().ok();//this line loads all the .env files 

    let openweather_api_token = env::var("OPENWEATHER_API_TOKEN").expect("OPENWEATHER_API_TOKEN must be set in .env file");
    println!("{}", openweather_api_token);

    // for (key, value) in env::vars(){
    //     println!("{key}:{value}");
    // }

    let args = Args::parse();
    println!("{}", args.days);

    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={LAT}&lon={LON}&appid={openweather_api_token}&units=metric&cnt=2");

    // let body: String  = reqwest::blocking::
    //     get("https://www.rust-lang.org")?
    //     .text()?;

    // let body: String  = reqwest::blocking::
    //     get(url)?
    //     .text()?;

    let body: CurrentWeather  = reqwest::blocking::
        get(url)?
        .json()?;
    println!("{:?}", body);

    Ok(())
}
