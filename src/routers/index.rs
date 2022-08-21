use poem::{
  handler,
  web::Redirect,
  http::Uri,
};
use crate::tools::read_config::ReadConfig;
use crate::tools::read_image_list::ReadImageList;
use rand::Rng;


#[handler]
pub async fn index() -> Redirect {
  let configuration = ReadConfig::dir_exists("./config").unwrap().read_config();
  let image_folder = configuration.base.image_path;
  let image_list = ReadImageList::new(format!("{}", image_folder));
  let length = image_list.image_list.len();
  let random_number = rand::thread_rng().gen_range(0..length);
  let file_name = &image_list.image_list[random_number];
  let url = format!("http://127.0.0.1:3000/images/{}", file_name);
  Redirect::moved_permanent(Uri::from_static(Box::leak(url.into_boxed_str())))
}
