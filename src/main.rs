use std::io;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;
use std::path::Path;
use std::num::ParseIntError;

struct Profile {
  id: i32,
  name: String,
  date: String,
  home: String,
  comment: String
}

impl Profile {
  fn print_profile(&self) {
    println!("Id    : {}", self.id);
    println!("Name  : {}", self.name);
    println!("Birth : {}", self.date);
    println!("Addr  : {}", self.home);
    println!("Com.  : {}", self.comment);
    println!("");
  }

  fn to_csv(&self) -> String{
    format!("{},{},{},{},{}", self.id, self.name, self.date, self.home, self.comment)
  }
}

fn new_profile(line: &str, profile_list: &mut Vec<Profile>) {
  let profile: Vec<_> = line.split(',').collect();
  if profile.len() == 5 {
    let p = Profile {
      id: profile[0].parse().unwrap(),
      name: profile[1].to_string(),
      date: profile[2].to_string(),
      home: profile[3].to_string(),
      comment: profile[4].to_string(),
    };
    profile_list.push(p);
  } else {
    println!("format error");
  }
}


fn cmd_quit(){
    std::process::exit(1);
}

fn cmd_check(len: usize){
  println!("{} profile(s)", len); 
}

fn cmd_print(param: Result<i32, ParseIntError>, profile_list: &mut Vec<Profile>) {
  let n = match param {
    Ok(x) => {
      x
    },
    Err(_) => {
      println!("format error");
      return;
    }
  };

  if profile_list.len() < n.abs() as usize {
    println!("over");
    return
  }

  if n == 0 {
   for profile in profile_list {
     profile.print_profile();
   }
  } else if n > 0 {
    for profile in profile_list.iter().take(n as usize) {
      profile.print_profile();
    }
  } else {
    let x = profile_list.len() - n.abs() as usize;
    for profile in profile_list.iter().skip(x) {
      profile.print_profile();
    }
  }
}

fn cmd_read(filename: &str, profile_list: &mut Vec<Profile>) {
  let is_file_exist: bool = Path::new(filename).exists();
  if is_file_exist == false {
    return
  }

  let f = File::open(filename).expect("file not found");
  let f = BufReader::new(f);
  for l in f.lines().filter_map(|result| result.ok()) {
    parse_line(&l, profile_list);
  }
}

fn cmd_write(filename: &str, profile_list: &mut Vec<Profile>) {
  let mut f = File::create(filename).unwrap();
  for profile in profile_list {
    let write_csv = profile.to_csv();
    writeln!(f, "{}", write_csv); 
  }
}

fn cmd_find(param: &str, profile_list: &mut Vec<Profile>) {
  let word = param.to_string();
  for profile in profile_list {
    if profile.id.to_string() == word || profile.name == word || profile.date == word || profile.home == word || profile.comment == word {
      profile.print_profile();
    }
  }
}

fn cmd_sort(param: i32, profile_list: &mut Vec<Profile>) {
  match param {
    1 => profile_list.sort_by_key(|p| p.id),
    2 => profile_list.sort_by(|a, b| a.name.cmp(&b.name)),
    3 => profile_list.sort_by(|a, b| a.date.cmp(&b.date)),
    4 => profile_list.sort_by(|a, b| a.home.cmp(&b.home)),
    5 => profile_list.sort_by(|a, b| a.comment.cmp(&b.comment)),
    _ => println!("format error")
  }
}

fn exec_command(cmd :char, param :&str, profile_list: &mut Vec<Profile>){
  match cmd {
    'Q' => cmd_quit(),
    'C' => cmd_check(profile_list.len()),
    'P' => cmd_print(param.parse(), profile_list),
    'R' => cmd_read(param, profile_list),
    'W' => cmd_write(param, profile_list),
    'F' => cmd_find(param, profile_list),
    'S' => cmd_sort(param.parse().unwrap(), profile_list),
    _ => println!("Invalid command {}: ignored.", cmd),
  }
}

fn parse_line(s: &str, profile_list: &mut Vec<Profile>) {
  let v: Vec<_> = s.split(" ").collect();
  if v[0].starts_with('%') {
    let cmd: char = v[0].chars().nth(1).unwrap();
    if v.len() == 1 {
      exec_command(cmd, "0", profile_list);
    } else {
      exec_command(cmd, v[1], profile_list);
    }
  } else {
    new_profile(s, profile_list);
  }
}

fn get_line() -> String {
   let mut s = String::new();
   io::stdin().read_line(&mut s).ok();
   s.trim().to_string()
}

fn main() {
  let n = 10000;
  let mut profile_list: Vec<Profile> = Vec::with_capacity(n);
  loop{
    let line = get_line();
    parse_line(&line, &mut profile_list);
  }
}

