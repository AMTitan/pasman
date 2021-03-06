use directories::BaseDirs;
use std::path::Path;
use std::fs::File;
use clap::*;
use std::time::SystemTime;
use std::io::{Write, Read};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead};
use rand::seq::SliceRandom;
use std::str::FromStr;
use std::time::Duration;
use humantime::format_duration;
use std::fs;
use prettytable::{Table, Row, Cell};

fn main() {
    let matches = App::new("pasman")
        .version("0.01")
        .about("A password manager")
        .arg(
            Arg::with_name("test")
                .short("t")
                .long("test")
                .help("test to see how many passwords you can crack per second")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .long("password")
                .value_name("master password")
                .help("pass through your password")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("lists all the passwords you have")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("find")
                .short("f")
                .long("find")
                .value_name("account name")
                .help("find a password")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("create")
                .short("c")
                .long("create")
                .help("make a password")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("generate")
                .short("g")
                .long("generate")
                .value_name("number")
                .help("generates a password with the x chars long")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("delete")
                .value_name("account name")
                .help("delete a password")
                .takes_value(true),
        )
        .get_matches();
    if matches.is_present("test") {
        let mut num = 0;
        let now = SystemTime::now();
        let nonce = Nonce::from_slice(b"unique nonce");
        let password = format!("{:x}", md5::compute("one"));
        let key = Key::from_slice(password.as_ref());
        let cipher = Aes256Gcm::new(key);
        let ciphertext = cipher.encrypt(nonce, b"billys passwords".as_ref())
            .expect("encryption failure!"); 
        while now.elapsed().unwrap().as_millis() < 1000 {
            let password = format!("{:x}", md5::compute(num.to_string()));
            let key = Key::from_slice(password.as_ref());
            let cipher = Aes256Gcm::new(key);
            cipher.decrypt(nonce, ciphertext.as_ref());
            num += 1;
        }
        println!("your computer can do {} hashes per second (Some computers can do WAY more than this, and your password could already be hashed so it would take no time even if it was really long, so pick a password that NO ONE has ever used)\n1 chars = {}\n2 chars = {}\n3 chars = {}\n4 chars = {}\n5 chars = {}\n6 chars = {}\n7 chars = {}\n8 chars = {}\n9 chars = {}\n10 chars = {}\n11 chars = {}\n12 chars = {}\n13 chars = {}\n14 chars = {}\n15 chars = {}\n16 chars = {}\n17 chars = {}\n18 chars = {}", num, format_duration(Duration::new((72_u128.pow(1) / num) as u64, 0)).to_string(), format_duration(Duration::new((72_u128.pow(2) / num) as u64, 0)).to_string(), format_duration(Duration::new((72_u128.pow(3) / num) as u64, 0)).to_string(), format_duration(Duration::new((72_u128.pow(4) / num) as u64, 0)).to_string(), format_duration(Duration::new((72_u128.pow(5) / num) as u64, 0)).to_string(), format_duration(Duration::new((72_u128.pow(6) / num) as u64, 0)).to_string(), format_duration(Duration::new((72_u128.pow(7) / num) as u64, 0)).to_string(), format_duration(Duration::new((72_u128.pow(8) / num) as u64, 0)).to_string(), format_duration(Duration::new((72_u128.pow(9) / num) as u64, 0)).to_string(), format_duration(Duration::new((72_u128.pow(10) / num) as u64, 0)).to_string(), format_duration(Duration::new((72_u128.pow(11) / num) as u64, 0)).to_string(), format_duration(Duration::new((72_u128.pow(12) / num) as u64, 0)).to_string(), format_duration(Duration::new((72_u128.pow(13) / num) as u64, 0)).to_string(), format_duration(Duration::new((72_u128.pow(14) / num) as u64, 0)).to_string(), format_duration(Duration::new((72_u128.pow(15) / num) as u64, 0)).to_string(), format_duration(Duration::new((72_u128.pow(16) / num) as u64, 0)).to_string(), format_duration(Duration::new((72_u128.pow(17) / num) as u64, 0)).to_string(), format_duration(Duration::new((72_u128.pow(18) / num) as u64, 0)).to_string());
    }
    else if matches.is_present("generate") {
        let chars = vec!['0','1','2','3','4','5','6','7','8','9','!','@','#','$','%','^','&','*','(',')','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
        let mut password:String = String::new();
        for _x in 0..FromStr::from_str(matches.value_of("generate").unwrap()).unwrap() {
            password.push_str(chars.choose(&mut rand::thread_rng()).unwrap().to_string().as_str())
        }
        println!("{}", password);
    }
    else {
        if let Some(base_dirs) = BaseDirs::new() {
            if !Path::new(&[base_dirs.config_dir().to_str().unwrap(), "/pas.man"].join("")).exists() {
                File::create(&[base_dirs.config_dir().to_str().unwrap(), "/pas.man"].join(""));
            }
            let mut line = String::new();
            if matches.is_present("password") {
                line = matches.value_of("password").unwrap().trim().to_string();
            }
            else {
                println!("What is your master password? (you can run the --test to see how long it would take to crack the password)");
                std::io::stdin().read_line(&mut line).unwrap();
                line = line.trim().to_string();
            }
            if line.contains(" ") {
                println!("sorry your master password can not have a space in it.");
                std::process::exit(1);
            }
            let password = format!("{:x}", md5::compute(line));
            let key = Key::from_slice(password.as_ref());
            let cipher = Aes256Gcm::new(key);
            let nonce = Nonce::from_slice(b"unique nonce");
            let mut contents = String::new();
            File::open(&[base_dirs.config_dir().to_str().unwrap(), "/pas.man"].join("")).unwrap().read_to_string(&mut contents);
            if contents.len() == 0 {
                println!("Lets add a password what do you want the name to be?");
                let mut new_accont:String = "".to_string();
                std::io::stdin().read_line(&mut new_accont).unwrap();
                new_accont = new_accont.trim().to_string();
                if new_accont.contains(" ") {
                    println!("sorry your account cant have a space in it try something like www.google.com");
                    std::io::stdin().read_line(&mut new_accont).unwrap();
                    new_accont = new_accont.trim().to_string();
                }
                println!("Lets add a password to it");
                let mut new_pass:String = "".to_string();
                std::io::stdin().read_line(&mut new_pass).unwrap();
                new_pass = new_pass.trim().to_string();
                if new_accont.contains(" ") {
                    println!("sorry your password can not have a space in it");
                    std::io::stdin().read_line(&mut new_pass).unwrap();
                    new_pass = new_pass.trim().to_string();
                }
                let ciphertext = cipher.encrypt(nonce, [new_accont, " : ".to_string(), new_pass].join("").as_ref())
                    .expect("encryption failure!"); 
                write_file([base_dirs.config_dir().to_str().unwrap(), "/pas.man"].join(""), hex::encode(ciphertext));
            }
            else {
                let decoded = hex::decode(contents.clone()).unwrap();
                cipher.decrypt(nonce, decoded.as_ref())
                    .expect("decryption failure!");
                let mut type_pick: String = "".to_string();
                if !matches.is_present("find") && !matches.is_present("create") && !matches.is_present("list") && !matches.is_present("delete") {
                    println!("Type the one you want\n1: add a password\n2: find a password\n3: list all passwords (UNSECURE)\n4: Delete a password\nshift+5 (%): Delete all passwords");
                    std::io::stdin().read_line(&mut type_pick).unwrap();
                    type_pick = type_pick.trim().to_string();
                }
                if matches.is_present("find") {
                    type_pick = "2".to_string();
                }
                if matches.is_present("create") {
                    type_pick = "1".to_string();
                }
                if matches.is_present("list") {
                    type_pick = "3".to_string();
                }
                if matches.is_present("delete") {
                    type_pick = "4".to_string();
                }
                if !["1", "2", "3", "4", "%"].contains(&&*type_pick) {
                    println!("Sorry but you have to pick on of the above type 1 etc");
                    std::io::stdin().read_line(&mut type_pick).unwrap();
                    type_pick = type_pick.trim().to_string();
                }
                if type_pick == "1" {
                    println!("what do you want the password account to be (ex. google.com)?");
                    let mut new_accont:String = "".to_string();
                    std::io::stdin().read_line(&mut new_accont).unwrap();
                    new_accont = new_accont.trim().to_string();
                    if new_accont.contains(" ") {
                        println!("sorry your account cant have a space in it try something like www.google.com");
                        std::io::stdin().read_line(&mut new_accont).unwrap();
                        new_accont = new_accont.trim().to_string();
                    }
                    println!("what do you want the password to it");
                    let mut new_pass:String = "".to_string();
                    std::io::stdin().read_line(&mut new_pass).unwrap();
                    new_pass = new_pass.trim().to_string();
                    if new_accont.contains(" ") {
                        println!("sorry your password can not have a space in it");
                        std::io::stdin().read_line(&mut new_pass).unwrap();
                        new_pass = new_pass.trim().to_string();
                    }
                    let decoded = hex::decode(contents.clone()).unwrap();
                    let plaintext = cipher.decrypt(nonce, decoded.as_ref())
                        .expect("decryption failure!");
                    let split = String::from_utf8_lossy(&*plaintext).to_string().replace("\n", " : ");
                    let split: Vec<&str> = split.split(" : ").collect();
                    let mut account_names: Vec<String> = Vec::new();
                    let mut passwords: Vec<String> = Vec::new();
                    for x in 0..split.len() {
                        if x % 2 == 0 {
                            account_names.push(split[x].to_string());
                        } else {
                            passwords.push(split[x].to_string());
                        }
                    }
                    if account_names.contains(&new_accont) {
                        for x in 0..account_names.len() {
                            if account_names[x] == new_accont {
                                passwords[x] = new_pass.to_string();
                            }
                        }
                        let mut new_text = "".to_string();
                        for x in 0..account_names.len() {
                            new_text.push_str([account_names[x].to_string(), " : ".to_string(), passwords[x].to_string(), "\n".to_string()].join("").as_str());
                        }
                        let ciphertext = cipher.encrypt(nonce, new_text.trim().as_ref())
                            .expect("encryption failure!");
                        write_file([base_dirs.config_dir().to_str().unwrap(), "/pas.man"].join(""), hex::encode(ciphertext));
                    }
                    else {
                        let ciphertext = cipher.encrypt(nonce, [String::from_utf8_lossy(&*plaintext).to_string(), "\n".to_string(), new_accont, " : ".to_string(), new_pass].join("").as_ref())
                            .expect("encryption failure!");
                        write_file([base_dirs.config_dir().to_str().unwrap(), "/pas.man"].join(""), hex::encode(ciphertext));
                    }
                }
                else if type_pick == "2" {
                    let decoded = hex::decode(contents.clone()).unwrap();
                    let plaintext = cipher.decrypt(nonce, decoded.as_ref())
                        .expect("decryption failure!"); 
                    if !matches.is_present("find") {
                        println!("what account are you looking for?");
                    }
                    loop {
                        let mut account: String = "".to_string();
                        if !matches.is_present("find") {
                            std::io::stdin().read_line(&mut account).unwrap();
                            account = account.trim().to_string();
                        }
                        else {
                            account = matches.value_of("find").unwrap().to_string();
                        }
                        if account.contains(" ") {
                            println!("Sorry the account you are looking for can not have a space in it");
                            std::io::stdin().read_line(&mut account).unwrap();
                            account = account.trim().to_string();
                        }
                        let split = String::from_utf8_lossy(&*plaintext).to_string().replace("\n", " : ");
                        let split: Vec<&str> = split.split(" : ").collect();
                        let mut account_names: Vec<String> = Vec::new();
                        let mut passwords: Vec<String> = Vec::new();
                        for x in 0..split.len() {
                            if x % 2 == 0 {
                                account_names.push(split[x].to_string());
                            } else {
                                passwords.push(split[x].to_string());
                            }
                        }
                        let mut name = Vec::new();
                        for x in account_names.clone() {
                            if x.contains(&account) {
                                name.push(x);
                            }
                        }
                        if name.len() == 0 {
                            println!("I have not fount any passwords with that name");
                            std::process::exit(1);
                        } else if name.len() == 1 {
                            for x in 0..account_names.clone().len() {
                                if name.first().unwrap() == &account_names[x] {
                                    println!("{}", passwords[x]);
                                }
                            }
                            std::process::exit(1);
                        } else {
                            if matches.is_present("find") {
                                println!("I have found many results try making it more specific next time");
                                std::process::exit(1);
                            }
                            println!("I have found\n{}\nWhat password do you want?", name.join("\n"));
                        }
                    }
                }
                else if type_pick == "3" {
                    let decoded = hex::decode(contents.clone()).unwrap();
                    let plaintext = cipher.decrypt(nonce, decoded.as_ref())
                        .expect("decryption failure!");
                    let mut table = Table::new();
                    table.add_row(Row::new(vec![
                        Cell::new("Account"),
                        Cell::new("Password")]));
                    for x in String::from_utf8_lossy(&*plaintext).to_string().split("\n") {
                        table.add_row(Row::new(vec![
                            Cell::new(x.split(" : ").nth(0).unwrap()),
                            Cell::new(x.split(" : ").nth(1).unwrap())]));
                    }
                    table.printstd();
                }
                else if type_pick == "%" {
                    fs::remove_file([base_dirs.config_dir().to_str().unwrap(), "/pas.man"].join(""));
                }
                else if type_pick == "4" {
                    let decoded = hex::decode(contents.clone()).unwrap();
                    let plaintext = cipher.decrypt(nonce, decoded.as_ref())
                        .expect("decryption failure!");
                    if !matches.is_present("delete") {
                        println!("what account are you looking for?");
                    }
                    loop {
                        let mut account: String = "".to_string();
                        if !matches.is_present("delete") {
                            std::io::stdin().read_line(&mut account).unwrap();
                            account = account.trim().to_string();
                        }
                        else {
                            account = matches.value_of("delete").unwrap().to_string();
                        }
                        if account.contains(" ") {
                            println!("Sorry the account you are looking for can not have a space in it");
                            std::io::stdin().read_line(&mut account).unwrap();
                            account = account.trim().to_string();
                        }
                        let split = String::from_utf8_lossy(&*plaintext).to_string().replace("\n", " : ");
                        let split: Vec<&str> = split.split(" : ").collect();
                        let mut account_names: Vec<String> = Vec::new();
                        let mut passwords: Vec<String> = Vec::new();
                        for x in 0..split.len() {
                            if x % 2 == 0 {
                                account_names.push(split[x].to_string());
                            } else {
                                passwords.push(split[x].to_string());
                            }
                        }
                        let mut name = Vec::new();
                        for x in account_names.clone() {
                            if x.contains(&account) {
                                name.push(x);
                            }
                        }
                        if name.len() == 0 {
                            println!("I have not found any passwords with that name");
                            std::process::exit(1);
                        } else if name.len() == 1 {
                            for x in 0..account_names.clone().len() {
                                if name.first().unwrap() == &account_names[x] {
                                    let mut new_file = Vec::new();
                                    let text = String::from_utf8_lossy(&*plaintext).to_string();
                                    for x in text.lines() {
                                        if x.split(" : ").nth(0).unwrap() != name.first().unwrap() {
                                            new_file.push(x);
                                        }
                                    }
                                    let ciphertext = cipher.encrypt(nonce, new_file.join("\n").as_ref())
                                        .expect("encryption failure!");
                                    write_file([base_dirs.config_dir().to_str().unwrap(), "/pas.man"].join(""), hex::encode(ciphertext));
                                }
                            }
                            std::process::exit(1);
                        } else {
                            if matches.is_present("delete") {
                                println!("I have found many results try being more specific next time");
                                std::process::exit(1);
                            }
                            println!("I have found\n{}\nWhat password do you want?", name.join("\n"));
                        }
                    }
                }
            }
        }
    }
}

fn write_file(file:String, text:String) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    file.write_all(text.trim().as_ref())?;
    Ok(())
}