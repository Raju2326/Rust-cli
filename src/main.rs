// #[derive(Debug)]
use colored::*;
use std::env;
use std::fs;
use std::process;
use cli_app::ArgConfig;
fn main (){

    let args:Vec<String> = env::args().collect();
    let result = parse_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
//      println!("{:?}",args);
//     let path:&str = &args[2];
//     let search = &args[1];
//    let fs = fs::read_to_string(path).unwrap();
//    println!("{}",fs);
//    let mut result:Vec<&str>=Vec::new();
//    for j in fs.lines(){
//     if j.contains(search){
//         result.push(j);
//     }
//    }
   println!("Solution dude below {:?}",result);
}
fn parse_args(args:&[String])->Result<ArgConfig,&str>{
    println!("{}",args.len());
    if args.len()>=3{
        Ok(ArgConfig{
            search:args[0].clone(),
            path:args[1].clone()
        })
    }else{
       Err("Missing required command line arguments")
    }

}