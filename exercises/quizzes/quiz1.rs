// これは以下のセクションのクイズです：
// - Variables
// - Functions
// - If
//
// メアリーはりんごを買っています。りんごの価格は以下のように計算されます：
// - りんご1個は2 rustbucksかかります
// - 40個より多くリンゴを買ったら、りんご1個の値段は1 rustbucksになります。

use core::num;

// TODO: 与えられたりんごの注文数の金額を計算する関数を書いてください。
// fn calculate_price_of_apples(???) -> ??? { ??? }
fn calculate_price_of_apples(num_apple: u32) -> u32 {
    if num_apple > 40 {
        1 * num_apple
    } else {
        2 * num_apple
    }
}

fn main() {
    // この行で関数のテストができます。
}

// このテストは変更しないでください。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
