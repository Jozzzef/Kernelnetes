__kernel void add_scalars(__global float* result, __global const float* a, __global const float* b) {
    // Only execute this once (with first work-item)
    if (get_global_id(0) == 0) {
        result[0] = a[0] + b[0];
    }
}
