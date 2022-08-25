/*
     ###         ###       #########  ###########     ##########   #########  #############
    ######      ###       ###        ###             ###    ###   ###             ###
   ###  ###    ###       ###          ##########    ###    ###   ########        ###
  ###    ###  ###  ###  ###                   ###  ###    ###   ###             ###
 ###      ######       ###                   ###  ###    ###   ###             ###
###        ###        ###########  ###########   ##########   ###             ###

*/


use std::io;
use std::env;
use std::process;
use std::time;
use rand::Rng;

fn main() {
    let mut input = String::new();
    let args: Vec<String> = env::args().collect();
    let numofnums: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    //Check for error
    if numofnums == 0 {
        help();
    }

    let mut arr: Vec<i8> = [].to_vec();
    let mut tracker = 0;

    //Init array
    loop {
        if tracker == numofnums {
            break;
        }
        arr.push(rand::thread_rng().gen_range(0..10));
        tracker += 1;
    }

    //Dump arr
    for i in 0..arr.len() {
        print!("{}", arr[i]);
    }
    println!("");

    //Start timer
    let start = time::Instant::now();
    
    //Get input
    io::stdin()
        .read_line(&mut input)
        .expect("Readline fail");

    //End timer and get time
    let time = start.elapsed();

    //Compare
    let input_arr: Vec<i8> = to_int_vec(&input.trim());
    let accuracy = compare(arr, input_arr);
    println!("Accuracy: {}%", (accuracy as f64 / numofnums as f64) * 100.0);
    println!("KPM: {}", (input.len() as f64 / time.as_secs_f64()) * 60.0);
    process::exit(0);
}

fn compare(arr1: Vec<i8>, arr2: Vec<i8>) -> usize {
    let mut correct = 0;
    let mut tracker = 0;
    for i in &arr1 {
        if tracker == arr2.len() {
            break;
        }
        if arr2[tracker] == -1 {
            continue;
        }
        if i == &arr2[tracker] {
            correct += 1;
        }
        tracker += 1;
    }
    return correct;
}

fn to_int_vec(input: &str) -> Vec<i8> {
    let mut ret: Vec<i8> = [].to_vec();
    for i in input.bytes() {
        let tmp: i8 = i.try_into().unwrap();
        if tmp < 48 || tmp > 57 {
            ret.push(-1);
        } else {
            ret.push(tmp - 48);
        }
    }
    return ret;
}

fn help() {
    println!("num-racer [# of numbers in sequence]");
    process::exit(1);
}
