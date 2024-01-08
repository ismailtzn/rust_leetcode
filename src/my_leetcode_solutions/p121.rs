#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = prices.get(0).expect("There is no enough prices");
        let mut sell = prices.get(0).expect("There is no enough prices");
        let mut profit = sell - buy;
        
        for price in prices.iter() {
            if price < buy {
                buy = price;
                sell = price;
            }
            else if price > sell {
                sell = price;
            }
            if sell - buy > profit {
                profit = sell - buy;
            }
        }

        profit
    }
}

#[test]
fn test() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let res = 5;
    assert_eq!(Solution::max_profit(prices), res);
    let prices = vec![7, 6, 4, 3, 1];
    let res = 0;
    assert_eq!(Solution::max_profit(prices), res);
}