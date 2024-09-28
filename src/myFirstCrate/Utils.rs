pub fn sum( start:u32 ,end:u32) ->u32{
   (start+end)*(end-start+1)/2
}

pub fn get_args(path:Option<&mut String>) -> Vec<String> {
   let mut args: Vec<String> = std::env::args().collect(); 
   match path{
     Some(i) =>{*i=args.remove(0); }
     _=>{args.remove(0); }
   }
   args 
}
