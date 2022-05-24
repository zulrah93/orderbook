/*
   Author: Daniel Lopez
   Kraken Interview: Implement/Architect Order Book
*/

//MODULES:
mod helper;
mod tests;
//IMPORTS:
use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::{BufRead, BufReader};

use helper::constants;

//Helper method to convert status to a readable string for output purposes
fn output_status_to_str(status: helper::types::OutputStatus) -> Option<String> {
    match status {
        helper::types::OutputStatus::ACK => Some("A".to_string()),
        helper::types::OutputStatus::REJ => Some("R".to_string()),
        helper::types::OutputStatus::BES => Some("B".to_string()),
        helper::types::OutputStatus::TRA => Some("T".to_string()),
        _ => None,
    }
}

//Helper method to convert status to a readable string for output purposes
fn side_to_str(side: helper::types::Side) -> Option<String> {
    match side {
        helper::types::Side::BUY => Some("B".to_string()),
        helper::types::Side::SELL => Some("S".to_string()),
        _ => None,
    }
}

// Helper method to convert unicode character to its equivalent enum type
fn char_to_side(column : char) -> Option<helper::types::Side> { 
    match column {
        'A' => Some(helper::types::Side::BUY),
        'R' => Some(helper::types::Side::SELL),
        _ => None // Only if we pass an invalid input
    }
}

//Parses the price column and returns the price level and respective order type -- returns None if it fails to parse
fn parse_price_line(price_line: &String) -> Option<helper::types::ORDERTYPE_PRICE> {

    if price_line.starts_with("<>") { // Potentially a limit order
        if let Ok(price_level) = price_line.chars().skip(2).collect::<String>().parse::<u64>() {
            Some((helper::types::OrderType::LIMIT_ORDER, price_level))
        }
        else {
            None
        }
    }
    else if let Ok(price_level) = price_line.parse::<u64>() { // Parse entire line and assume market order with a valid integer value
            Some((helper::types::OrderType::MARKET_ORDER, price_level))
    }
    else { // Invalid or corrupted input
        None
    }

}

//Helper function that only prints to console if compiling in debugging rather than release mode -- will not compile on release if called somewhere in code
#[cfg(debug_assertions)]
fn debug_println(input: String) {
    println!("DEBUG: {}", input);
}

#[derive(Clone)]
struct Order {
    status: Cell<helper::types::TransactionStatus>, // Represents the order type; we use interior mutability to avoid having to pass a mutable reference as mutations have potential for uintended side effects
    client: u64, // Represents the client trading. ⚠️ 64-bit unsigned chosen to support a large amount of bids at the cost of memory and potentially using up more memory bandwith.
    ticker: String, // Represents the security typically stock traded the symbol is called a ticker symbol for example Microsoft is MSFT
    price: u64, // 64-bit unsigned chosen to support a near infinite bidding price.  May not be optimized for all archs.
    order_type: helper::types::OrderType, // Represents whether this order will bought at market price or a limit order will be set
    quantity: u64, // 64-bit unsigned chosen to support a near infinite quantity
    side: helper::types::Side, // Whether its a buy or sell order
    order_id: u64, // Represents the unique id for this order. u64 for maximum amount of orders
}

//Returned as a percent from 0-100 hence the unsigned byte return to save space
fn quoted_spread(ask_price: u64, bid_price: u64, midpoint_price: u64) -> u8 {
    (100 * ((ask_price - bid_price) / midpoint_price)) as u8 // Formula from: https://en.wikipedia.org/wiki/Bid%E2%80%93ask_spread
}

impl Order {
    fn new(
        status: helper::types::TransactionStatus,
        client: u64,
        ticker: String,
        price: u64,
        order_type: helper::types::OrderType,
        quantity: u64,
        side: helper::types::Side,
        order_id: u64,
    ) -> Self {
        Order {
            status: Cell::new(status),
            client: client,
            ticker: ticker,
            price,
            order_type,
            quantity,
            side: side,
            order_id: order_id,
        }
    }

    fn is_pk(&self, pk : helper::types::ORDER_PRIMARY_KEY) -> bool { // Useful for filter higher order function (HOF)
        self.get_pk() == pk
    }

    fn get_pk(&self) -> helper::types::ORDER_PRIMARY_KEY {
        (self.client, self.order_id)
    }

    //Used to update the Order status from New to Cancel for example
    fn set_order_status(&self, status: helper::types::TransactionStatus) {
        self.status.set(status);
    }

