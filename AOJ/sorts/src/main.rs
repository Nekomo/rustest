use std::io::stdin;

fn insertion_sort(array:&mut Vec<i32>){
    //println!("{:?}",array);
    print_array(array.to_vec());
    for i in 1 .. array.len(){
      //①位置決定
      let mut insertpos:usize=114514;
      for j in (0..i).rev(){
          if array[j]<array[i]{
            insertpos=j+1;
            break;
          }
          if j==0{
            insertpos=0;
          }
      }
      //②挿入
      for j in (insertpos..i).rev()
      {
        array.swap(j,j+1);
      }
      print_array(array.to_vec());
    }
    //print_array(array.to_vec());
}

fn main() {
  //入力を受け取ってi32に変換
  let n:i32 = readline().parse().unwrap();
  //println!("{}",n);
  //let mut array=vec![5,4,3,2,1];
  let mut array=readline_as_vec();
  insertion_sort(&mut array);
}

fn print_array(array:Vec<i32>){
  let mut array_content=String::new();
  for a in array{
    array_content += &a.to_string();
    array_content += " ";
  }
  println!("{}",array_content.to_string().trim_right());
}
fn readline()->String
{
  let mut s = String::new();
  stdin().read_line(&mut s).unwrap();
  //改行コードの削除
  s.trim_right().to_owned()
}

fn readline_as_vec()->Vec<i32>
{
  let mut s = String::new();
  stdin().read_line(&mut s).unwrap();
  //改行コードの削除
  s=s.trim_right().to_owned();
  let mut read_vec:Vec<i32>=Vec::new();
  //空白で分割
  for chr in s.split_whitespace(){
    read_vec.push(chr.parse::<i32>().unwrap());
  }
  read_vec
}