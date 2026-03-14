// COPYRIGHT: DevClient, a CLI multi tool rust program for general tech purposes.    Copyright (C) 2025  Kevin De Togni, MKlabs
// LICENSE: distribuited on the GNU general public license 3.0v license
// CRATES
use std::io;
use rand::Rng;
use std::process::Command;

// MAIN CODE
fn main() {
    
    // LOGO, CREDITS AND FIST PROMPT
    println!("DevClient, a CLI multi tool rust program for general tech purposes.    Copyright (C) 2025  Kevin De Togni, MKlabs");
    println!("distribuited on the GNU general public license 3.0v license");
    println!();
    println!("      ____             ____ ");
    println!("     /   /            /  _/");
    println!("  __/   /____ __  __ /  /     __   __ ____   _  __ _____");
    println!(" / __  // _ /_\\ \\/ / \\  \\___ / /_ / // _ /_ / \\/ //_  _/");
    println!("/_____/ \\____/ \\__/   \\r___//___//_/ \\____//_/\\_/  /_/");
    println!();
    println!("NOTE: some features only work on UNIX like or UNIX based system!");
    println!();
    println!("select an option: 1 = passgen, 2 = btcadressgen, 3 = cardnumbergen, 4 = nmap local ip scan");
    println!("                  5 = system information, 6 = pkg updater, 7 = install requirements");
    println!();    
    let mut user_first_prompt_choice = String::new();
    io::stdin().read_line(&mut user_first_prompt_choice).expect("failed to read line");
    let user_first_prompt_choice_f: i32 = user_first_prompt_choice.trim().parse().unwrap();
    println!();
    
    // PASSGEN
    if user_first_prompt_choice_f == 1 {
        passgen();
    }
    
    // BTCADDRESSGEN
    else if user_first_prompt_choice_f == 2 {
        btcadressgen();
    }
    
    // CARDNUMBERGEN (in develpment)
    else if user_first_prompt_choice_f == 3 {
        cardnumbergen();
    }
    
    // NMAP LOCAL IP SCAN
    else if user_first_prompt_choice_f == 4 {
        nmap();
    }

    // SYSTEM INFORMATION
    else if user_first_prompt_choice_f == 5 {
        sysinfo();
    }
    
    // PACKAGE UPDATER
    else if user_first_prompt_choice_f == 6 {
        pacupd();
    }

    // INSTALL REQUIREMENTS
    else if user_first_prompt_choice_f == 7 {
        instreq();
    }
    
    // END
    println!();
    println!("press ENTER to exit");
    let mut end = String::new();
    io::stdin().read_line(&mut end).expect("failed to read line");
}

fn passgen() {
    // PASSGEN - TYPE OF PASSWORDS
    println!("what type of passwords do you want to generate? 1 = numbers only, 2 = letters only, 3 = numbers and letters");
    println!();
    let mut user_passgen_choice: String = String::new();
    io::stdin().read_line(&mut user_passgen_choice).expect("failed to read line");
    let user_passgen_choice_f: i32 = user_passgen_choice.trim().parse().unwrap();
        
    // PASSGEN - NUMBER OF PASSWORDS
    println!();
    println!("how many passwords do you want to generate?");
    println!();
    let mut user_passgen_password_number_choice = String::new();
    io::stdin().read_line(&mut user_passgen_password_number_choice).expect("failed to read input");
    let user_passgen_password_number_choice_f: i32 = user_passgen_password_number_choice.trim().parse().unwrap();
    println!();
        
    // PASSGEN - LENGTH OF PASSWORDS
    println!("how long do you want them to be?");
    println!();
    let mut user_passgen_password_lenght_choice = String::new();
    io::stdin().read_line(&mut user_passgen_password_lenght_choice).expect("failed to read line");
    let user_passgen_password_lenght_choice_f = user_passgen_password_lenght_choice.trim().parse().unwrap();
    println!();
        
    // PASSGEN - NUMBERS ONLY
    if user_passgen_choice_f == 1 {
        for _i in 0..user_passgen_password_number_choice_f {
            for _ in 0..user_passgen_password_lenght_choice_f {
                let  rng = rand::thread_rng().gen_range(0..=9);
                print!("{}", rng);
            }
            println!();
        }
    }
        
    // PASSGEN - LETTERS ONLY
    else if user_passgen_choice_f == 2 {
        let letters = b"qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";
        let  mut rng = rand::thread_rng();
        println!();
        for _i in 0..user_passgen_password_number_choice_f {
            let password_letters_only: String = (0..user_passgen_password_lenght_choice_f).map(|_| {
                let idx = rng.gen_range(0..letters.len());
                letters[idx] as char
            })
            .collect();
            println!("{}", password_letters_only);
        }
    }
        
    // PASSGEN - NUMBERS AND LETTERS
    else if user_passgen_choice_f == 3 {
        let letters_and_numbers = b"qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890";
        let mut rng = rand::thread_rng();
        println!();
        for _i in 0..user_passgen_password_number_choice_f {
            let password_letters_and_numbers: String = (0..user_passgen_password_lenght_choice_f).map(|_| {
                let idx = rng.gen_range(0..letters_and_numbers.len());
                letters_and_numbers[idx] as char
            })
            .collect();
            println!("{}", password_letters_and_numbers);
        }
    }
}

