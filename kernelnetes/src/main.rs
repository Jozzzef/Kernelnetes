mod host;
mod apps;
use apps::add_scalars::add_scalars;

fn main() {
    // Setup host environment
    let ctx = host::easy_setup();
    // Do actual execution (command queues -> programs -> kernels)
    // in apps folder is where you put your code
    let res = add_scalars(&ctx, 1.0, 2.0);
    println!("{}", res);
    println!("___");
}
