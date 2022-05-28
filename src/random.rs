use rand::Rng;

pub fn get_random_number() -> String {
  rand::thread_rng().gen::<i32>().to_string()
}