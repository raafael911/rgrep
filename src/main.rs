extern crate rgrep;

use rgrep::Config;

fn main() {

    let params = Config::new();

    rgrep::do_search(&params);
}
