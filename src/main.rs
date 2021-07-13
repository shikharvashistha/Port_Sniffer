//we'll use cargo run -- -h (we use -- to pass arguments to the executable and not cargo)
//ip_sniffer.exe -h help screen
//ip_sniffer.exe -j 100 192.168.1.1 how many threads to use
//ip_sniffer.exe 192.168.1.1 calling code on ip with default threads
use std::env;//standards library environment tool namespace
//helps to pull arg's out of command line
use std::net::IpAddr;
use std::str::FromsStr;//As we're bringing in our arguments as strings
//This allows us to convert our string to our IP Address type
struct Arguments{
    flag: String, 
    ipaddr: IpAddr,//IpAddr is enum either V4 or V6
    threads: u16,
}

impl Arguments{
    fn new(args: &[String]) -> Result<Arguments, &'static str>{
        //static slice of string is used so we can send back the errors
        //to main fxn and handle them there
        //arguments to our method new will be reference to vector of string
        //it'll return a Result that have Arguments struct inside of Ok portion
        //and we have a static slice of string inside the error portion of it
        if args.len() < 2 {
            return Err("Not enough arguments"); //pass back this error
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            //This if let binding to destruct our IP address from string which returns a 
            //result and if we get back an Ok then we use this IP address inside of here
            return Ok(Arguments {flag: String::from(""), ipaddr, threads: 4})
            //Then we explicitly say return our arguments struct and then we'll put 
            //flag in here as an empty string and then send back IP and the thread number 
            //which is 4 by default        
        } else{
            let flag = args[1].clone();
            //is we don't get Ok when trying to convert F into an IP address
            //that means there's is no IP which means that wither a we've one of our Flags
            //is either -h or -j or nothing important
            //we'll check then
            if flag.contains("-h") || flag.contains("-help") && args.length() ==2 {
                println!("Usage: -j to select how many threds you wnat 
                    \r\n  -h or help to show this help message")
                return Err("help");
        } else if falg.contains("-h") || flag.contains("-help") {//if args.length != 2
            return Err("Too many arguments");
        } else if flag.contains("-j") {
            let ipaddr = match IpAddr::from_str(&args[3]){
                //we're going to match in turning our args index 3
                //into an IP address and bind to ipaddr variable
                Ok(s) => s, //unwrapping our Ok value here
                Err(_) => return Err("Not a valid IP Address; must be IPv4 or IPv6");
                //if we get an error we return Error
            };
            let threads = match args[2].parse::<u16>(){
                //if we get strings we need to convert them to u16
                Ok(s) => s,//unwrap the value inside of Ok and then bind it to
                //threads if we get an error we just pass back failed to parse thread number
                Err(_) => return Err("Failed to parse thread number")
            };
            return Ok(Arguments{threads, flag, ipaddr});
        } else {
            return Err("Invalid Syntax");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //it'll take all the arguments that we pass to this program
    //and put them in vector<String>
    /*
    How to iterate over the vector element's
    for i in &args{
        println!("{}", i);
    }
    println!("{:?}", args); //:? debug flag
    */
    let program = args[0].clone();//This will be location of our executable
    /*
    let flag = args[1].clone();
    let threads = args[2].clone();
    let ipaddr = args[3].clone();
    we can do it this way but it's little verbose
    so we will make seperate struct's for each of them to make it look descent
    */
    let arguments = Arguments::new(&args).unwrap_or_else(
            |err| {
                if err.contains("help") {
                    //7:56
                } else {

                }
            }

        );

}
