// 定数
const const_fruits: &str = "Banana";
const const_Number: u32 = 100_000;

pub fn mutble_immutble() {
    let fruits = "apple";
    let result = fruits;
    println!("result {}", result);
    /*
    letはデフォルトでは定数になっているので代入は無理
    fruits = "applepen";
    */

    // mutをつけることによって可変となる
    let mut mut_fruits = "mut_apple";
    println!("変更前: {}", mut_fruits);
    mut_fruits = "mut_apple_pen";
    println!("変更後: {}", mut_fruits);

    // 使用していない値だと警告が出るから頭に_をつけると警告が消える(意図敵であることを明示する)
    let _none = "";

    // 定数
    println!("{}", const_fruits);

    // システムのサイズ
    println!("{}", usize::BITS);

    // メモリーのアドレス
    println!("const_fruitsのメモリーのアドレス: {:p}", &const_fruits);
    println!("const_Numberのメモリーのアドレス: {:p}", &const_Number);

    // スタックのアドレス
    let orange = "オレンジ";
    let grape = "グレープ";
    println!("orangeのメモリーのアドレス: {:p}", &orange);
    println!("grapeのメモリーのアドレス: {:p}", &grape);

    // シャドーイング
    let shadowing = 1;
    println!("shadowingバインド前のメモリーのアドレス: {:p}", &shadowing);

    let shadowing = shadowing + 1;
    println!("shadowingバインド後のメモリーのアドレス: {:p}", &shadowing);
    println!("shadowingバインド後の値: {}", shadowing);

    {
        let shadowing = 0;
        println!("shadowing別スコープのメモリーのアドレス: {:p}", &shadowing);
        println!("shadowingバインド後の値: {}", shadowing);
    }
    println!("shadowing別スコープのメモリーのアドレス: {:p}", &shadowing);
    println!("shadowingバインド後の値: {}", shadowing);

    // タプル
    let tuple_sumple = (0, 100.0, "タプル");

    // タプルの取り出し方使い方
    let (zero, handred, tuple) = tuple_sumple;

    println!("タプル: {} {} {}", zero, handred, tuple);

    println!(
        "タプル: {} {} {}",
        tuple_sumple.0, tuple_sumple.1, tuple_sumple.2
    );

    // タプルの入れ子
    let mut tuples_sumple = ((1, 100.0, "タプル1"), (2, 200.0, "タプル2"));

    // 各値だけじゃなくてポインターも取得する
    let ((ref mut tuples1Num, ref tuples2Float, ref tuples3Str), _) = tuples_sumple;
    println!("タプル: {} {} {}", tuples1Num, tuples2Float, tuples3Str);

    // 配列
    let listA = [0, 1, 2, 3, 4, 5];
    let listB = [0; 10]; // この書き方は、この場合0を10個作る
    println!("配列A: {:?} {} {}", listA, listA[0], listA[4]);
    println!("配列B: {:?} {} {}", listB, listB[2], listB[9]);
}
