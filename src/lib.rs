// use std::error::Error;

// slint::include_modules!();

// pub fn slint() -> Result<(), Box<dyn Error>> {
//     let ui = MainWindow::new()?;

//     ui.run()?;

//     Ok(())
// }

// #[no_mangle]
// pub extern  "C" fn open()->i32{
//     let _ = slint();
//     3
// }

// #[no_mangle]
// pub extern "C" fn hello(){
//     println!("this rust dll")
// }
// #[no_mangle]
// pub extern  "C" fn add(a:&i32,b:&i32)->i32{
//     a + b
// }
