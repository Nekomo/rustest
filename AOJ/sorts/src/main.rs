use proconio::input;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
fn array_to_string(array:Vec<i32>)
{
  let st = String::new();
  for a in array{
    st+=a;
    st+=' ';
  }
  st
}
fn insertion_sort(array:&mut Vec<i32>){
    //for (i,a) in array.iter_mut().enumerate(){
    println!("{:?}",array);
    for i in 1 .. array.len(){
      //println!("val=({},{}),type=({},{})",i,a,type_of(i),type_of(a))
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
      //println!("{:?}",array);
      //println!("{},{}",i,insertpos);
      //②挿入
      for j in (insertpos..i).rev()
      {
        array.swap(j,j+1);
      }
      let arrstr=array_to_string(&array);
      println!("{}",arrstr);
    }
    println!("{:?}",array);
}

fn main() {

  let mut array=vec![5,4,3,2,1];
  insertion_sort(&mut array);
}
