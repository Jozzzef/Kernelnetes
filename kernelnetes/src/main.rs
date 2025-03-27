mod host;
mod apps;
use apps::add_scalars::add_scalars;

fn main() {
    // Setup host environment
    let ctx = host::easy_setup();
    // Do actual execution (command queues -> programs -> kernels)
    // in apps folder is where you put your code
    add_scalars(&ctx);
}


