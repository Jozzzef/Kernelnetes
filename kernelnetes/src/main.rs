mod host;
mod apps;
use apps::add_scalars::add_scalars;
use apps::matrix_multiply::matrix_multiply;

fn main() {
    // Setup host environment
    let ctx = host::easy_setup();
    // Do actual execution (command queues -> programs -> kernels)
    // in apps folder is where you put your code
    let res = add_scalars(&ctx, 1.0, 2.0);
    println!("{}", res);
    println!("___");

    let a: Vec<Vec<f64>> = vec![vec![1.0,2.0,3.0], vec![1.0,2.0,3.0], vec![1.0,2.0,3.0], vec![1.0,2.0,3.0]];
    let b: Vec<Vec<f64>> = vec![vec![1.0,2.0,3.0,4.0], vec![1.0,2.0,3.0,4.0], vec![1.0,2.0,3.0,4.0]];
    let res_mm = matrix_multiply(&ctx, &a, &b);
    println!("{:?}", res_mm);
    println!("___");
}
