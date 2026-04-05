// COPYRIGHT: DevClient, a CLI multi tool rust program for general tech purposes.    Copyright (C) 2025  Kevin De Togni, mrmaxxarch
// LICENSE: distribuited on the GNU general public license 3.0v license
// CRATES
use std::io;
use rand::Rng;
use std::process::Command;

// MAIN CODE
fn main() {
    
    // LOGO, CREDITS AND FIST PROMPT
    println!("DevClient, a CLI multi tool rust program for general tech purposes.    Copyright (C) 2025  Kevin De Togni, mrmaxxarch");
    println!("distribuited on the GNU general public license 3.0v license");
    println!();
    println!("      ____             ____ ");
    println!("     /   /            /  _/");
    println!("  __/   /____ __  __ /  /     __   __ ____   _  __ _____");
    println!(" / __  // _ /_\\ \\/ / \\  \\___ / /_ / // _ /_ / \\/ //_  _/");
    println!("/_____/ \\____/ \\__/   \\r___//___//_/ \\____//_/\\_/  /_/  ©");
    println!();
    println!("[ INFO ]: some features only work on UNIX like or UNIX based system!");
    println!();
    println!("[ CONSOLE ]: select an option: 1 = PasswordGenerator, 2 = BitcoinAddressGenerator, 3 = cardnumbergen");
    println!("                               4 = nmap local ip scan, 5 = system information, 6 = pkg updater");
    println!();    
    let mut UserFirstPromptChoice = String::new();
    io::stdin().read_line(&mut UserFirstPromptChoice).expect("failed to read line");
    let UserFirstPromptChoiceF: i32 = UserFirstPromptChoice.trim().parse().unwrap();
    println!();
    
    // PASSGEN
    if UserFirstPromptChoiceF == 1 {
        PasswordGenerator();
    }
    
    // BTCADDRESSGEN
    else if UserFirstPromptChoiceF == 2 {
        BitcoinAddressGenerator();
    }
    
    // CARDNUMBERGEN (in develpment)
    else if UserFirstPromptChoiceF == 3 {
        cardnumbergen();
    }
    
    // NMAP LOCAL IP SCAN
    else if UserFirstPromptChoiceF == 4 {
        nmap();
    }

    // SYSTEM INFORMATION
    else if UserFirstPromptChoiceF == 5 {
        sysinfo();
    }
    
    // PACKAGE UPDATER
    else if UserFirstPromptChoiceF == 6 {
        pacupd();
    }
    // END
    println!();
    println!("press ENTER to exit");
    let mut end = String::new();
    io::stdin().read_line(&mut end).expect("failed to read line");
}

fn PasswordGenerator() {
    // PASSGEN - TYPE OF PASSWORDS
    println!("[ CONSOLE ]: what type of passwords do you want to generate? 1 = numbers only, 2 = letters only, 3 = numbers and letters");
    println!();
    let mut UserPasswordGeneratorChoice: String = String::new();
    io::stdin().read_line(&mut UserPasswordGeneratorChoice).expect("failed to read line");
    let UserPasswordGeneratorChoiceF: i32 = UserPasswordGeneratorChoice.trim().parse().unwrap();
        
    // PASSGEN - NUMBER OF PASSWORDS
    println!();
    println!("[ CONSOLE ]: how many passwords do you want to generate?");
    println!();
    let mut UserPasswordGeneratorPasswordNumberChoice = String::new();
    io::stdin().read_line(&mut UserPasswordGeneratorPasswordNumberChoice).expect("failed to read input");
    let UserPasswordGeneratorPasswordNumberChoiceF: i32 = UserPasswordGeneratorPasswordNumberChoice.trim().parse().unwrap();
    println!();
        
    // PASSGEN - LENGTH OF PASSWORDS
    println!("[ CONSOLE ]: how long do you want them to be?");
    println!();
    let mut UserPasswordGeneratorPasswordLenghtChoice = String::new();
    io::stdin().read_line(&mut UserPasswordGeneratorPasswordLenghtChoice).expect("failed to read line");
    let UserPasswordGeneratorPasswordLenghtChoiceF = UserPasswordGeneratorPasswordLenghtChoice.trim().parse().unwrap();
    println!();
        
    // PASSGEN - NUMBERS ONLY
    if UserPasswordGeneratorChoiceF == 1 {
        for _i in 0..UserPasswordGeneratorPasswordNumberChoiceF {
            print!("[ OUTPUT ]: ");
            for _ in 0..UserPasswordGeneratorPasswordLenghtChoiceF {
                let  rng = rand::thread_rng().gen_range(0..=9);
                print!("{}", rng);
            }
            println!();
        }
    }
        
    // PASSGEN - LETTERS ONLY
    else if UserPasswordGeneratorChoiceF == 2 {
        let letters = b"qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";
        let  mut rng = rand::thread_rng();
        println!();
        for _i in 0..UserPasswordGeneratorPasswordNumberChoiceF {
            print!("[ OUTPUT ]: ");
            let password_letters_only: String = (0..UserPasswordGeneratorPasswordLenghtChoiceF).map(|_| {
                let idx = rng.gen_range(0..letters.len());
                letters[idx] as char
            })
            .collect();
            println!("{}", password_letters_only);
        }
    }
        
    // PASSGEN - NUMBERS AND LETTERS
    else if UserPasswordGeneratorChoiceF == 3 {
        let LettersAndNumbers = b"qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890";
        let mut rng = rand::thread_rng();
        println!();
        for _i in 0..UserPasswordGeneratorPasswordNumberChoiceF {
            print!("[ OUTPUT ]: ");
            let PasswordLettersAndNumbers: String = (0..UserPasswordGeneratorPasswordLenghtChoiceF).map(|_| {
                let idx = rng.gen_range(0..LettersAndNumbers.len());
                LettersAndNumbers[idx] as char
            })
            .collect();
            println!("{}", PasswordLettersAndNumbers);
        }
    }
}