fn btcadressgen() {
    // BTCADDRESSGEN - NUMBER OF ADDRESSES
    println!("how many adresses do you want to generate?");
    println!();
    let mut user_btcaddressgen_password_number_choice = String::new();
    io::stdin().read_line(&mut user_btcaddressgen_password_number_choice).expect("failed to read line");
    let user_btcaddressgen_password_number_choice_f: i32 = user_btcaddressgen_password_number_choice.trim().parse().unwrap();
    let mut rng = rand::thread_rng();
    println!();
        
    // BTCADDRESSGEN - ADDRESS TYPE
    let characters_base58 = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let characters_bech32 = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";
    println!("what type do you wanna use? 1 = Legacy (P2PKH), 2 = P2SH, 3 = Bech32 (P2WPKH), 4 = Bech32 (P2WSH), 5 = Bech32m (Taproot)");
    println!();
    let mut user_bitcoin_type_choice = String::new();
    io::stdin().read_line(&mut user_bitcoin_type_choice).expect("failed to read line");
    let user_bitcoin_type_choice_f:i32 = user_bitcoin_type_choice.trim().parse().unwrap();
    println!();
        
    // BTCADDRESSGEN - LEGACY (P2PKH)
    if user_bitcoin_type_choice_f == 1 {
        for _i in 0..user_btcaddressgen_password_number_choice_f {
            print!("1");
            let random_adddres_base58: String = (0..=34).map(|_| {
                let idx = rng.gen_range(0..characters_base58.len());
                characters_base58[idx] as char
            })
            .collect();
            println!("{}", random_adddres_base58);
        }
    }
        
    // BTCADDRESSGEN - P2SH
    if user_bitcoin_type_choice_f == 2 {
        for _i in 0..user_btcaddressgen_password_number_choice_f {
            print!("3");
            let random_adddres_base58: String = (0..=34).map(|_| {
                let idx = rng.gen_range(0..characters_base58.len());
                characters_base58[idx] as char
            })
            .collect();
            println!("{}", random_adddres_base58);
        }
    }
        
    // BTCADDRESSGEN - BECH32 (P2WPKH)
    if user_bitcoin_type_choice_f == 3 {
        for _i in 0..user_btcaddressgen_password_number_choice_f {
            print!("bc1");
            let random_adddres_bech32: String = (0..=39).map(|_| {
                let idx = rng.gen_range(0..characters_bech32.len());
                characters_bech32[idx] as char
            })
            .collect();
            println!("{}", random_adddres_bech32);
        }
    }
        
    // BTCADDRESSGEN - BECH32 (P2WSH)
    if user_bitcoin_type_choice_f == 4 {
        for _i in 0..user_btcaddressgen_password_number_choice_f {
            print!("bc1");
            let random_adddres_bech32: String = (0..=59).map(|_| {
                let idx = rng.gen_range(0..characters_bech32.len());
                characters_bech32[idx] as char
            })
            .collect();
            println!("{}", random_adddres_bech32);
        }
    }
        
    // BTCADDRESSGEN - BECH32M (TAPROOT)
    if user_bitcoin_type_choice_f == 5 {
        for _i in 0..user_btcaddressgen_password_number_choice_f {
            print!("bc1p");
            let random_adddres_bech32: String = (0..=58).map(|_| {
                let idx = rng.gen_range(0..characters_bech32.len());
                characters_bech32[idx] as char
            })
            .collect();
            println!("{}", random_adddres_bech32);
        }
    }
}

