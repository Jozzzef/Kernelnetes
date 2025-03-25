use opencl3 as cl3;
use opencl3::platform::Platform;
use std::ptr;


pub fn init_host() {
    println!("init");
}

pub fn get_platform(){
    println!("get platform");
    let plats_res = cl3::platform::get_platforms();
    let plats : Vec<Platform> = match plats_res {
        Ok(array) => array,
        Err(e) => {
            println!("{}", e);
            let empty_plat: Platform = Platform::new(ptr::null_mut());
            vec![empty_plat]
        }
    };
    for el in &plats {
        print!("{:?} ", el.name());
    }
}

pub fn get_device(){

}

pub fn create_context(){

}

pub fn create_command_queue(){

}

pub fn easy_setup(){
    init_host(); 
    get_platform(); 
    get_device(); 
    get_device(); 
    create_context(); 
    create_command_queue();
}