    //Returns the current state of the Order
    fn get_order_status(&self) -> helper::types::TransactionStatus {
        self.status.get()
    }

    // Parses the current CSV line will return None if current line is a comment, corrupted/invalid, or information is incomplete
    fn from(csv_line: &String) -> Option<Self> {
        if csv_line.starts_with(helper::constants::SKIP_LINE_CHAR) {
            None
        } else {
            //The columns where the data has not been transformed nor has been validated. We use map to convert the &str iterator to a String vector.
            let raw_columns = csv_line
                .split(helper::constants::CSV_COLUMN_SEPARATOR)
                .map(|column| String::from(column))
                .collect::<Vec<String>>();
            if raw_columns.len() == helper::constants::ORDER_COLUMN_COUNT {
                None      //TODO: Replace None with validation of each column
            }
            else {
                None
            }
        }
    }

    fn csv_output(&self) -> String {
        format!("") //TODO: Deserialize to String so that it maybe used for standard output
    }

    fn spread(&self, order: &Order) -> u64 {
        // Special case in the event we calculate the spread of an order on the same side should not happen in normal circumstances
        if self.side == order.side {
            return std::u64::MAX;
        }

        let ask = if self.side == helper::types::Side::SELL {
            // Ensure we refer to the ask order object
            self
        } else {
            order
        };
        let bid = if self.side == helper::types::Side::SELL {
            // Ensure we refer to the bid order object
            order
        } else {
            self
        };

        ask.price - bid.price
    }
}


// Represents a traditional order book found in a stock exchange like NYSE
struct OrderBook {
    //Using interior mutability but using RefCell since we are going to need a mutable reference to update the queue
    bids_ref: RefCell<Vec<Order>>,
    asks_ref: RefCell<Vec<Order>>
}

impl OrderBook {
    //Constructs an empty order book no bids or asks placed
    fn new() -> Self {
        OrderBook {
            bids_ref: RefCell::new(vec![]),
            asks_ref: RefCell::new(vec![]),
        }
    }

    fn build(&self, csv_line : &String) {
        if let Some(order) = Order::from(csv_line) {
            match order.side {
                helper::types::Side::BUY => {
                    //FIXME: Implement insertion logic -- currently inserting AS IS
                    let mut bids = self.bids_ref.borrow_mut();
                    bids.push(order);
                },
                helper::types::Side::SELL => {
                    //FIXME: Implement insertion logic
                    let mut asks = self.asks_ref.borrow_mut();
                    asks.push(order);
                }
            }
        }
        else {
            debug_println("FAILED TO PARSE LINE".to_string());
        }
    }

    fn calculate(&self) -> String {
        String::default()
    }

    fn flush(&self) { // Clears the order book
        self.bids_ref.borrow_mut().clear();
        self.asks_ref.borrow_mut().clear();
    }

    fn cancel_order(pk : helper::types::ORDER_PRIMARY_KEY) { // Cancels the order given a primary key

    }

}

//Stores the application arguments passed by the user
#[derive(Default, Debug)]
struct OrderBookConfiguration {
    orderbook_path: Option<String> // Input file that represents an orderbook in CSV format
}

impl OrderBookConfiguration {
    fn open(&self) -> Result<File, ()> {
        Err(())
    }

    fn new(input_path: &str) -> Self {
        OrderBookConfiguration {
            orderbook_path: Some(input_path.to_string()),
        }
    }
}

fn get_user_input() -> OrderBookConfiguration {
    use clap::{arg, Command}; // Placed here not to clutter global namespace
    let matches = Command::new("orderbook")
    .version(env!("CARGO_PKG_VERSION"))
    .author("Daniel Lopez <dlope073@gmail.com>")
    .about("This is a barebones implementation of a order book found in a typical stock exchange like NYSE")
    .arg(arg!(
        -c --csv <FILE> ... "CSV File which represents a typical orderbook"
    ))
    .get_matches();
    OrderBookConfiguration::new(if matches.is_present("csv") {
        matches.value_of("csv").unwrap_or_default()
    } else {
        ""
    })
}

fn main() {

    let args = get_user_input();

    debug_println(format!("args = {:?}", args));

    if let Ok(csv_file) = args.open() {

        let reader = BufReader::new(csv_file);
        let orderbook = OrderBook::new();
        for result in reader.lines() {
            if let Ok(csv_line) = result.as_ref() {
                orderbook.build(csv_line);
            }
        }
        println!("Order Book Scenario Output:\n{}",orderbook.calculate());

    }
}