fn cardnumbergen() { //FIX
    let cardnumbergen_card_number_1 = rand::thread_rng().gen_range(000..999);
    let cardnumbergen_card_number_2 = rand::thread_rng().gen_range(0000..9999);
    let cardnumbergen_card_number_3 = rand::thread_rng().gen_range(0000..9999);
    let cardnumbergen_card_number_4 = rand::thread_rng().gen_range(0000..9999);
    let cardnumbergen_card_expiration_month = rand::thread_rng().gen_range(01..=12);
    let cardnumbergen_card_expiration_year = rand::thread_rng().gen_range(2026..2036);
    let cardnumbergen_card_cvv = rand::thread_rng().gen_range(000..=999);
    let mastercard_number_1 = rand::thread_rng().gen_range(2221..2720);
    let american_express_number_1 = rand::thread_rng().gen_range(00..99);
    let american_express_number_2 = rand::thread_rng().gen_range(000000..999999);
    let american_express_number_3 = rand::thread_rng().gen_range(00000..99999);
        
    // CARDNUMBERGEN - CARD TYPE
    println!("enter card to generate: 1 = Visa, 2 = Mastercard, 3 = American Express.");
    println!();
    let mut user_card_type = String::new();
    io::stdin().read_line(&mut user_card_type).expect("failed to read line");
    let user_card_type_f:i32 = user_card_type.trim().parse().unwrap();
    println!();
        
    // CARDNUMBERGEN - NUMBER OF GENERATED CARDS
    println!("how many cards do you wanna generate?");
    println!();
    let mut cardnumbergen_number_of_cards = String::new();
    io::stdin().read_line(&mut cardnumbergen_number_of_cards).expect("failed to read line");
    let cardnumbergen_number_of_cards_f:i32 = cardnumbergen_number_of_cards.trim().parse().unwrap();
    println!();
    if user_card_type_f == 1 {
        for _i in 0..cardnumbergen_number_of_cards_f {
            
            // CARDNUMBERGEN - VISA
            println!("--------------------------------");
            println!("Visa");
            println!();
            println!("card number: 4{}-{}-{}-{}", cardnumbergen_card_number_1, cardnumbergen_card_number_2, cardnumbergen_card_number_3, cardnumbergen_card_number_4 );
            println!();
            println!("expirarion date: {}/{}", cardnumbergen_card_expiration_month, cardnumbergen_card_expiration_year);
            println!();
            println!("cvv: {}", cardnumbergen_card_cvv);
            println!("--------------------------------");
        }
    }
    else if user_card_type_f == 2 {
        for _i in 0..cardnumbergen_number_of_cards_f {
                
            // CARDNUMBERGEN - MASTERCARD
            println!("--------------------------------");
            println!("MasterCard");
            println!();
            println!("card number: {}-{}-{}-{}", mastercard_number_1, cardnumbergen_card_number_2, cardnumbergen_card_number_3, cardnumbergen_card_number_4);
            println!();
            println!("expiration date: {}/{}", cardnumbergen_card_expiration_month, cardnumbergen_card_expiration_year);
            println!();
            println!("cvv: {}", cardnumbergen_card_cvv);
            println!("--------------------------------");
        }
    }
    else if user_card_type_f == 3 {
        for _i in 0..cardnumbergen_number_of_cards_f {
                
            // CARDNUMBERGEN - AMERICAN EXPRESS
            println!("-------------------------------");
            println!("American Express");
            println!();
            println!("card number: 37{}-{}-{}", american_express_number_1, american_express_number_2, american_express_number_3);
            println!();
            println!("expiration date: {}/{}", cardnumbergen_card_expiration_month, cardnumbergen_card_expiration_year);
            println!();
            println!("cvv: {}", cardnumbergen_card_cvv);
            println!("-------------------------------");
        }
    }
}

