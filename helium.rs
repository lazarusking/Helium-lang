//Standard Library
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io;

//Function for factorial
fn fact(n: i32) -> i32 {
	if n <= 1 {
		n
	} else {
		n * fact(n-1)
	}
}

fn main() {
   //Keywords to be used
   kw = ["find", "unique", "--","", "delete_value", "printall", "writeln", "create_table", "add_value", "print_value"];
   
   //Parameter
   let start = ln.find('[').unwrap();
   let end = ln.find(']').unwrap();
   
   println!("::Helium v.0.9 Running::");
   print!("Run File: ");
   
   //User input for file
   let mut file = io::stdin();
   
   //Open a file and create an error if there is none
   let ofile = File::open(file).expect("\nFatal-Error: No correspoding files found");
   
   //Another check if extension is .he
   if !file.contains(".he") {
   	panic!("Fatal Error: File extension does not end with .he\n\nProgram Completed with Errors");
	unreachable!();
   }
   
   //BufReading...
   let mut line = BufReader::new(&ofile);
   
   //Read files line by line
   for ln in line.lines().enumerate() {
   	//Read line word by word
   	if ln.contains(kw[7]) {
   	     let mut TABLE = [];
       } else if ln.contains(kw[1]) {
            //Parameter
            let mut val = &ln[start+1..end-1];
            let mut x = 0;
            //Iterate to find parameter in TABLE
            loop {
            	//Print if true
            	if TABLE[x] == val {
            	   println!("{}", val);
                }
                
                x += 1;
                
                if TABLE.len() == x {
                	break;
                }
            }
          } else if ln.contains(kw[8]) {
          	let value = &ln[start+1..end-1];
              //Adding value
          	TABLE.push(value);
          } else if ln.contains(kw[4]) {
              //Remove a value
              let ind = &ln[s+1..e-1];
              TABLE.remove(ind);
          } else if ln.contains(kw[5]) {
             let cc = 0;
             loop {
            	println!("{}", TABLE[x]);
                cc += 1;
                if cc == TABLE.len() {
                	break;
                }
             }
          } else if ln.contains(kw[1]) {
          	let count = 0;
          //Loop checking if duplicates are found
          	loop {
          	   let mut ano = TABLE.find(TABLE[count]);
                   let mut aa = TABLE[ano];
          	   if TABLE[count] == aa {
          	      TABLE.remove(ano);
                 }
                 count += 1;
               if count == TABLE.len() {
                  break;
               } 
            }
        } else if ln.contains(kw[3]) {
             //Nothing since it is a comment
        } else if ln.contains('') {
        	//Nothing since nothing
        } else if ln.contains(kw[9]) {
        	let values = &ln[start+1..end+1];
        
            let valuesplace = TABLE.find(values).unwrap();
            
            let valueend = TABLE[valuesplace];
            
            println!("{}", valuesplace);
        } else if ln.contains(kw[6]) {
            //Printing strings
            let prstr = &ln[start+1..end-1];
            
            println!("{}", prstr);
        } else if ln.contains('cpt') {
            let expr = &ln[start+1..end-1];
            let s = expr.find('[').unwrap();
            let e = expr.find(']').unwrap();
            if expr.contains("to_negative") {
                let num: i32 = expr[s+1..e-1].parse().unwrap();
                println!("{}" (num*2)-num);
            } else if expr.contains("absolute") {
                let num2: i32 = expr[s+1..e-1].parse().unwrap();
                println!("{}", (num2+num2)+num2);
            } else if expr.contains('factorial') {
            	let num3: i32 = expr[s+1..end-1].parse().unwrap();
                println!("{}", fact(num3));
            }
        } else {
        	//Panic and stop if error
        	panic!("No Commands Found");
                panic!("Program Completed with Errors");
                unreachable!();
        }
    }
    println!("\nProgram Succesfully Completed");
}

/*

   DATA-BASE/COMPUTING HELIUM v.0.9
   Coder: Brylle(Z34O) @ VoltzEx
   
   Structure:
   
   create_table
   writeln["hello"]
   add_value["bob"]
   add_value["xxx"]
   print_value["bob"]
   printall  --Will print bob and xxx
   remove_value[0] --Bob removed
   find["xxx"] --True, so prints xxx
   unique --Nothing will happen since duplicates are not found
   
   djdjdjd --Interpretation will stop, no commands found. Fatal Error.

*/
