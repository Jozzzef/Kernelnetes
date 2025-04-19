__kernel void matrix_multiply(__global const float* matrix_A, 
                              __global const float* matrix_B,
                              __global float* matrix_output,
                              const int n_dim_a){

    int row = get_global_id(0);
    int col = get_global_id(1);

    printf("kernel #(%i,%i) | n_dim_a = %i \n", row, col, n_dim_a);

    float sum = 0.0f;

    for(int k = 0; k < n_dim_a; k++) {
        sum += matrix_A[row * n_dim_a + k] * matrix_B[k * n_dim_a + col];
    }
    
    matrix_output[row * n_dim_a + col] = sum;
}