fn nmap() {
    // NMAP - PORT
    println!("select port: 1 = 24, 2 = 16 (the port 16 takes longer to scan than the port 24");
    println!();
    let mut user_nmap_port_selection = String::new();
    io::stdin().read_line(&mut user_nmap_port_selection).expect("failed to read line");
    let user_nmap_port_selection_f:i32 = user_nmap_port_selection.trim().parse().unwrap();
    println!();
        
    // NMAP - PORT 24
    if user_nmap_port_selection_f == 1 {
        let mut sh = Command::new("sh");
        sh.arg("nmap24.sh");
        match sh.output() {
            Ok(o) => {
                unsafe {
                    println!("{}", String::from_utf8_unchecked(o.stdout));
                }
            }
            Err(e) => {
                println!("{}",e);
            }
        }
    }
        
    // NMAP - PORT 16
    else if user_nmap_port_selection_f == 2 {
        let mut sh = Command::new("sh");
        sh.arg("nmap16.sh");
        match sh.output() {
            Ok(o) => {
                unsafe {
                    println!("{}", String::from_utf8_unchecked(o.stdout));
                }
            }
            Err(e) => {
                println!("{}",e);
            }
        }
    }
}

fn sysinfo() {
    let mut sh = Command::new("sh");
    sh.arg("fastfetch.sh");
    match sh.output() {
        Ok(o) => {
            unsafe {
                println!("{}", String::from_utf8_unchecked(o.stdout));
            }
        }
        Err(e) => {
            println!("{}",e);
        }
    }
}

fn pacupd () {
    // PACKAGE UPDATER - PACKAGE MANAGER
    println!("select package manager: 1 = apt, 2 = dnf, 3 = pacman");
    println!();
    let mut user_package_manager = String::new();
    io::stdin().read_line(&mut user_package_manager).expect("failed to read line");
    let user_package_manager_f: i32 = user_package_manager.trim().parse().unwrap();
    println!();
        
    // PACKAGE UPDATER - DEBIAN
    if user_package_manager_f == 1 {
        let mut sh = Command::new("sh");
        sh.arg("apt.sh");
        match sh.output() {
            Ok(o) => {
                unsafe {
                    println!("{}", String::from_utf8_unchecked(o.stdout));
                }
            }
            Err(e) => {
                println!("{}",e);
            }
        }
    }
        
    // PACKAGE UPDATER - FEDORA
    else if user_package_manager_f == 2 {
        let mut sh = Command::new("sh");
        sh.arg("dnf.sh");
        match sh.output() {
            Ok(o) => {
                unsafe {
                    println!("{}", String::from_utf8_unchecked(o.stdout));
                }
            }
            Err(e) => {
                println!("{}",e);
            }
        }
    }
        
    // PACKAGE UPDATER - ARCH
    else if user_package_manager_f == 3 {
        let mut sh = Command::new("sh");
        sh.arg("pacman.sh");
        match sh.output() {
            Ok(o) => {
                unsafe {
                    println!("{}", String::from_utf8_unchecked(o.stdout));
                }
            }
            Err(e) => {
                println!("{}",e);
            }
        }
    }
}

fn instreq() {
    let mut user_package_manager = String::new();
    println!("enter package manager: 1 = apt, 2 = dnf, 3 = pacman");
    io::stdin().read_line(&mut user_package_manager).expect("failed to read line");
    let user_package_manager_f:i32 = user_package_manager.trim().parse().unwrap();
    if user_package_manager_f == 1 {
        let mut sh = Command::new("sh");
        sh.arg("installrequirements-apt.sh");
        match sh.output() {
            Ok(o) => {
                unsafe {
                    println!("{}", String::from_utf8_unchecked(o.stdout));
                }
            }
            Err(e) => {
                println!("{}",e);
            }
        }
    }

    else if user_package_manager_f == 2 {
        let mut sh = Command::new("sh");
        sh.arg("installrequirements-dnf.sh");
        match sh.output() {
            Ok(o) => {
                unsafe {
                    println!("{}", String::from_utf8_unchecked(o.stdout));
                }
            }
            Err(e) => {
                println!("{}",e);
            }
        }
    }

    else if user_package_manager_f == 3 {
        let mut sh = Command::new("sh");
        sh.arg("installrequirements-pacman.sh");
        match sh.output() {
            Ok(o) => {
                unsafe {
                    println!("{}", String::from_utf8_unchecked(o.stdout));
                }
            }
            Err(e) => {
                println!("{}",e);
            }
        }
    }
}