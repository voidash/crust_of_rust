use futures::executor::block_on;
use std::{pin::Pin, fmt::format};


// async fn fetch_beers() -> Result<(),Box<dyn std::error::Error>> {
//  let res = reqwest::get("https://api.openbrewerydb.org/v1/breweries?per_page=3").await?;
//     println!("status: {}", res.status());
//     println!("headers: \n{:?}", res.headers());
//     let body = res.text().await?;
//     println!("body: \n{}",body);
//     Ok(())
// }

fn exchange_drive(drive:&mut Drive) -> Drive{
    let defined_drive = Drive {
        size: 32768,
        letter: 'c',
        file_system : format!("ntfs")
    };
    std::mem::replace(drive, defined_drive)
}

#[derive(Debug,Clone)]
struct Drive {
    size: usize,
    letter: char,
    file_system: String
}

fn pin_test() {
    let mut my_drive = Drive {
        size: 7823,
        letter: 'd',
        file_system: format!("fat32")
    };
    let defined_drive = Drive {
        size: 32768,
        letter: 'c',
        file_system : format!("ntfs")
    };
    let pinned_drive = Pin::new(&mut my_drive);

    // let c = exchange_drive(pinned_drive.get_mut());
    // let e = std::mem::replace(pinned_drive.get_mut(),defined_drive);
    // let e = std::mem::replace(pinned_drive.get_mut(), defined_drive);
    let d = pinned_drive.get_mut();
    d.file_system = format!("jvke");
    d.size = 2334;
    let c = d;
    c.file_system = format!("ssd");
    c.size = 33433;


    println!("{:?}", my_drive);
    // println!("{:?}",e);
}
