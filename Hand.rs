// get_final_hand() out of all combination
// get_ranks(hand)

//get_win_hand(hand_a,hand_b)


fn get_suit(card: i32) -> i32
{ 
  if card >= 1  && card <=13 
  {
    1
  }
  else if card >=14 && card <= 26
  {
    2
  }
  else if card >= 27 && card <= 39
  {
    3
  }
  else if card >= 40 && card <= 52 
  {
    4
  }
  else 
  {
    0
  }
}


fn get_val(card: i32) -> i32
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

fn get_val_hand(arr: &mut[i32]) -> &mut[i32]
{
  for i in 0..(arr.len())
  { 
    println!("Hand is (inside) {:?}",arr);
    arr[i] = get_val(arr[i]);  
  }
  arr
}

/*
fn get_val_h(arr: &mut Vec<i32>) -> Vec<i32>
{
  for i in 0..(arr.len())
  { 
    arr[i] = get_val(arr[i]);  
  }
  arr
}
*/
fn get_let(card: i32) -> String
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

fn strint(mut s1: String, num: i32) ->String
{
  return format!("{}{}",s1,num)
}


fn convert(arr: [i32;5]) -> Vec<String>
{
  let mut vec = Vec::new();
  for i in 0..(arr.len())
  {
    let mut var = format!("{}{}",get_val(arr[i]),get_let(arr[i]));
    vec.push(var);
  } 
  vec
}

//////////////////////////////////////////////////

//.into() method make it into OWNED type 

//pub enum Option<T> 
//{
//  None,
//  Some(T),
//}

//struct my_bool 
//{
//  tr: 1
//  fa: 0
//}


fn has_flush(hand: &mut[i32]) -> bool
{ //same suit
  let mut t = true;
  
  for i in 0..((hand.len())-1)
  {
    //println!("{}",get_suit(hand[i]));
    //println!("Card {} is: {}",i,hand[i]);
    if (get_suit(hand[i]) != get_suit(hand[i+1]))
    {
     t = false;
     break;
    } 
  }
  t
}


fn has_royal_flush(hand: &mut[i32]) -> bool
{ let mut var = false;
  
  if has_flush(hand) == true {

  //let mut var = false;
  println!("Hand is (outside1) {:?}",hand);
  let mut new_hand = get_val_hand(hand);
  
  new_hand.sort();
  if new_hand[0] == 1 
  {
    if new_hand[1] == 10
    {
      if new_hand[2] == 11
      {
        if new_hand[3] == 12
        {
          if new_hand[4] == 13{ var = true;}
        }
      }
    }
  }

}
  var
}



fn has_straight_flush(hand: &mut[i32]) -> bool
{
  let mut var = false;
  
  if has_flush(hand) == true 
  {
    if has_straight(hand) == true
    {
      var = true;
    }
  }
  var
}

fn has_straight(hand: &mut[i32]) -> bool 
{
  let mut var = false;
  println!("Hand is (outside2) {:?}",hand);
  let mut new_hand = get_val_hand(hand);
  println!("Hand is (straight) {:?}",new_hand);
  new_hand.sort();
  println!("Hand is (straight) {:?}",new_hand);
  //println!("New hand is:{:?}",new_hand);
  for i in 0..((new_hand.len())-1)
  {
    println!("Hand is (straight) {:?}",new_hand); 
  //if new_hand[i] < new_hand[i+1]
    if (new_hand[i+1] - new_hand[i] == 1)
      {
        var = true;
      }
    else
    {
      var = false;
      break;
    }
  }
  var
}

fn has_fr_of_kind(hand: &mut[i32]) -> bool
{
  let mut my_bool = false;
  //let mut vec = Vec::new();
 // let mut res: [i32;2] = [0,0];
 println!("Hand is (outside3) {:?}",hand);
  let mut new_hand = get_val_hand(hand);
  new_hand.sort();
  
  let mut count_a = 0;
  let mut var_a = new_hand[0];
  for i in 0..(new_hand.len())
  { 
    if new_hand[i] == var_a
    {
      count_a += 1;
    }
  }

  let mut count_b = 0;
  let mut var_b = new_hand[1];
  for i in 0..(new_hand.len())
  { 
    if new_hand[i] == var_b
    {
      count_b += 1;
    }
  }
  
  if count_a == 4 || count_b == 4
  {
    my_bool = true;
  }
 // println!("{}",count_a);
 // println!("{}",count_b);
  my_bool
}

fn has_thr_of_kind(hand: &mut[i32]) -> bool
{
  let mut my_bool = false;
  println!("Hand is (outside4) {:?}",hand);
  let mut new_hand = get_val_hand(hand);
  new_hand.sort();
  
  let mut count_a = 0;
  let mut var_a = new_hand[2];
  for i in 0..(new_hand.len())
  { 
    if new_hand[i] == var_a
    {
      count_a += 1;
    }
  }

  if count_a == 3
  {
    my_bool = true;
  }
  my_bool
}

