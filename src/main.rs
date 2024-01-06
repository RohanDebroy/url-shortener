use url_shortener;

pub fn main() {
    let result = url_shortener::start();

    if let Some(err) = result.err() {
        println!("Error: {}", err)
    }
}
