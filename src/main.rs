extern crate getopts;
extern crate text_io;
extern crate arboard;
extern crate clearscreen;

use getopts::Options;
use text_io::read;
use String;
use polars::prelude::*;
use arboard::Clipboard;
use clearscreen::clear;
use std::io::{stdin, stdout, Read, Write};

fn main() {
    clear().expect("Failed to clear screen");
    let collect = std::env::args().collect();
    let args: Vec<String> = collect;
    let _program = args[0].clone();

    println!(r#"    
 ____   _                           _                                 _        _      _    
|  _ \ | |  __ _   ___  ___  _ __  | |_  __ _   __ _   ___ __      __(_)  ___ | |__  | |_  
| |_) || | / _` | / __|/ _ \| '_ \ | __|/ _` | / _` | / _ \\ \ /\ / /| | / __|| '_ \ | __| 
|  __/ | || (_| || (__|  __/| | | || |_| (_| || (_| ||  __/ \ V  V / | || (__ | | | || |_  
|_|    |_| \__,_| \___|\___||_| |_| \__|\__,_| \__, | \___|  \_/\_/  |_| \___||_| |_| \__| 
    _                   _                     _|___/                                       
   / \    _ __    __ _ | | _   _  ___   __ _ | |_  ___   _ __                              
  / _ \  | '_ \  / _` || || | | |/ __| / _` || __|/ _ \ | '__|                             
 / ___ \ | | | || (_| || || |_| |\__ \| (_| || |_| (_) || |                                
/_/   \_\|_| |_| \__,_||_| \__, ||___/ \__,_| \__|\___/ |_|                                
                           |___/                                                           
 _____  _____  _____  _____  _____  _____  _____  _____  _____  _____  _____  _____  _____ 
|_____||_____||_____||_____||_____||_____||_____||_____||_____||_____||_____||_____||_____|
                                                                                           
                                                                                           "#);

    let mut opts = Options::new();
    opts.optopt("a", "age", "Zwangerschapsduur", "WEKEN");
    opts.optopt("w", "weight", "Placentagewicht", "GRAM");
    opts.optopt("n", "number", "Aantal kinderen", "N");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    let option_age = matches.opt_str("a");
    let option_weight = matches.opt_str("w");
    let option_number = matches.opt_str("n");
    
    let add_line;
    if option_age == None || option_weight == None || option_number == None {
        add_line = true;
    } else {
        add_line = false;
    }

    let (a, w, n) = check_variables(option_age, option_weight, option_number);

    extra_line(add_line);

    let age: i32 = a.trim().parse().unwrap();
    let weight: i32 = w.trim().parse().unwrap();
    let number: i32 = n.trim().parse().unwrap();

    match number {
        1 => singleton(age, weight),
        2 => twin(age, weight),
        3 => triplet(age, weight),
        _ => println!("Geen gewichten voor een placenta voor het opgegeven aantal kinderen")
    }

    pause();

}

fn check_variables(option_age:Option<String>, option_weight:Option<String>, option_number:Option<String> ) -> (String, String, String) {
    let age = match option_age {
        Some(a) => a,
        None => ask_variable("zwangerschapsduur".to_string())
    };

    let weight = match option_weight {
        Some(w) => w,
        None => ask_variable("placentagewicht".to_string())
    };

    let number = match option_number {
        Some(n) => n,
        None => ask_variable("aantal kinderen".to_string())
    };

    return (age, weight, number);
}

fn ask_variable(variable: String) -> String {
    print!("Geef een waarde voor {}: ", variable);
    let input: String = read!("{}\n");
    return input;
}

fn extra_line(line: bool) {
    if line == true {
        println!();
    }
}

fn singleton(age: i32, weight: i32) {
    let df = df!(
    "Week" => &[21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41],
    "P3" => &[102, 107, 88, 90, 112, 122, 108, 184, 201, 220, 239, 258, 278, 296, 317, 336, 353, 370, 386, 401, 414],
    "P10" => &[114, 122, 120, 129, 142, 160, 154, 210, 229, 249, 269, 290, 311, 331, 352, 372, 391, 409, 426, 442, 456],
	"P25" => &[128, 138, 152, 153, 169, 184, 186, 238, 259, 281, 303, 325, 347, 369, 391, 412, 432, 452, 470, 487, 502],
	"P50" => &[143, 157, 187, 198, 206, 223, 235, 270, 293, 316, 340, 364, 387, 411, 434, 457, 478, 499, 519, 537, 553],
	"P75" => &[158, 175, 220, 228, 242, 260, 280, 302, 327, 352, 377, 403, 428, 453, 477, 501, 524, 547, 567, 587, 605],
	"p90" => &[172, 191, 260, 280, 270, 300, 330, 331, 357, 384, 411, 438, 464, 493, 516, 542, 566, 589, 611, 632, 651],
	"p97" => &[184, 206, 285, 304, 298, 325, 362, 357, 385, 413, 441, 470, 497, 526, 551, 578, 603, 628, 651, 673, 693],
    ).unwrap();

    println!("---------------------------");
    println!("EENLINGSPLACENTA");
    println!("Zwangerschapsduur:\t {}", age);
    println!("Placentagewicht:\t{}", weight);
    println!("---------------------------\n");

    let filter = df
        .clone()
        .lazy()
        .filter(col("Week").eq(age))
        .collect()
        .unwrap();

    if filter.shape().0 == 1 {
        println!("{}", filter)
    } else {
        panic!("Geen gewichten voor opgegeven zwangerschapsduur beschikbaar")
    }
    
    let iterator = filter.iter().cloned();

    let mut weightvector: Vec<i32> = vec![];
    for serie in iterator.clone() {
        let as_vec: Vec<_> = serie.i32().unwrap().into_no_null_iter().collect();
        weightvector.push(as_vec[0]);
    }
    weightvector.remove(0);

    let result = closest(weight, weightvector);
    println!("Resultaat: {}", result);

    let mut clipboard = Clipboard::new().unwrap();

    clipboard.set_text(result.clone()).unwrap();

    println!("Het resultaat werd gekopieerd naar het Clipboard");
    println!("Plakken met Ctrl+V");

}

fn twin(age: i32, weight: i32) {
    let df = df!(
        "Week" => &[19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41],
        "P3" => &[131, 143, 152, 166, 181, 200, 221, 243, 268, 293, 320, 346, 373, 399, 424, 447, 469, 489, 505, 519, 529, 537, 539],
        "P10" => &[161, 166, 176, 191, 210, 232, 257, 284, 314, 345, 377, 409, 441, 472, 503, 531, 558, 582, 602, 619, 631, 639, 642],
        "P25" => &[185, 190, 202, 219, 241, 267, 297, 330, 365, 401, 439, 478, 516, 554, 590, 624, 656, 684, 708, 728, 743, 753, 756],
        "P50" => &[210, 218, 231, 251, 276, 307, 341, 380, 421, 464, 509, 554, 600, 644, 687, 727, 764, 798, 827, 850, 868, 879, 882],
        "P75" => &[239, 245, 260, 282, 311, 346, 386, 430, 478, 527, 579, 631, 683, 734, 783, 830, 873, 912, 945, 972, 993, 1005, 1009],
        "p90" => &[263, 270, 286, 310, 343, 382, 426, 475, 528, 584, 641, 700, 758, 815, 870, 923, 971, 1014, 1051, 1082, 1105, 1118, 1123],
        "p97" => &[289, 292, 310, 336, 371, 414, 462, 516, 574, 635, 698, 762, 826, 889, 949, 1007, 1059, 1107, 1148, 1181, 1207, 1221, 1226],
        ).unwrap();
    
        println!("---------------------------");
        println!("TWEELINGSPLACENTA");
        println!("Zwangerschapsduur:\t {}", age);
        println!("Placentagewicht:\t{}", weight);
        println!("---------------------------\n");
    
        let filter = df
            .clone()
            .lazy()
            .filter(col("Week").eq(age))
            .collect()
            .unwrap();
    
        if filter.shape().0 == 1 {
            println!("{}", filter)
        } else {
            panic!("Geen gewichten voor opgegeven zwangerschapsduur beschikbaar")
        }
        
        let iterator = filter.iter().cloned();
    
        let mut weightvector: Vec<i32> = vec![];
        for serie in iterator.clone() {
            let as_vec: Vec<_> = serie.i32().unwrap().into_no_null_iter().collect();
            weightvector.push(as_vec[0]);
        }
        weightvector.remove(0);
    
        let result = closest(weight, weightvector);
        println!("Resultaat: {}", result);
    
        let mut clipboard = Clipboard::new().unwrap();
    
        clipboard.set_text(result.clone()).unwrap();
    
        println!("Het resultaat werd gekopieerd naar het Clipboard");
        println!("Plakken met Ctrl+V");
}

fn triplet(age: i32, weight: i32) {
    let df = df!(
        "Week" => &[20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37],
        "P10" => &[226, 257, 289, 331, 371, 408, 444, 480, 516, 553, 591, 631, 674, 719, 768, 821, 940, 1007],
        "P50" => &[253, 284, 319, 361, 406, 456, 509, 564, 621, 673, 738, 797, 855, 911, 965, 1017, 1108, 1147],
        "p90" => &[285, 320, 345, 400, 445, 498, 558, 630, 697, 772, 849, 925, 1000, 1072, 1139, 1200, 1297, 1330],
        ).unwrap();
    
        println!("---------------------------");
        println!("DRIELINGSPLACENTA");
        println!("Zwangerschapsduur:\t {}", age);
        println!("Placentagewicht:\t{}", weight);
        println!("---------------------------\n");
    
        let filter = df
            .clone()
            .lazy()
            .filter(col("Week").eq(age))
            .collect()
            .unwrap();
    
        if filter.shape().0 == 1 {
            println!("{}", filter)
        } else {
            panic!("Geen gewichten voor opgegeven zwangerschapsduur beschikbaar")
        }
        
        let iterator = filter.iter().cloned();
    
        let mut weightvector: Vec<i32> = vec![];
        for serie in iterator.clone() {
            let as_vec: Vec<_> = serie.i32().unwrap().into_no_null_iter().collect();
            weightvector.push(as_vec[0]);
        }
        weightvector.remove(0);
    
        let result = closesttrip(weight, weightvector);
        println!("Resultaat: {}", result);
    
        let mut clipboard = Clipboard::new().unwrap();
    
        clipboard.set_text(result.clone()).unwrap();
    
        println!("Het resultaat werd gekopieerd naar het Clipboard");
        println!("Plakken met Ctrl+V");
}

fn closest(weight: i32, weights: Vec<i32>) -> String {
    let mut absvalue: i32 = 1000;
    let mut value: i32 = 0;
    for  w in weights.iter().cloned() {
        let absolute = (w-weight).abs();
        if absolute < absvalue {
            absvalue = absolute;
            value = w;
        } 
    }

    let index = weights.iter().position(|&r| r == value).unwrap();

    let result;
    if (weight >= (value-5)) && (weight <= (value+5)) {
        result = format!("Rond de {}", index_mapper(index));
    } else if index == 0 && weight < value {
        result = format!("Onder de {}", index_mapper(index));
    } else if index == 6 && weight > value {
        result = format!("Boven de {}", index_mapper(index));
    } else if weight < value {
        result = format!("Tussen de {} en de {}", index_mapper(index-1), index_mapper(index));
    } else {
        result = format!("Tussen de {} en de {}", index_mapper(index), index_mapper(index+1));
    }

    return result;
}

fn index_mapper (index: usize) -> String {
    let percentile: String = match index {
        0 => "P3".to_string(),
        1 => "P10".to_string(),
        2 => "P25".to_string(),
        3 => "P50".to_string(),
        4 => "P75".to_string(),
        5 => "P90".to_string(),
        6 => "P97".to_string(),
        _ => panic!("Indexerror")
    };
    return percentile;
}

fn closesttrip(weight: i32, weights: Vec<i32>) -> String {
    let mut absvalue: i32 = 1000;
    let mut value: i32 = 0;
    for  w in weights.iter().cloned() {
        let absolute = (w-weight).abs();
        if absolute < absvalue {
            absvalue = absolute;
            value = w;
        } 
    }

    let index = weights.iter().position(|&r| r == value).unwrap();

    let result;
    if (weight >= (value-5)) && (weight <= (value+5)) {
        result = format!("Rond de {}", index_mapper_trip(index));
    } else if index == 0 && weight < value {
        result = format!("Onder de {}", index_mapper_trip(index));
    } else if index == 2 && weight > value {
        result = format!("Boven de {}", index_mapper_trip(index));
    } else if weight < value {
        result = format!("Tussen de {} en de {}", index_mapper_trip(index-1), index_mapper_trip(index));
    } else {
        result = format!("Tussen de {} en de {}", index_mapper_trip(index), index_mapper_trip(index+1));
    }

    return result;
}

fn index_mapper_trip (index: usize) -> String {
    let percentile: String = match index {
        0 => "P10".to_string(),
        1 => "P50".to_string(),
        2 => "P90".to_string(),
        _ => panic!("Indexerror")
    };
    return percentile;
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Druk op Enter om af te sluiten...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}