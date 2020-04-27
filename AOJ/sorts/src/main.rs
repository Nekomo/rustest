use proconio::input;

fn insertion_sort(array:&mut Vec<i32>){
    println!("{:?}",array);
    for i in 1 .. array.len(){
      //①位置決定
      let mut insertpos:usize=i;
      for j in (0..i).rev(){
          if array[j]<array[i]{
            insertpos=i+1;
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
      println!("{:?}",array);
    }
    println!("{:?}",array);
}

fn main() {

  let mut array=vec![5,4,3,2,1];
  insertion_sort(&mut array);
}