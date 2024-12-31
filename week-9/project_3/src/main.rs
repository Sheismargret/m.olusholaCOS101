use std::fs::File;
use std::io::{self, Write, Read};

fn main() -> io::Result<()> {
    
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let geopolitical_zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // File creation
    let mut file = File::create("convicted_ministers.txt")?;

    writeln!(
        file,
        "{:<5} {:<30} {:<20} {:<15}",
        "S/N", "Name of Commissioner", "Ministry", "Geopolitical Zone"
    )?;
    writeln!(file, "{}", "-".repeat(75))?;

    
    for (i, (commissioner, (ministry, zone))) in commissioners
        .iter()
        .zip(ministries.iter().zip(geopolitical_zones.iter()))
        .enumerate()
    {
        writeln!(
            file,
            "{:<5} {:<30} {:<20} {:<15}",
            i + 1,
            commissioner,
            ministry,
            zone
        )?;
    }

    let mut file = File::open("convicted_ministers.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("Contents of the file:\n{}", contents);

    Ok(())
}

