use opencl3::program::Program;
use opencl3::context::Context;
use opencl3::command_queue::CommandQueue;
use opencl3::kernel::Kernel;
use opencl3::memory::{Buffer, ClMem};

pub fn add_scalars(ctx: &Context) {
    println!("add_scalar");
    const FILE_LOC: &str = "./kernels/add_scalars.c";
    let cq = create_command_queue(ctx);
    let prog = compile_program(FILE_LOC, ctx);
    let kernel = get_kernel(&prog, "add_scalars");
    //need to add memory stuff here now ...
}

pub fn compile_program(path_to_file: &str, ctx: &Context) -> Program {
    let program_source = std::fs::read_to_string(path_to_file)
        .expect("Failed to read kernel file");
    Program::create_and_build_from_source(ctx, program_source.as_str(), "").unwrap()
}

pub fn create_command_queue(ctx: &Context){
    CommandQueue::create_default_with_properties(ctx, 0, 0).unwrap();
}

pub fn get_kernel(program: &Program, kern_name: &str){
    Kernel::create(program, kern_name).expect("Kernel::create failed");
}
