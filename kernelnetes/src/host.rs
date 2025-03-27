use opencl3 as cl3;
use opencl3::platform::Platform;
use opencl3::device::Device;
use std::ptr;

pub fn install_opencl_streamline(){
    let ubuntu = "sudo apt update \
        sudo apt install ocl-icd-opencl-dev opencl-headers clinfo";
    let nvidia = "sudo apt install nvidia-driver-535 nvidia-cuda-toolkit";
    let rocm = "sudo apt install rocm-opencl-runtime";
    let intel_cpu = "sudo apt install intel-opencl-icd";
    let arc = "sudo apt install intel-level-zero-gpu level-zero \
        sudo apt install intel-opencl-icd";
    let confirm = "clinfo";
}

pub fn init_host() {
    println!("init");
}

pub fn get_platforms() -> Vec<Platform> {
    let plats_res = cl3::platform::get_platforms();
    match plats_res {
        Ok(array) => array,
        Err(e) => {
            println!("{}", e);
            let empty_plat: Platform = Platform::new(ptr::null_mut());
            vec![empty_plat]
        }
    }
}

pub enum OCLDevices {
    Defaults,
    Cpus,
    Gpus,
    Accelerators,
    Customs,
    All
}

pub fn get_devices(p: Platform, device_type: OCLDevices) -> Vec<Device>{
   match device_type {
        OCLDevices::Defaults => p.get_devices(opencl3::device::CL_DEVICE_TYPE_DEFAULT)
                                    .unwrap()
                                    .into_iter()
                                    .map(Device::new)
                                    .collect(),
        OCLDevices::Cpus => p.get_devices(opencl3::device::CL_DEVICE_TYPE_CPU)
                                    .unwrap()
                                    .into_iter()
                                    .map(Device::new)
                                    .collect(),
        OCLDevices::Gpus => p.get_devices(opencl3::device::CL_DEVICE_TYPE_GPU)
                                    .unwrap()
                                    .into_iter()
                                    .map(Device::new)
                                    .collect(),
        OCLDevices::Accelerators => p.get_devices(opencl3::device::CL_DEVICE_TYPE_ACCELERATOR)
                                    .unwrap()
                                    .into_iter()
                                    .map(Device::new)
                                    .collect(),
        OCLDevices::Customs => p.get_devices(opencl3::device::CL_DEVICE_TYPE_CUSTOM)
                                    .unwrap()
                                    .into_iter()
                                    .map(Device::new)
                                    .collect(),
        OCLDevices::All => p.get_devices(opencl3::device::CL_DEVICE_TYPE_ALL)
                                    .unwrap()
                                    .into_iter()
                                    .map(Device::new)
                                    .collect()
    }

}

pub fn create_context(){

}

pub fn create_command_queue(){

}

pub fn easy_setup(){
    init_host(); 
    let plat_vec = get_platforms();
    println!("PLATFORMS VECTOR PRINT ===========>");
    for j in &plat_vec{
        println!("{:?} ", j.name().unwrap());
    }
    let d_vec = get_devices(plat_vec[0], OCLDevices::All); 
    for i in &d_vec {
        println!("DEVICE VECTOR PRINT ===========>");
        println!("device name: {:#?} | vendor: {:#?}", i.name().unwrap(), i.vendor().unwrap());
    }
    create_context(); 
    create_command_queue();
}
