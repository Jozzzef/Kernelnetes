use opencl3::program::Program;
use opencl3::context::Context;
use opencl3::command_queue::CommandQueue;
use opencl3::kernel::Kernel;
use opencl3::memory::{Buffer, 
                        CL_MEM_READ_ONLY, 
                        CL_MEM_WRITE_ONLY, 
                        CL_MEM_READ_WRITE,
                        CL_MEM_COPY_HOST_PTR}; // Allocate new memory on the device (GPU)
use std::ptr::null_mut;

pub fn add_scalars(ctx: &Context, a: f32, b: f32) -> f64 {
    println!("add_scalar");
    const FILE_LOC: &str = "./kernels/add_scalars.c";
    let cq = create_command_queue(ctx);
    let prog = compile_program(FILE_LOC, ctx);
    let kernel = get_kernel(&prog, "add_scalars");
    let host_data_a = vec![a];
    let host_data_b = vec![b];
    let buffer_a = Buffer::<f32>::create_from_vec(ctx, CL_MEM_COPY_HOST_PTR, &host_data_a).unwrap();
    let buffer_b = Buffer::<f32>::create_from_vec(ctx, CL_MEM_COPY_HOST_PTR, &host_data_b).unwrap();
    unsafe {
        let result_buffer = Buffer::<f64>::create(ctx, CL_MEM_WRITE_ONLY, 1, null_mut()).unwrap();
        kernel.set_arg(0, &result_buffer).unwrap();
        kernel.set_arg(1, &buffer_a).unwrap();
        kernel.set_arg(2, &buffer_b).unwrap();
    }

    3.0 // remove later
}

pub fn compile_program(path_to_file: &str, ctx: &Context) -> Program {
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
