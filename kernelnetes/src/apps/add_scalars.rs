use opencl3::program::Program;
use opencl3::context::Context;

pub fn add_scalars() {
    println!("add_scalar");
    const FILE_LOC: &str = "./kernels/add_scalars.c";
}


pub fn compile_program(path_to_file: &str, ctx: &Context) -> Program {
    let program_source = std::fs::read_to_string(path_to_file)
        .expect("Failed to read kernel file");
    Program::create_and_build_from_source(ctx, program_source.as_str(), "").unwrap()
}

pub fn create_command_queue(){

}
