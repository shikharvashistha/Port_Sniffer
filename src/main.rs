use std::env; //standards library environment tool namespace
              //helps to pull arg's out of command line
use std::io::{self, Write};
use std::net::IpAddr; //to connect to our IP Address
use std::str::FromStr; //As we're bringing in our arguments as strings
                       //This allows us to convert our string to our IP Address type
use std::net::TcpStream;
use std::process; //This will allow use to manage the way our program shuts down
use std::sync::mpsc::{channel, Sender};
use std::thread;
//tcpstream will allow us to create a tcp stream

const MAX: u16 = 65535; //These are the max ports we can sniff

struct Arguments {
    flag: String,
    ipaddr: IpAddr, //IpAddr is enum either V4 or V6
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
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
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            //This if let binding to destruct our IP address from string which returns a
            //result and if we get back an Ok then we use this IP address inside of here
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr,
                threads: 4,
            });
            //Then we explicitly say return our arguments struct and then we'll put
            //flag in here as an empty string and then send back IP and the thread number
            //which is 4 by default
        } else {
            let flag = args[1].clone();
            //is we don't get Ok when trying to convert F into an IP address
            //that means there's is no IP which means that wither a we've one of our Flags
            //is either -h or -j or nothing important
            //we'll check then
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!(
                    "Usage: -j to select how many threds you wnat
                    \r\n  -h or help to show this help message"
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                //if args.length != 2
                return Err("Too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    //we're going to match in turning our args index 3
                    //into an IP address and bind to ipaddr variable
                    Ok(s) => s, //unwrapping our Ok value here
                    Err(_) => return Err("Not a valid IP Address; must be IPv4 or IPv6"), //if we get an error we return Error
                };
                let threads = match args[2].parse::<u16>() {
                    //if we get strings we need to convert them to u16
                    Ok(s) => s, //unwrap the value inside of Ok and then bind it to
                    //threads if we get an error we just pass back failed to parse thread number
                    Err(_) => return Err("Failed to parse thread number"),
                };
                return Ok(Arguments {
                    threads,
                    flag,
                    ipaddr,
                });
            } else {
                return Err("Invalid Syntax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    //our first thread will look at port number 1 our second thread will look at port number 2 and so on.
    let mut port: u16 = start_port + 1;
    //as we're using 0 based indexing
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                //This means the port is open
                print!("."); //It is there to just send back the feedback to the user that it is working
                             //everytime it finds a port that is open
                             //To use IO in thread we need to import IO
                io::stdout().flush().unwrap();
                //It allow us to send all these print statements to the mutex
                //of shared data
                tx.send(port).unwrap(); //This will be send back to rx that we created in main() the port number
            }
            //which is open
            Err(_) => {}
        }
        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads; //we iterate our port by the number of thread
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
    let program = args[0].clone(); //This will be location of our executable
                                   /*
                                   let flag = args[1].clone();
                                   let threads = args[2].clone();
                                   let ipaddr = args[3].clone();
                                   we can do it this way but it's little verbose
                                   so we will make seperate struct's for each of them to make it look descent
                                   */
    let arguments = Arguments::new(&args).unwrap_or_else(
        //closure
        |err| {
            if err.contains("help") {
                process::exit(0); //quit out of our program we pass 0 so it doesn't pin it
            } else {
                eprintln!("{} Problem parsing arguments: {}", program, err); //error print new line
                process::exit(0);
            }
        },
    );
    let num_threads = arguments.threads; //we bind arguments.threds to our variable num_thread
    let addr = arguments.ipaddr;
    let (tx, rx) = channel(); //we instantiate a channel and destruct the tuple returned from it
    for i in 0..num_threads {
        let tx = tx.clone(); //we need transmitter for every thread we've
        thread::spawn(move || {
            //we spawn our thread with a move closure
            scan(tx, i, addr, num_threads);
        });
    }
    let mut out = vec![];
    drop(tx); //This helps TX is only in the other threads not in the main thread
    for p in rx {
        //we get output from our reciever here
        out.push(p);
    }
    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}
