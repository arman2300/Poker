//let perm:[u32;9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];let winner:Vec<String>= deal(perm);

// deal function should accepts an array of 9 integers and returns a VECTOR of five STRINGS. 
fn get_val(card: u32) -> u32
{
  if card >= 1  && card <=13 
  {
    card
  }
  else if card >=14 && card <= 26
  {
    card - 13
  }
  else if card >= 27 && card <= 39
  {
    card - 26
  }
  else if card >= 40 && card <= 52 
  {
    card - 39
  }
  else 
  {
    0
  }
}

fn get_let(card: u32) -> String
{
  if card >= 1  && card <=13 
  {
    "C".to_string()
  }
  else if card >=14 && card <= 26
  {
    "D".to_string()
  }
  else if card >= 27 && card <= 39
  {
    "H".to_string()
  }
  else if card >= 40 && card <= 52 
  {
    "S".to_string()
  }
  else 
  {
    "X".to_string()
  }
}

fn split_pool(arr: [u32;9]) -> [u32;5]
{
  let mut pool: [u32;5] = [0;5];
  pool = [arr[4],arr[5],arr[6],arr[7],arr[8]];
  pool
}

fn convert(arr: [u32;5]) -> Vec<String>
{
  let mut vec = Vec::new();
  for i in 0..(arr.len())
  {
    let mut var = format!("{}{}",get_val(arr[i]),get_let(arr[i]));
    vec.push(var);
  } 
  vec
}

//pub fn deal(perm:[u32;9]) -> Vec<String>

fn deal(perm:[u32;9]) -> Vec<String> 
{ 
  //code goes here
  let mut hand_A: [u32;5] = split_pool(perm);
  
  let h_a: Vec<String> = convert(hand_A);

  h_a

}





fn main()
{
  println!("{:?}",deal([1,2,3,4,5,6,7,8,9]));
  println!("{:?}",convert([1,2,3,4,5]));

}