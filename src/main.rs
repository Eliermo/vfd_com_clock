use std::{thread::sleep, time};
use std::time::Duration;
use chrono::Local;
use escpos::printer::Printer;
use escpos::printer_options::PrinterOptions;
use escpos::utils::*;
use escpos::{driver::*, errors::Result};

fn main() {
    let now = Local::now();
    println!("Працуем з {}", now.format("%d/%m/%Y %H:%M:%S"));
    println!("Захардкодзілі /dev/ttyS0 і 9600 бодрэйт");
    loop {
        let now = Local::now();
        let curr_timedate:String = format! ("        {}            {}     ", now.format("%H:%M"), now.format("%d/%m/%Y"));
        match send_to_vfd(curr_timedate) {
            Ok(_) => (),
            Err(err) => println!("Нешта здарылася ў {}.\nВось што: {:#}", now.format("%H:%M:%S"), err)
        }
        sleep(time::Duration::from_secs(20));
    }
}

fn send_to_vfd (curr_timedate: String) -> Result<()> {
    let driver = SerialPortDriver::open("/dev/ttyS0", 9_600, Some(Duration::from_secs(5)))?;
    Printer::new(driver, Protocol::default(), Some(PrinterOptions::default()))
        .page_code(PageCode::PC866)?
        .init()?
        .writeln(&curr_timedate)? //2 радочкаў па 20 сімвалаў
        .print()?;  // print() or print_cut() is mandatory to send the data to the printer
    Ok(())
}