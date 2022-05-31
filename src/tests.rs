use crate::helper::constants;
use crate::OrderBook;

#[cfg(test)]
mod tests {
    use crate::{
        helper::constants::{
            EXPECTED_EMPTY_ORDERBOOK, EXPECTED_SCENARIO_1, EXPECTED_SCENARIO_10,
            EXPECTED_SCENARIO_11, EXPECTED_SCENARIO_12, EXPECTED_SCENARIO_2, EXPECTED_SCENARIO_3,
            EXPECTED_SCENARIO_4, EXPECTED_SCENARIO_5, EXPECTED_SCENARIO_6, EXPECTED_SCENARIO_7,
            EXPECTED_SCENARIO_8, EXPECTED_SCENARIO_9,
        },
        OrderBook,
    };

    //Tests an empty order book -- not part of scenarios folder
    #[test]
    fn empty_scenario() {
        let orderbook = OrderBook::new();
        assert_eq!(orderbook.std_output(), EXPECTED_EMPTY_ORDERBOOK);
    }

    #[test]
    fn scenario1() {
        let orderbook = OrderBook::new();
        assert_eq!(orderbook.std_output(), EXPECTED_SCENARIO_1);
    }

    #[test]
    fn scenario2() {
        let orderbook = OrderBook::new();
        assert_eq!(orderbook.std_output(), EXPECTED_SCENARIO_2);
    }

    #[test]
    fn scenario3() {
        let orderbook = OrderBook::new();
        assert_eq!(orderbook.std_output(), EXPECTED_SCENARIO_3);
    }

    #[test]
    fn scenario4() {
        let orderbook = OrderBook::new();
        assert_eq!(orderbook.std_output(), EXPECTED_SCENARIO_4);
    }

    #[test]
    fn scenario5() {
        let orderbook = OrderBook::new();
        assert_eq!(orderbook.std_output(), EXPECTED_SCENARIO_5);
    }

    #[test]
    fn scenario6() {
        let orderbook = OrderBook::new();
        assert_eq!(orderbook.std_output(), EXPECTED_SCENARIO_6);
    }

    #[test]
    fn scenario7() {
        let orderbook = OrderBook::new();
        assert_eq!(orderbook.std_output(), EXPECTED_SCENARIO_7);
    }

    #[test]
    fn scenario8() {
        let orderbook = OrderBook::new();
        assert_eq!(orderbook.std_output(), EXPECTED_SCENARIO_8);
    }

    #[test]
    fn scenario9() {
        let orderbook = OrderBook::new();
        assert_eq!(orderbook.std_output(), EXPECTED_SCENARIO_9);
    }

    #[test]
    fn scenario10() {
        let orderbook = OrderBook::new();
        assert_eq!(orderbook.std_output(), EXPECTED_SCENARIO_10);
    }

    #[test]
    fn scenario11() {
        let orderbook = OrderBook::new();
        assert_eq!(orderbook.std_output(), EXPECTED_SCENARIO_11);
    }

    #[test]
    fn scenario12() {
        let orderbook = OrderBook::new();
        assert_eq!(orderbook.std_output(), EXPECTED_SCENARIO_12);
    }
}
