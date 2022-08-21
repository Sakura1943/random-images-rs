#![feature(proc_macro_hygiene, decl_macro)]

mod tools;
use std::{fs::File};

use tools::read_config::ReadConfig;
use tools::read_image_list::ReadImageList;
use rand::Rng;

use rocket::{get, routes, Rocket, response::Redirect};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket().launch();
    Ok(())
}

#[get("/")]
fn index() -> Redirect {
    let configuration = ReadConfig::dir_exists("./config").unwrap().read_config();
    let image_folder = configuration.base.image_path;
    let image_list = ReadImageList::new(format!("{}", image_folder));
    let length = image_list.image_list.len();
    let random_number = rand::thread_rng().gen_range(0..length);
    println!("{}", image_list.image_list[random_number]);
    Redirect::to(format!("/random/{}",image_list.image_list[random_number]))
}

#[get("/<path>")]
fn random_redirect(path: String) -> Option<File> {
    File::open(format!("./images/{}", path)).ok()
}

fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![index]).mount("/random", routes![random_redirect])
}
