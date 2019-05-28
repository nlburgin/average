use fnv::FnvHashMap;
use std::io::BufRead;
use ordered_float::OrderedFloat;

const MEAN: u8 = 0;
const MEDIAN: u8 = 1;
const MODE: u8 = 2;

macro_rules! HELP {
  () => {r#"
average
compute average.
input is newline-delimted list of numbers from standard input.
Usage: average [options]
  --mean         compute mean (default)
  --median       compute median
  --mode         compute mode. Tiebreaker is not defined.
  
  --round [n]    number of decimal places to round to (default 5)
                 rounding applied only before final output for median and mean, 
                 but applied before attempting to compute mode.
  
  --int,    -i   round to the nearest integer (equiv. to --round 0)  
  --help,   -h   show this help then exit
  
  Example: $ printf '2\n5\29' | average --median
           5
  
  ERRORS: Uses the laziest form of error handling available in Rust, 
  therefore error messages aren't very good.
  
  If you see the Rust runtime complaining about a "failed unwrap",
  It means invalid input was given.
  
  "#}
}

fn main() {  
  let mut op = MEAN;
  let args = std::env::args();
  let mut round: usize = 5;
  let mut rarg = false;
  //args.next();
  for arg in args{
    if rarg == true {round = arg.parse().unwrap(); rarg = false;}
    else {match &arg as &str {
      "--mode" => op = MODE,
      "--median" => op = MEDIAN,
      "--mean" => op = MEAN,
      "--int" | "-i" => round = 0,
      "--round" => rarg = true,
      "--help" | "-h" => print_help(),
      _ =>  (),
    };}
  }
  println!("{}", match op{
    MEAN => {
      let mut sum: f64 = 0.0;
      let mut count: f64 = 0.0;
      for line in std::io::stdin().lock().lines(){
        let tmp:f64 = line.unwrap().parse().unwrap();
        sum = sum + tmp; 
        count = count + 1.0;
      }
      format!("{1:.0$}",round,sum / count)
    },
    MEDIAN => {
      let mut list: Vec<OrderedFloat<f64>> = Vec::with_capacity(5000);
      let mut count: usize = 0;
      for line in std::io::stdin().lock().lines(){
        list.push(line.unwrap().parse().unwrap());
        count = count + 1;
      }
      count = count / 2;
      let list2 = &mut list[..];
      list2.sort_unstable();
      format!("{1:.0$}",round,list2[count])
    },
    MODE => {
      let mut hm = FnvHashMap::default();
      for line in std::io::stdin().lock().lines(){
        let number:f64 = line.unwrap().parse().unwrap();
        let rounded = format!("{1:.0$}", round,number);
        let count = hm.get(&rounded).unwrap_or(&0);
        hm.insert(rounded,count + 1);
      }
      let mut highest = 0;
      let mut mode:String = "0".to_string();
      for (k,v) in hm {
        if v > highest{
          highest = v;
          mode = k;
        }
      }
      mode
    },
    _ => panic!("Severe internal error :("),
  });
}

fn print_help(){
  eprintln!(HELP!());
  ::std::process::exit(1);
}
