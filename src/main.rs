use std::fs;
use rand::Rng;
use file_indexing::pust;
use scanrs::scann;
fn main() {
    println!("How many people do you want to gen?");
    let peopletimes: usize = scann();

    let namefile = "names.txt";
    let lastnamefile = "lastnames.txt";
    let writefile = "namenpesel.txt";

    fs::write(writefile, "").expect("File clean failure");
    let firstnames = fs::read_to_string(namefile)
                .expect("Something went wrong reading the file");
    let lastnames = fs::read_to_string(lastnamefile)
                .expect("Something went wrong reading the file");


    let mut firstnamelines: Vec<String> = firstnames.split_whitespace().map(|s| s.to_string()).collect();
    let mut lastnamelines: Vec<String> = lastnames.split_whitespace().map(|s| s.to_string()).collect();

    let line = connect_pesel(&mut firstnamelines, &mut lastnamelines, peopletimes);
        
    pust(writefile, line.as_str()).expect("Something went wrong writing the file");
}

fn connect_pesel(names: &mut Vec<String>, lastnames: &mut Vec<String>, times: usize) -> String{
    let mut rng = rand::thread_rng();
    let mut namenpesel = "".to_string();
    let mut showing: String;
    for i in 0..times{
        showing = format!("{} {} {}\n", names[rng.gen_range(0..names.len())],lastnames[rng.gen_range(0..lastnames.len())] , rng.gen_range::<u64, _>(10000000000..99999999999));
        namenpesel += showing.as_str();
        println!("{}/{}",i ,times);
        
    }
    return namenpesel;
}