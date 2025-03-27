__kernel void add_scalars(__global float* result, const float a, const float b) {
    // Only execute this once (with first work-item)
    if (get_global_id(0) == 0) {
        result[0] = a + b;
    }
}
