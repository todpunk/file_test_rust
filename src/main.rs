use clap::{Parser, Subcommand};
use rust_decimal::Decimal;
use std::{collections::HashMap, fs::File, io::{Read, BufReader}, path::Path, mem};
use std::fs;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use uuid::{Uuid, Builder};

pub static DATA_DIRECTORY: &str = "./demo_data/";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(allow_negative_numbers = false)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Raw {
    },
    Buffered {
    },
    Whole {
    },
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Raw {} => {
            process_raw();
        },
        Commands::Buffered {} => {
            process_buffered();
        },
        Commands::Whole {} => {
            process_whole();
        },
    }
}

fn process_raw() {
    let order_products_dir = DATA_DIRECTORY.to_owned()+"order_products/";

    let mut count: u64 = 0;
    let debug_print_num: u64 = 100_000;
    let mut total_price_per: Decimal = Decimal::new(0, 0);
    let mut total_quantity: u64 = 0;

    {
        // Process Order Price
        let price_path: String = order_products_dir.to_owned() + "price_per/";
        let price_size: u64 = mem::size_of::<Decimal>() as u64;
        let mut price_file_num: u64 = 0;
        let mut row_num: u64 = 0;
        println!("{}", "Starting price_per");
        while Path::new(&(price_path.to_owned() + "price_per_" + &format!("{:020}", price_file_num))).is_file() {
            if row_num % debug_print_num == 0 {
                println!("price_per rows: {}", row_num);
            }
            let mut file = OpenOptions::new()
                    .read(true)
                    .open(&(price_path.to_owned() + "price_per_" + &format!("{:020}", price_file_num)))
                    .unwrap();
            let mut buffer: Vec<u8> = vec![0_u8; price_size as usize];
            while (row_num * price_size) < file.metadata().unwrap().len() {
                file.read_exact(&mut buffer).unwrap();
                total_price_per += Decimal::deserialize(buffer[0..16].try_into().unwrap());
                row_num += 1;
                count += 1;
            }
            price_file_num += 1;
        }
        let quantity_path: String = order_products_dir + "quantity/";
        let quantity_size: u64 = mem::size_of::<u64>() as u64;
        let mut quantity_file_num: u64 = 0;
        row_num = 0;
        println!("{}", "Starting quantity");
        while Path::new(&(quantity_path.to_owned() + "quantity_" + &format!("{:020}", quantity_file_num))).is_file() {
            if row_num % debug_print_num == 0 {
                println!("quantity rows: {}", row_num);
            }
            let mut file = OpenOptions::new()
                    .read(true)
                    .open(&(quantity_path.to_owned() + "quantity_" + &format!("{:020}", quantity_file_num)))
                    .unwrap();
            let mut buffer: Vec<u8> = vec![0_u8; quantity_size as usize];
            while (row_num * quantity_size) < file.metadata().unwrap().len() {
                file.read_exact(&mut buffer).unwrap();
                total_quantity += u64::from_be_bytes(buffer[0..8].try_into().unwrap());
                row_num += 1;
                count += 1;
            }
            quantity_file_num += 1;
        }
        println!("Count: {}", count);
        println!("Average Price: {}", total_price_per/Decimal::new(count.try_into().unwrap(), 0));
        println!("Total Price: {}", total_price_per);
        println!("Average Quantity: {}", total_quantity/count);
        println!("Total Quantity: {}", total_quantity);
    }
}

