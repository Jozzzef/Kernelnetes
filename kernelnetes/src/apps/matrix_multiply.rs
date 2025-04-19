use opencl3::context::Context;
use opencl3::command_queue::{CommandQueue, CL_BLOCKING, CL_NON_BLOCKING};
use opencl3::memory::{Buffer, 
                        CL_MEM_READ_ONLY, 
                        CL_MEM_WRITE_ONLY, 
                        CL_MEM_READ_WRITE,
                        CL_MEM_COPY_HOST_PTR}; 
use std::ptr::null_mut;
use std::ffi::c_void;
use super::helpers::*;
use opencl3::types::cl_int;

pub fn matrix_multiply(ctx: &Context, matrix_a: &Vec<Vec<f64>>, matrix_b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    println!("matrix_multiply");
    const FILE_LOC: &str = "./src/kernels/matrix_multiply.cl";
    let cq = create_command_queue(ctx);
    let prog = compile_program(FILE_LOC, ctx);
    let krnl = get_kernel(&prog, "matrix_multiply");

    //matrix data manipulation
    // For matrix_a
    let m_dim_a = matrix_a.len(); // Number of rows
    let n_dim_a = if !matrix_a.is_empty() { matrix_a[0].len() } else { 0 }; // Number of columns
    // For matrix_b
    let m_dim_b = matrix_b.len(); // Number of rows
    let n_dim_b = if !matrix_b.is_empty() { matrix_b[0].len() } else { 0 }; // Number of columns
    if n_dim_a != m_dim_b {
        println!("{}, {}", n_dim_a, m_dim_b);
        panic!("Inner dimensions are not the same");
    }
    let flat_matrix_a: Vec<f64> = matrix_a.iter()
        .flat_map(|inner_vec| inner_vec.iter().copied())
        .collect();
    let flat_matrix_b: Vec<f64> = matrix_b.iter()
        .flat_map(|inner_vec| inner_vec.iter().copied())
        .collect();

    let input_val_scalar = cl_int::from(n_dim_a as i32);
    unsafe {
        let buffer_a = Buffer::<f64>::create(ctx, 
                            CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR, 
                            flat_matrix_a.len(), 
                            flat_matrix_a.as_ptr() as *mut c_void //point to the data to copy to buff
                        ).unwrap();
        let buffer_b = Buffer::<f64>::create(ctx, 
                            CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR, 
                            flat_matrix_b.len(),
                            flat_matrix_b.as_ptr() as *mut c_void
                        ).unwrap();
        let result_buffer = Buffer::<f64>::create(ctx, 
                                CL_MEM_WRITE_ONLY, 
                                m_dim_a * n_dim_b, 
                                null_mut() //dont initialize with any data from the host
                        ).unwrap();
        krnl.set_arg(0, &buffer_a).unwrap_or_else(|err| {
            eprintln!("Error occurred: {}", err);
        });
        krnl.set_arg(1, &buffer_b).unwrap_or_else(|err| {
            eprintln!("Error occurred: {}", err);
        });
        krnl.set_arg(2, &result_buffer).unwrap_or_else(|err| {
            eprintln!("Error occurred: {}", err);
        });
        println!("{}", prog.kernel_names());
        krnl.set_arg(3, &input_val_scalar).unwrap_or_else(|err| {
            eprintln!("Error occurred: {}", err);
        });
        let global_work_size: [usize; 3] = [m_dim_a, n_dim_b, 1]; // The global work size represents how many instances of your kernel need to run in parallel
                // only need one work-item (one thread) to execute the kernel once
        let local_work_size: [usize; 3] = [1, 1, 1]; // Let OpenCL decide
        let _ = cq.enqueue_nd_range_kernel(krnl.get(), 
                                    2, //tell OpenCL to just consider the first two dimensions
                                    std::ptr::null(), // null for default offset of 0 
                                    global_work_size.as_ptr(), 
                                    local_work_size.as_ptr(), 
                                    &[] // no events to wait on
                                );
        cq.finish().unwrap();
        let mut result_data: Vec<f64> = Vec::new();
        let _ = cq.enqueue_read_buffer(&result_buffer, 
                                                CL_BLOCKING, 
                                                0, 
                                                &mut result_data, 
                                                &[]
                                            ).unwrap();
        println!("res = {:?}", result_data);

        //map the resulting flat Vec to a nested Vec
        if !result_data.is_empty() {
            (0..m_dim_a) //for each row in the matrix
                .map(|row|{
                    let start = row * n_dim_b;
                    let end = start + n_dim_b;
                    result_data[start..end].to_vec()
                }).collect::<Vec<Vec<f64>>>()
        } 
        else {
            println!("The result is empty!");
            Vec::<Vec<f64>>::new()
        }
    }
}
