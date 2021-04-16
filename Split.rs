
fn split_hand_a(arr: [i32;9]) -> [i32;2]
{
  let mut hand_a: [i32;2] = [0;2];
  hand_a = [arr[0],arr[2]];
  hand_a
}

fn split_hand_b(arr: [i32;9]) -> [i32;2]
{
  let mut hand_b: [i32;2] = [0;2];
  hand_b = [arr[1],arr[3]];
  hand_b
}

fn split_pool(arr: [i32;9]) -> [i32;5]
{
  let mut pool: [i32;5] = [0;5];
  pool = [arr[4],arr[5],arr[6],arr[7],arr[8]];
  pool
}

 // [[i32;5];10] return type?

  fn get_comb(pool: [i32;5], hand: [i32;2]) ->[[i32;5];10]
  {
    let mut comb: [[i32;5];10] = [[0;5];10];

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



fn main()
{
  let array: [i32;9] = [1,2,3,4,5,6,7,8,9];
  let p: [i32;5] = [1,2,3,4,5];
  let h: [i32;2] = [7,8];
  println!("{:?}", split_hand_a(array));
  println!("{:?}", split_hand_b(array));
  println!("{:?}", split_pool(array));
  println!("{:?}", get_comb(p,h));
}