fn process_buffered() {
    let order_products_dir = DATA_DIRECTORY.to_owned()+"order_products/";

    let mut count: u64 = 0;
    let debug_print_num: u64 = 100_000;
    let mut total_price_per: Decimal = Decimal::new(0, 0);
    let mut total_quantity: u64 = 0;

    {
        // Process Order Price
        let price_path: String = order_products_dir.to_owned() + "price_per/";
        let price_size: u64 = mem::size_of::<Decimal>() as u64;
        let mut price_file_num: u64 = 0;
        let mut row_num: u64 = 0;
        println!("{}", "Starting price_per");
        while Path::new(&(price_path.to_owned() + "price_per_" + &format!("{:020}", price_file_num))).is_file() {
            if row_num % debug_print_num == 0 {
                println!("price_per rows: {}", row_num);
            }
            let file = OpenOptions::new()
                    .read(true)
                    .open(&(price_path.to_owned() + "price_per_" + &format!("{:020}", price_file_num)))
                    .unwrap();
            let mut reader = BufReader::new(&file);
            let mut buffer: Vec<u8> = vec![0_u8; price_size as usize];
            while (row_num * price_size) < file.metadata().unwrap().len() {
                reader.read_exact(&mut buffer).unwrap();
                total_price_per += Decimal::deserialize(buffer[0..16].try_into().unwrap());
                row_num += 1;
                count += 1;
            }
            price_file_num += 1;
        }
        let quantity_path: String = order_products_dir + "quantity/";
        let quantity_size: u64 = mem::size_of::<u64>() as u64;
        let mut quantity_file_num: u64 = 0;
        row_num = 0;
        println!("{}", "Starting quantity");
        while Path::new(&(quantity_path.to_owned() + "quantity_" + &format!("{:020}", quantity_file_num))).is_file() {
            if row_num % debug_print_num == 0 {
                println!("quantity rows: {}", row_num);
            }
            let file = OpenOptions::new()
                    .read(true)
                    .open(&(quantity_path.to_owned() + "quantity_" + &format!("{:020}", quantity_file_num)))
                    .unwrap();
            let mut reader = BufReader::new(&file);
            let mut buffer: Vec<u8> = vec![0_u8; quantity_size as usize];
            while (row_num * quantity_size) < file.metadata().unwrap().len() {
                reader.read_exact(&mut buffer).unwrap();
                total_quantity += u64::from_be_bytes(buffer[0..8].try_into().unwrap());
                row_num += 1;
                count += 1;
            }
            quantity_file_num += 1;
        }
        println!("Count: {}", count);
        println!("Average Price: {}", total_price_per/Decimal::new(count.try_into().unwrap(), 0));
        println!("Total Price: {}", total_price_per);
        println!("Average Quantity: {}", total_quantity/count);
        println!("Total Quantity: {}", total_quantity);
    }
}

fn process_whole() {
    let order_products_dir = DATA_DIRECTORY.to_owned()+"order_products/";

    let mut count: u64 = 0;
    let debug_print_num: u64 = 100_000;
    let mut total_price_per: Decimal = Decimal::new(0, 0);
    let mut total_quantity: u64 = 0;

    {
        // Process Order Price
        let price_path: String = order_products_dir.to_owned() + "price_per/";
        let price_size: usize = mem::size_of::<Decimal>();
        let mut price_file_num: u64 = 0;
        let mut row_num: u64 = 0;
        println!("{}", "Starting price_per");
        while Path::new(&(price_path.to_owned() + "price_per_" + &format!("{:020}", price_file_num))).is_file() {
            if row_num % debug_print_num == 0 {
                println!("price_per rows: {}", row_num);
            }
            let mut file = OpenOptions::new()
                    .read(true)
                    .open(&(price_path.to_owned() + "price_per_" + &format!("{:020}", price_file_num)))
                    .unwrap();
            let mut buffer: Vec<u8> = Vec::new();
            let read_bytes = file.read_to_end(&mut buffer).unwrap();
            let mut file_counter: usize = 0;
            while file_counter < read_bytes {
                total_price_per += Decimal::deserialize(buffer[file_counter..file_counter+16].try_into().unwrap());
                row_num += 1;
                count += 1;
                file_counter += price_size;

            }
            price_file_num += 1;
        }
        let quantity_path: String = order_products_dir + "quantity/";
        let quantity_size = mem::size_of::<u64>();
        let mut quantity_file_num: u64 = 0;
        row_num = 0;
        println!("{}", "Starting quantity");
        while Path::new(&(quantity_path.to_owned() + "quantity_" + &format!("{:020}", quantity_file_num))).is_file() {
            if row_num % debug_print_num == 0 {
                println!("quantity rows: {}", row_num);
            }
            let mut file = OpenOptions::new()
                    .read(true)
                    .open(&(quantity_path.to_owned() + "quantity_" + &format!("{:020}", quantity_file_num)))
                    .unwrap();
            let mut buffer: Vec<u8> = Vec::new();
            let read_bytes = file.read_to_end(&mut buffer).unwrap();
            let mut file_counter: usize = 0;
            while file_counter < read_bytes {
                total_quantity += u64::from_be_bytes(buffer[file_counter..file_counter+8].try_into().unwrap());
                row_num += 1;
                count += 1;
                file_counter += quantity_size;

            }
            quantity_file_num += 1;
        }
        println!("Count: {}", count);
        println!("Average Price: {}", total_price_per/Decimal::new(count.try_into().unwrap(), 0));
        println!("Total Price: {}", total_price_per);
        println!("Average Quantity: {}", total_quantity/count);
        println!("Total Quantity: {}", total_quantity);
    }
}
