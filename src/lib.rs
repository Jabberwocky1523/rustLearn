use std::error::Error;

slint::include_modules!();

pub fn slint() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()?;

    Ok(())
}
#[no_mangle]
pub extern  "C" fn open()->i32{
    let _ = slint();
    3
}

#[no_mangle]
pub extern "C" fn hello(){
    println!("this rust dll")
}
#[no_mangle]
pub extern  "C" fn add(a:&i32,b:&i32)->i32{
    a + b
}