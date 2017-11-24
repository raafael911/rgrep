extern crate rgrep;

use rgrep::Config;

fn main() {

    let params = Config::new();

    let matches = rgrep::do_search(&params);

    for mat in matches {

        rgrep::print_match(mat);
    }
}
