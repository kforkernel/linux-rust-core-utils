/*
Copyright (c) 2026 @kforkernel
All rights reserved.

Licensed under the kforkernel Use License (KUL) v1.0.

See LICENSE.md for terms and conditions.
*/

use std::env;

fn main() {
    
    // getting and checking command line args
    let args: Vec<String> = env::args().collect();

    // checking atleast 1 argument and if so print new line
    if args.len() < 2{
       println!();
       std::process::exit(0);
    }




    // saving cmdline args in custom struct and checking flags
    let arg_results = CmdLineArgs::get_args(args);

    // putting data in variables
    let newline_check = arg_results.newline_flag;
    let escape_check = arg_results.escape_flag;
    let words = arg_results.words;




    /* DEBUG
        let master_check = arg_results.master_flag;
        println!("########## DEBUG ##########");
        println!("Master: {}", master_check);
        println!("Newline: {}", newline_check);
        println!("Escape: {}", escape_check);
        println!("########## DEBUG ##########");
    */




    // this joins all words together with space aka " " between them
    // this is done because without it the last word will have space at end
    // so if you used -n flag there would be a space after last word and your
    // cmd prompt which affects shasum and other tools
    let mut output = words.join(" ");

    // checking for -e flag
    if escape_check{
        output = output.replace("\\n", "\n");
        output = output.replace("\\t", "\t");
        output = output.replace("\\\\", "\\"); // this actually means output = output.replace("\\", "\");
        // because you have to escape also when writing the output 
    }
    
    // printing words
    print!("{output}");
    
    // checking for -n flag
    if !newline_check{
        println!();
    }
}


struct CmdLineArgs{
    master_flag: bool,
    newline_flag: bool,
    escape_flag: bool,
    words: Vec<String>,
}

// this will sort the cmdline args and turn on any needed flags
impl CmdLineArgs{
    fn get_args(args: Vec<String>) -> CmdLineArgs{

        // create default struct variable
        let mut parsed_args = CmdLineArgs{
            master_flag: false,
            newline_flag: false,
            escape_flag: false,
            words: Vec::new(),
        };

        // checking for master flag
        if args.len() == 2{
            parsed_args.master_flag = false;
            parsed_args.newline_flag = false;
            parsed_args.escape_flag = false;
            parsed_args.words = vec![args[1].clone()];
            return parsed_args;
        }

        // setting mcheck for disabling flags
        let mut mcheck = false;

        // setting smart first word detect to disable flags
        let mut wcheck = false;

        // looping through cmdline args starting from index 1 
        for i in &args[1..]{
            if i != "-n" && i != "-e" && i != "--" && !wcheck {                
                wcheck = true;
                parsed_args.words.push(i.clone());
                continue;
            }

            if i == "--" && !mcheck && !wcheck{ // this means if mcheck = false it will run cos it makes it true ! makes opposite
                // but if mcheck is true because the -- flag was detected before it will become false and this code wont run
                // because !true is false
                parsed_args.master_flag = true;
                mcheck = true;
                continue;
            }

            // if master mcheck on everything counts as word and flags of afte the --
            if mcheck{ // btw same as if mcheck == true
                parsed_args.words.push(i.clone());
                continue;
            }

            if i == "-n" && !parsed_args.newline_flag && wcheck == false{ // checks if -n been used already if have it acts as if -n isnt a flag
                parsed_args.newline_flag = true; // turn flag on
            } 
        
            else if i == "-e" && !parsed_args.escape_flag && wcheck == false{ // checks if -e been used already if have it acts as if -e isnt a flag
                parsed_args.escape_flag = true;  // turn flag on
            } 
            
            else{
                parsed_args.words.push(i.clone()); // collect words
            }
        }

        parsed_args
    }
}
