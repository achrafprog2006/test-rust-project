
use std::{
     io::{self,Write}, net::{IpAddr,TcpStream}, process, str::FromStr, sync::mpsc::{channel, Sender}, thread, time::Duration
  }; 

#[derive(Debug)]
struct Arguments{
  ipAddr : IpAddr,
  threads:u16
}
impl Arguments{
//args are 2 or 3 
  fn new(args:&[String])->Result<Arguments , &'static str>{
    let l = args.len();
    if l<1{return Err("Error: few Arguments !!");}
    if l>3{return Err("Error: too much arguments !!");}

    let ip =args[0].clone();
    if let Ok(i) = IpAddr::from_str(&ip){
      if l==1 {
         return Ok(Arguments{  
           ipAddr:i,
           threads:4
           }) ;
      }
      else if l==3{
         let (flag,num) =(args[1].clone(),args[2].clone());
         if flag.contains("-j"){
             let threads:u16 = match num.parse::<u16>(){ 
              Ok(i)=>i , 
              Err(e)=>{return Err("invalid number !!");}
            };
            return Ok(Arguments{
             ipAddr:i,
             threads
             }) ;
         
         }
      }else {return Err("Error : few Arguments !!");}

    }
    if l==1 && args[0]=="-h"{  
      println!("help hhhh mskin");  
      return Err("help") ;
    }
      Err("  3awd l rask ")

  }

}
const MAX:u16 = 10000;
fn scan(tx :Sender<u16> , start_port:u16 , ip:IpAddr , num_threads:u16){
  let mut port = start_port+1;
   loop{
       match TcpStream::connect((ip,port)){
        Ok(_)=>{
          io::stdout().flush().unwrap();
          tx.send(port).unwrap();
          println!(".{}",port);
        }
        Err(_)=>{          println!("np {}",port);
      }
       }
       if(MAX-port)<=num_threads{break;}
       port+=num_threads;
   }
}


fn main(){

  let mut programPath: String =String::new();

  let args= app1::myFirstCrate::Utils::get_args(Some(&mut programPath));
  let argument = Arguments::new(&args).unwrap_or_else(
    |err:&str|{
      if !err.contains("help"){println!("{}",err);}
      else{}
      process::exit(0);
    }
   );

   println!("{:?}" , argument);

   let num_threads = argument.threads;
   let (tx,rx) = channel();

   for i in 0..num_threads{
      let tx =tx.clone();
      thread::spawn(move ||{
     scan(tx,i,argument.ipAddr,num_threads);
   });
   }

   let mut out= vec![];
   drop(tx);
   println!("");
   for i in rx{
     out.push(i);
   }
   out.sort();
   for i in out{
    println!("{} is open!!",i);
   }
   
}