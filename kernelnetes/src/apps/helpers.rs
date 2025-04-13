use std::path::Path;
use std::env;
use opencl3::kernel::Kernel;
use opencl3::program::Program;
use opencl3::context::Context;
use opencl3::command_queue::{CommandQueue, CL_BLOCKING, CL_NON_BLOCKING};

pub fn compile_program(path_to_file: &str, ctx: &Context) -> Program {
    let p = Path::new(path_to_file);
    let cwd = env::current_dir().unwrap();
    if p.exists() {
        println!("File exists! @ {:?}", p.canonicalize());
    } else {
        println!("File does not exist! CWD is: {:?}", cwd);
    }
    let program_source = std::fs::read_to_string(path_to_file)
        .expect("Failed to read kernel file");
    Program::create_and_build_from_source(ctx, program_source.as_str(), "").unwrap()
}

pub fn create_command_queue(ctx: &Context) -> CommandQueue {
    CommandQueue::create_default_with_properties(ctx, 0, 0).unwrap()
}

pub fn get_kernel(program: &Program, kern_name: &str) -> Kernel {
    Kernel::create(program, kern_name).expect("Kernel::create failed")
}