fn has_pair(hand: &mut[i32]) -> bool
{
  let mut my_bool = false;
  println!("Hand is (outside5) {:?}",hand);
  let mut new_hand = get_val_hand(hand);
  new_hand.sort();
  
  let mut count_a = 0;
  let mut var_a = new_hand[1];
  for i in 0..(new_hand.len())
  { 
    if new_hand[i] == var_a
    {
      count_a += 1;
    }
  }

  let mut count_b = 0;
  let mut var_b = new_hand[3];
  for i in 0..(new_hand.len())
  { 
    if new_hand[i] == var_b
    {
      count_b += 1;
    }
  }
  
  if count_a == 2 || count_b == 2
  {
    my_bool = true;
  }
  
  my_bool 
}

fn has_full_house(hand: &mut[i32]) -> bool
{
  let mut my_bool = false;
  if has_thr_of_kind(hand) == true
  {
    if has_pair(hand) == true
    {
      my_bool = true;
    }
  }
  my_bool
}

fn has_two_pair(hand: &mut[i32]) -> bool 
{
  let mut my_bool = false;
  println!("Hand is (outside6) {:?}",hand);
  let mut new_hand = get_val_hand(hand);
  new_hand.sort();
  
  let mut count_a = 0;
  let mut var_a = new_hand[1];
  for i in 0..(new_hand.len())
  { 
    if new_hand[i] == var_a
    {
      count_a += 1;
    }
  }

  let mut count_b = 0;
  let mut var_b = new_hand[3];
  for i in 0..(new_hand.len())
  { 
    if new_hand[i] == var_b
    {
      count_b += 1;
    }
  }
  
  if count_a == 2 && count_b == 2
  {
    my_bool = true;
  }
  
  my_bool 

}

////////////////////////////////////////////


fn get_ranks(hand: &mut[i32;5]) -> i32
{
  println!("Hand is {:?}",hand);
  let mut var: i32 = 0;
  if has_flush(hand) //== false
  {
    //println!("{}",has_flush(hand));
    println!("Hand is(flush) {:?}",hand);
    var = 5;
  }
  else if has_straight_flush(hand)
  {
    var = 2;
  }
  else if has_royal_flush(hand)
  {
    var = 1;
  }
  else if has_fr_of_kind(hand)
  {
    var = 3;
  }
  else if has_full_house(hand)
  {
    var = 4;
  }
  
  else if has_straight(hand)
  {
    var = 6;
  }
  else if has_thr_of_kind(hand)
  {
    var = 7;
  }
  else if has_two_pair(hand)
  {
    var = 8;
  }
  else if has_pair(hand)
  {
    var = 9;
  }
  else 
  {
    var = 10;
  }

  var
}




fn main() 
{
  /*
  let x = 15;
  let y = 32;
  
  //&mut[34,32,10,5,27]
  let mut z: [i32;5] = [34,32,10,5,27]; 
  // let mut v = vec![34,32,10,5,27];

  let mut h: [i32;5] = [14,23,15,22,16];

  //Check get_ranks for below
  //let mut r: [i32;5] = [8,23,8,51,4];
  //let mut r: [i32;5] = [40,41,48,43,51];
  //[10,32,10,5,8] pair but shown as flash
  let mut r: [i32;5] = [10,32,23,5,8];
  let mut r2: [i32;5] = r.clone();
  let g: [i32;5] = [14,23,24,25,26];
  let test: [i32;5] = [14,23,24,25,26];

  println!("{}",get_suit(x));
//  println!("{}",get_val(y));
//  println!("{:?}",get_val_hand(&mut(z)));
// println!("{:?}",get_val_h(v));
//  println!("{}",get_let(34));
println!("flush,{}",has_flush(&mut(h)));  
println!("flush,{}",has_flush(&mut(r)));


//  println!("{:?}",convert([1,2,3,14,5]));
  println!("Royal Flush,{:?}", has_royal_flush(&mut(r)));
  println!("straightFlush,{:?}", has_straight_flush(&mut(r)));
  println!("straight,{:?}", has_straight(&mut(r)));
 // println!("fOUR,{:?}", has_fr_of_kind(&mut(r)));
 // println!("THR,{:?}", has_thr_of_kind(&mut(r)));
 // println!("Pair,{:?}", has_pair(&mut(r)));
 // println!("2 Pairs,{:?}", has_two_pair(&mut(r)));

  println!("FullHouse,{:?}", has_full_house(&mut(r)));

  println!("{:?}", get_ranks(&mut(h)));
  println!("{:?}", r2);
  println!("{:?}", get_ranks(&mut(r2)));
 */
}