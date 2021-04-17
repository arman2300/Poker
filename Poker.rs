// Armaan Mohammad Ali #500875679
// Alexander Kazakov #


//let perm:[u32;9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];let winner:Vec<String>= deal(perm);

// deal function should accepts an array of 9 integers and returns a VECTOR of five STRINGS. 
fn get_suit(card: u32) -> u32
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

//fn get_val_hand(arr: &mut[u32]) -> &mut[u32]

fn get_val_hand(arr: [u32;5]) -> [u32;5]
{
  let mut new_arr: [u32;5] = [0;5];
  for i in 0..(arr.len())
  { 
    new_arr[i] = get_val(arr[i]);  
  } 
  new_arr
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

fn get_ranks(hand: [u32;5]) -> u32
{
  let mut var: u32 = 0;
  if has_royal_flush(hand)
  {
    var = 1;
  }
  else if has_straight_flush(hand)
  {
    var = 2;
  }
  else if has_fr_of_kind(hand)
  {
    var = 3;
  }
  else if has_full_house(hand)
  {
    var = 4;
  }
  else if has_flush(hand) //==false
  {
    //println!("hello,{}",has_flush(hand));
    var = 5;
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

fn split_pool(arr: [u32;9]) -> [u32;5]
{
  let mut pool: [u32;5] = [0;5];
  pool = [arr[4],arr[5],arr[6],arr[7],arr[8]];
  pool
}

fn split_hand_a(arr: [u32;9]) -> [u32;2]
{
  let mut hand_a: [u32;2] = [0;2];
  hand_a = [arr[0],arr[2]];
  hand_a
}

fn split_hand_b(arr: [u32;9]) -> [u32;2]
{
  let mut hand_b: [u32;2] = [0;2];
  hand_b = [arr[1],arr[3]];
  hand_b
}

fn get_comb(pool: [u32;5], hand: [u32;2]) ->[[u32;5];10]
{
  let mut comb: [[u32;5];10] = [[0;5];10];

  comb = 
  [
    [hand[0],hand[1],pool[0],pool[1],pool[2]],
    [hand[0],hand[1],pool[0],pool[1],pool[3]],
    [hand[0],hand[1],pool[0],pool[1],pool[4]],
    [hand[0],hand[1],pool[0],pool[2],pool[3]],
    [hand[0],hand[1],pool[0],pool[2],pool[4]],
    [hand[0],hand[1],pool[0],pool[3],pool[4]],
    [hand[0],hand[1],pool[1],pool[2],pool[3]],
    [hand[0],hand[1],pool[1],pool[2],pool[4]],
    [hand[0],hand[1],pool[1],pool[3],pool[4]],
    [hand[0],hand[1],pool[2],pool[3],pool[4]]
  ];

  comb
}

fn get_final_hand(combin: [[u32;5];10]) -> [u32;5] 
{
  let mut ranks: [u32;10] = [0;10]; 
  for i in 0..10
  {
   let mut rank: u32 = 0;
   let cur_comb: [u32;5] = combin[i]; 
   rank = get_ranks(cur_comb);
   ranks[i] = rank;
  }

//  ranks
//Inside the ranks array, find the lowest index
let mut smallest: u32 = ranks[0];
let mut index: usize = 0;
for i in 0..9
{
  if ranks[i] < ranks[i+1]
  {
    smallest = ranks[i];
    index = i;
  }
}
// use that index to get the final hand from comb
let final_hand: [u32;5] = combin[index];
final_hand

}

fn get_winning_hand(a: [u32;5], b: [u32;5]) -> u32
{
  let mut winner: u32 = 0;
  if get_ranks(a) < get_ranks(b)
  {
    winner = 1;
  }
  else if get_ranks(a) > get_ranks(b)
  {
    winner = 2;
  }
  else 
  {
    winner = 3; 
  }

  winner

}
///////////////////////////////////////
// ALL THE MAIN TESTS
fn has_flush(hand: [u32;5]) -> bool
{ //same suit
  let mut t = true;
  
  for i in 0..((hand.len())-1)
  {
    if (get_suit(hand[i]) != get_suit(hand[i+1]))
    {
     t = false;
     break;
    } 
  }
  t
}

//fn has_royal_flush(hand: &mut[u32]) -> bool

fn has_royal_flush(hand: [u32;5]) -> bool
{ let mut var = false;
  
  if has_flush(hand) == true {

  //let mut var = false;
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

//fn has_straight_flush(hand: &mut[u32]) -> bool

fn has_straight_flush(hand: [u32;5]) -> bool
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


//fn has_straight(hand: &mut[u32]) -> bool 
fn has_straight(hand: [u32;5]) -> bool 
{
  let mut var = false;
  let mut new_hand = get_val_hand(hand);
  new_hand.sort();
  //println!("{:?}",new_hand);
  for i in 0..((new_hand.len())-1)
  {
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

//fn has_fr_of_kind(hand: &mut[u32]) -> bool

fn has_fr_of_kind(hand: [u32;5]) -> bool
{
  let mut my_bool = false;
  //let mut vec = Vec::new();
 // let mut res: [i32;2] = [0,0];
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

//fn has_thr_of_kind(hand: [u32]) -> bool

fn has_thr_of_kind(hand: [u32;5]) -> bool
{
  let mut my_bool = false;
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

//fn has_pair(hand: &mut[u32]) -> bool

fn has_pair(hand: [u32;5]) -> bool
{
  let mut my_bool = false;
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

//fn has_full_house(hand: &mut[u32]) -> bool

fn has_full_house(hand: [u32;5]) -> bool
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

//fn has_two_pair(hand: &mut[u32]) -> bool

fn has_two_pair(hand: [u32;5]) -> bool 
{
  let mut my_bool = false;
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
//////////////////////////////////////////
//Tie Breaker
fn tie_royal_flush(hand_a: [u32;5],hand_b: [u32;5]) -> u32
{
  1
}

fn tie_straight_flush(hand_a: [u32;5],hand_b: [u32;5]) -> u32
{
  
  let mut ha: [u32;5] = hand_a;
  let mut hb: [u32;5] = hand_b;

  ha.sort();
  hb.sort();

  let mut x: u32 = 0;
  
    if ha[4] > hb[4] 
    {
      x = 1;
    }
    else 
    {
      x = 2;
    } 
  x

}

fn tie_fr_of_kind(hand1: [u32;5],hand2: [u32;5]) -> u32 {
  let mut ha: [u32;5] = hand1;
  let mut hb: [u32;5] = hand1;

  ha.sort();
  hb.sort();
  
  let mut ret = 0;  
  if ha[2] > hb[2]{
      ret = 1;
    }
  else if hb[2] > ha[2]{
      ret = 2;
    }
    ret
}

fn tie_full_house(hand1: [u32;5],hand2: [u32;5]) -> u32 {
  let mut ha: [u32;5] = hand1;
  let mut hb: [u32;5] = hand1;

  ha.sort();
  hb.sort();
  
  let mut ret = 0;  
  if ha[2] > hb[2]{
      ret = 1;
    }
  else if hb[2] > ha[2]{
      ret = 2;
    }
    ret
}

fn tie_flush(hand1: [u32;5],hand2: [u32;5]) -> u32 {
  let mut ha: [u32;5] = hand1;
  let mut hb: [u32;5] = hand1;

  ha.sort();
  hb.sort();
  
  let mut ret = 0;  
  if ha[4] > hb[4]{
      ret = 1;
    }
  else if hb[4] > ha[4]{
      ret = 2;
    }
    ret
}

fn tie_straight(hand1: [u32;5],hand2: [u32;5]) -> u32 {
  let mut ha: [u32;5] = hand1;
  let mut hb: [u32;5] = hand1;

  ha.sort();
  hb.sort();
  
  let mut ret = 0;  
  if ha[4] > hb[4]{
      ret = 1;
    }
  else if hb[4] > ha[4]{
      ret = 2;
    }
    ret
}

fn tie_thr_of_kind(hand1: [u32;5],hand2: [u32;5]) -> u32 {
  let mut ha: [u32;5] = hand1;
  let mut hb: [u32;5] = hand1;

  ha.sort();
  hb.sort();
  
  let mut ret = 0;  
  if ha[2] > hb[2]{
      ret = 1;
    }
  else if hb[2] > ha[2]{
      ret = 2;
    }
    ret
}

fn tie_pair(hand1: [u32;5],hand2: [u32;5]) -> u32{
  let mut ha: [u32;5] = hand1;
  let mut hb: [u32;5] = hand1;

  ha.sort();
  hb.sort();
  
  let mut ret = 0;  
  if ha[3] > hb[3]{
      ret = 1;
    }
  else if hb[3] > ha[3]{
      ret = 2;
    }
    ret
}

fn tie_two_pair(hand_a: [u32;5], hand_b: [u32;5]) -> u32
{
  let mut ha: [u32;5] = hand_a;
  let mut hb: [u32;5] = hand_b;

  ha.sort();
  hb.sort();

  let mut x: u32 = 0;
  if ha[1] == hb[1]
  {
    x = 0;
  }
  else if ha[1] > hb[1] 
  {
    x = 1;
  }
  else if ha[1] < hb[1]
  {
    x = 2;
  }
  else if ha[3] > hb[3]
  {
    x = 1;
  }
  else
  {
    x = 2;
  }
  
  x
}

fn tie_high_card(hand_a: [u32;5], hand_b:[u32;5]) -> u32
{
  let mut ha: [u32;5] = hand_a;
  let mut hb: [u32;5] = hand_b;

  ha.sort();
  hb.sort();

  let mut x: u32 = 0;
  for i in (0..5).rev()
  {
    if ha[i] == hb[i] 
    {
      x = 0;
    }
    else if ha[i] > hb[i] 
    {
      x = 1;
      break;
    }
    else 
    {
      x = 2;
      break;
    } 
  }
  
  x

}

///////////////////////////////////

fn get_tie(hand_a: [u32;5], hand_b: [u32;5], rank: u32) -> u32
{
  let mut var: u32 = 0; 
  if rank == 1 
  {
    var = tie_royal_flush(hand_a, hand_b);
  }
  else if rank == 2
  {
    var = tie_straight_flush(hand_a, hand_b);
  }
  else if rank == 3
  {
    var = tie_fr_of_kind(hand_a, hand_b);
  }
  else if rank == 4
  {
    var = tie_full_house(hand_a, hand_b);
  }
  else if rank == 5
  {
    var = tie_flush(hand_a, hand_b);
  }
  else if rank == 6
  {
    var = tie_straight(hand_a, hand_b);
  }
  else if rank == 7
  {
    var = tie_thr_of_kind(hand_a, hand_b);
  }
  else if rank == 8
  {
    var = tie_two_pair(hand_a, hand_b);
  }
  else if rank == 9
  {
    var = tie_pair(hand_a, hand_b);
  }
  else
  {
    var = tie_high_card(hand_a, hand_b);
  }
  
var

}


/////////////////////////////////////////
/*
fn get_ranks(hand: [u32;5]) -> u32
{
  //println!("Hand is {:?}",hand);
  let mut var: u32 = 0;
  if has_flush(hand) //== false
  {
    //println!("{}",has_flush(hand));
    //println!("Hand is(flush) {:?}",hand);
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
*/
//pub fn deal(perm:[u32;9]) -> Vec<String>

pub fn deal(perm:[u32;9]) -> Vec<String> 
{ 
  //code goes here
  let hand_a: [u32;2] = split_hand_a(perm);
  let hand_b: [u32;2] = split_hand_b(perm);
  let pool: [u32;5] = split_pool(perm);
  // let h_a: Vec<String> = convert(hand_A);
 // h_a

 let comb_a: [[u32;5];10] = get_comb(pool,hand_a);
 let comb_b: [[u32;5];10] = get_comb(pool,hand_b);
 
 let fin_hand_a: [u32;5] = get_final_hand(comb_a);
 let fin_hand_b: [u32;5] = get_final_hand(comb_b);
 
let winner_hand: u32 = get_winning_hand(fin_hand_a,fin_hand_b);


let h_a: Vec<String> = convert(fin_hand_a);
let h_b: Vec<String> = convert(fin_hand_b);

//println!("{:?}",h_a);
//println!("{:?}",h_b);

//let mut win_hand = Vec<String>new();
//let mut win_hand: Vec<String> = vec![];
//let win_hand: Vec<String> = Vec::new();

let mut k: u32 = 0;
//let mut j: u32 = 0;
if winner_hand == 3 
{
  k = get_tie(fin_hand_a,fin_hand_b,get_ranks(fin_hand_a));
}


let n = if winner_hand == 1 || k == 1
{
  h_a
}
else if winner_hand == 2 || k == 2
{
  h_b
}
else
{
  h_a
};

n

}
//Inside the else statement try running get_tie(final_hand_a,final_hand_b) then check if you get 1 or 2 and based on that just make this h_a or h_b the last thing inside the else {}

/*
fn main()
{
  println!("{:?}",deal([ 40, 41, 42, 43, 48, 49, 50, 51, 52 ]));
  //println!("{:?}",convert([1,2,3,4,5]));
}
*/