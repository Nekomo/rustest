//乱数を生成する「randクレート」を外部依存として使用する
extern crate rand;

//入力を受け付ける「ioライブラリ」をインポート
use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    //println! ... 文字列を表示する「マクロ」
    //マクロ呼び出しの際はマクロ名の後ろに"!"をつける
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is {}",secret_number);

    loop{
        println!("Please input your guess.");
        
        //mutでミュータブルになる
        let mut guess=String::new();
        //io::stdin().read_lineには、guessの参照を渡してあげる
        //参照もデフォルトでは不変
        io::stdin().read_line(&mut guess)
            //result型の列挙子(Ok/Err)が帰ってくる
            //expectメソッドは、Errの場合にプログラムをクラッシュさせ、引数として渡されたメッセージを表示する
            .expect("Failed to read line.");
        //trimで改行を削除できる
        //コロンで型注釈
        let guess:u32=match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
            //.expect("Please type a number!");
        println!("You guessed : {}",guess);


        match guess.cmp(&secret_number)
        {
            Ordering::Less=>println!("Too small!"),
            Ordering::Greater=>println!("Too big!"),
            Ordering::Equal=>{
                println!("You win!");
                break;
            },
        }
    }
}
