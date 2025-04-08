use opencl3::program::Program;
use opencl3::context::Context;
use opencl3::command_queue::{CommandQueue, CL_BLOCKING, CL_NON_BLOCKING};
use opencl3::kernel::Kernel;
use opencl3::memory::{Buffer, 
                        CL_MEM_READ_ONLY, 
                        CL_MEM_WRITE_ONLY, 
                        CL_MEM_READ_WRITE,
                        CL_MEM_COPY_HOST_PTR}; 
use std::ptr::null_mut;
use std::ffi::c_void;
use std::path::Path;
use std::env;

pub fn add_scalars(ctx: &Context, a: f32, b: f32) -> f32 {
    println!("add_scalar");
    const FILE_LOC: &str = "./src/kernels/add_scalars.c";
    let cq = create_command_queue(ctx);
    let prog = compile_program(FILE_LOC, ctx);
    let krnl = get_kernel(&prog, "add_scalars");
    let host_data_a = [a];
    let host_data_b = [b];
    unsafe {
        let buffer_a = Buffer::<f32>::create(ctx, 
                            CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR, 
                            host_data_a.len(), 
                            host_data_a.as_ptr() as *mut c_void //point to the data to copy to buff
                        ).unwrap();
        let buffer_b = Buffer::<f32>::create(ctx, 
                            CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR, 
                            host_data_b.len(),
                            host_data_b.as_ptr() as *mut c_void
                        ).unwrap();
        let result_buffer = Buffer::<f32>::create(ctx, 
                                CL_MEM_WRITE_ONLY, 
                                1, 
                                null_mut() //dont initialize with any data from the host
                        ).unwrap();
        krnl.set_arg(0, &result_buffer).unwrap();
        krnl.set_arg(1, &buffer_a).unwrap();
        krnl.set_arg(2, &buffer_b).unwrap();
        let global_work_size: [usize; 3] = [1, 1, 1]; // The global work size represents how many instances of your kernel need to run in parallel
                // only need one work-item (one thread) to execute the kernel once
        let local_work_size: [usize; 3] = [1, 1, 1]; // Let OpenCL decide
        let _ = cq.enqueue_nd_range_kernel(krnl.get(), 
                                    1, //OpenCL should ignore global_work_size[1] and global_work_size[2] (that's why they're set to 0) 
                                    std::ptr::null(), // null for default offset of 0 
                                    global_work_size.as_ptr(), 
                                    local_work_size.as_ptr(), 
                                    &[] // no events to wait on
                                );
        cq.finish().unwrap();
        let mut result_data = [0.0f32; 1];

        let _ = cq.enqueue_read_buffer(&result_buffer, 
                                                CL_BLOCKING, 
                                                0, 
                                                &mut result_data, 
                                                &[]
                                            ).unwrap();
        println!("res = {:?}", result_data);
        //return
        result_data[0]
    }
}

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