fn BitcoinAddressGenerator() {
    // BTCADDRESSGEN - NUMBER OF ADDRESSES
    println!("[ CONSOLE ]: how many adresses do you want to generate?");
    println!();
    let mut UserBitcoinAddressGeneratorPasswordNumberChoice = String::new();
    io::stdin().read_line(&mut UserBitcoinAddressGeneratorPasswordNumberChoice).expect("failed to read line");
// FIX: restart variables replacing from here with sed command
    let UserBitcoinAddressGeneratorPasswordNumberChoice_f: i32 = UserBitcoinAddressGeneratorPasswordNumberChoice.trim().parse().unwrap();
    let mut rng = rand::thread_rng();
    println!();
        
    // BTCADDRESSGEN - ADDRESS TYPE
    let characters_base58 = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let characters_bech32 = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";
    println!("[ CONSOLE ]: what type do you wanna use? 1 = Legacy (P2PKH), 2 = P2SH, 3 = Bech32 (P2WPKH), 4 = Bech32 (P2WSH), 5 = Bech32m (Taproot)");
    println!();
    let mut user_bitcoin_type_choice = String::new();
    io::stdin().read_line(&mut user_bitcoin_type_choice).expect("failed to read line");
    let user_bitcoin_type_choice_f:i32 = user_bitcoin_type_choice.trim().parse().unwrap();
    println!();
        
    // BTCADDRESSGEN - LEGACY (P2PKH)
    if user_bitcoin_type_choice_f == 1 {
        for _i in 0..UserBitcoinAddressGeneratorPasswordNumberChoice_f {
            print!("[ OUTPUT ]: ");
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
        for _i in 0..UserBitcoinAddressGeneratorPasswordNumberChoice_f {
            print!("[ OUTPUT ]: ");
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
        for _i in 0..UserBitcoinAddressGeneratorPasswordNumberChoice_f {
            print!("[ OUTPUT ]: ");
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
        for _i in 0..UserBitcoinAddressGeneratorPasswordNumberChoice_f {
            print!("[ OUTPUT ]: ");
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
        for _i in 0..UserBitcoinAddressGeneratorPasswordNumberChoice_f {
            print!("[ OUTPUT ]: ");
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

fn cardnumbergen() {
        
    // CARDNUMBERGEN - CARD TYPE
    println!("[ CONSOLE ]: enter card to generate: 1 = Visa, 2 = Mastercard, 3 = American Express.");
    println!();
    let mut user_card_type = String::new();
    io::stdin().read_line(&mut user_card_type).expect("failed to read line");
    let user_card_type_f:i32 = user_card_type.trim().parse().unwrap();
    println!();
        
    // CARDNUMBERGEN - NUMBER OF GENERATED CARDS
    println!("[ CONSOLE ]: how many cards do you wanna generate?");
    println!();
    let mut cardnumbergen_number_of_cards = String::new();
    io::stdin().read_line(&mut cardnumbergen_number_of_cards).expect("failed to read line");
    let cardnumbergen_number_of_cards_f:i32 = cardnumbergen_number_of_cards.trim().parse().unwrap();
    println!();
    if user_card_type_f == 1 {
        for _i in 0..cardnumbergen_number_of_cards_f {
            
            // CARDNUMBERGEN - VISA
            let cardnumbergen_card_number_1 = rand::thread_rng().gen_range(000..999);
            let cardnumbergen_card_number_2 = rand::thread_rng().gen_range(0000..9999);
            let cardnumbergen_card_number_3 = rand::thread_rng().gen_range(0000..9999);
            let cardnumbergen_card_number_4 = rand::thread_rng().gen_range(0000..9999);
            let cardnumbergen_card_expiration_month = rand::thread_rng().gen_range(01..=12);
            let cardnumbergen_card_expiration_year = rand::thread_rng().gen_range(2026..2036);
            let cardnumbergen_card_cvv = rand::thread_rng().gen_range(000..=999);
            println!("[ OUTPUT ]: --------------------------------");
            println!("[ OUTPUT ]:              Visa");
            println!("[ OUTPUT ]: ");
            println!("[ OUTPUT ]: card number: 4{}-{}-{}-{}", cardnumbergen_card_number_1, cardnumbergen_card_number_2, cardnumbergen_card_number_3, cardnumbergen_card_number_4 );
            println!("[ OUTPUT ]: ");
            println!("[ OUTPUT ]: expirarion date: {}/{}", cardnumbergen_card_expiration_month, cardnumbergen_card_expiration_year);
            println!("[ OUTPUT ]: ");
            println!("[ OUTPUT ]: cvv: {}", cardnumbergen_card_cvv);
            println!("[ OUTPUT ]: --------------------------------");
        }
    }
    else if user_card_type_f == 2 {
        for _i in 0..cardnumbergen_number_of_cards_f {
                
            // CARDNUMBERGEN - MASTERCARD
            let cardnumbergen_card_number_2 = rand::thread_rng().gen_range(0000..9999);
            let cardnumbergen_card_number_3 = rand::thread_rng().gen_range(0000..9999);
            let cardnumbergen_card_number_4 = rand::thread_rng().gen_range(0000..9999);
            let mastercard_number_1 = rand::thread_rng().gen_range(2221..2720);
            let cardnumbergen_card_expiration_month = rand::thread_rng().gen_range(01..=12);
            let cardnumbergen_card_expiration_year = rand::thread_rng().gen_range(2026..2036);
            let cardnumbergen_card_cvv = rand::thread_rng().gen_range(000..=999);
            println!("[ OUTPUT ]: --------------------------------");
            println!("[ OUTPUT ]:           MasterCard");
            println!("[ OUTPUT ]: ");
            println!("[ OUTPUT ]: card number: {}-{}-{}-{}", mastercard_number_1, cardnumbergen_card_number_2, cardnumbergen_card_number_3, cardnumbergen_card_number_4);
            println!("[ OUTPUT ]: ");
            println!("[ OUTPUT ]: expiration date: {}/{}", cardnumbergen_card_expiration_month, cardnumbergen_card_expiration_year);
            println!("[ OUTPUT ]: ");
            println!("[ OUTPUT ]: cvv: {}", cardnumbergen_card_cvv);
            println!("[ OUTPUT ]: --------------------------------");
        }
    }
    else if user_card_type_f == 3 {
        for _i in 0..cardnumbergen_number_of_cards_f {
                
            // CARDNUMBERGEN - AMERICAN EXPRESS
            let american_express_number_1 = rand::thread_rng().gen_range(00..99);
            let american_express_number_2 = rand::thread_rng().gen_range(000000..999999);
            let american_express_number_3 = rand::thread_rng().gen_range(00000..99999);
            let cardnumbergen_card_expiration_month = rand::thread_rng().gen_range(01..=12);
            let cardnumbergen_card_expiration_year = rand::thread_rng().gen_range(2026..2036);
            let cardnumbergen_card_cvv = rand::thread_rng().gen_range(000..=999);
            println!("[ OUTPUT ]: -------------------------------");
            println!("[ OUTPUT ]:        American Express");
            println!("[ OUTPUT ]: ");
            println!("[ OUTPUT ]: card number: 37{}-{}-{}", american_express_number_1, american_express_number_2, american_express_number_3);
            println!("[ OUTPUT ]: ");
            println!("[ OUTPUT ]: expiration date: {}/{}", cardnumbergen_card_expiration_month, cardnumbergen_card_expiration_year);
            println!("[ OUTPUT ]: ");
            println!("[ OUTPUT ]: cvv: {}", cardnumbergen_card_cvv);
            println!("[ OUTPUT ]: -------------------------------");
        }
    }
}

fn nmap() {
    // NMAP - PORT
    println!("[ CONSOLE ]: select port: 1 = 24, 2 = 16 (the port 16 takes longer to scan than the port 24");
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
                    println!("[ OUTPUT ]: {}", String::from_utf8_unchecked(o.stdout));
                }
            }
            Err(e) => {
                println!("[ ERROR ]: {}",e);
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
                    println!("[ OUTPUT ]: {}", String::from_utf8_unchecked(o.stdout));
                }
            }
            Err(e) => {
                println!("[ ERROR ]:{}",e);
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
                println!("[ OUTPUT ]: {}", String::from_utf8_unchecked(o.stdout));
            }
        }
        Err(e) => {
            println!("[ ERROR ]: {}",e);
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
                    println!("[ OUTPUT ]: {}", String::from_utf8_unchecked(o.stdout));
                }
            }
            Err(e) => {
                println!("[ ERROR ]: {}",e);
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
                    println!("[ OUTPUT ]: {}", String::from_utf8_unchecked(o.stdout));
                }
            }
            Err(e) => {
                println!("[ ERROR ]: {}",e);
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
                    println!("[ OUTPUT ]: {}", String::from_utf8_unchecked(o.stdout));
                }
            }
            Err(e) => {
                println!("[ ERROR ]: {}",e);
            }
        }
    }
}