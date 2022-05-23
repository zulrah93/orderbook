/*
   Author: Daniel Lopez
   Kraken Interview: Implement/Architect Order Book
*/

use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::{BufRead, BufReader};

//HELPER CONSTANTS
static SKIP_LINE_CHAR: char = '#'; // Since CSV files do not allow comments we shall ignore lines that start with the pound/hashtag symbol.
static CSV_COLUMN_SEPARATOR: char = ','; // TODO: Support the various seperators but comma is the most common.

#[derive(Clone, Copy)]
enum TransactionStatus {
    NEW, // New order
    CAN, // Cancel order
    FLU, // Flush all orders
}

enum OutputStatus {
    ACK, //Acknowledge Bid
    REJ, //Reject Bid
    BES, //Best or top of the book change
    TRA, // Trade
}

//Helper method to convert status to a readable string for output purposes
fn output_status_to_str(status: OutputStatus) -> Option<String> {
    match status {
        OutputStatus::ACK => Some("A".to_string()),
        OutputStatus::REJ => Some("R".to_string()),
        OutputStatus::BES => Some("B".to_string()),
        OutputStatus::TRA => Some("T".to_string()),
        _ => None,
    }
}

#[derive(Copy,Clone,PartialEq)] // So we can do equality comparison when we compare Order sides
enum Side {
    BUY,  // BID
    SELL, // ASK
}

//Helper method to convert status to a readable string for output purposes
fn side_to_str(side: Side) -> Option<String> {
    match side {
        Side::BUY => Some("A".to_string()),
        Side::SELL => Some("R".to_string()),
        _ => None,
    }
}

//Helper function that only prints to console if compiling in debugging rather than release mode
#[cfg(debug_assertions)]
fn debug_println(input: String) {
    println!("DEBUG: {}", input);
}

#[derive(Clone)]
struct Order {
    status: Cell<TransactionStatus>, // Represents the order type; we use interior mutability to avoid having to pass a mutable reference as mutations have potential for uintended side effects
    client: u64, // Represents the client trading. ⚠️ 64-bit unsigned chosen to support a large amount of bids at the cost of memory and potentially using up more memory bandwith.
    ticker: String, // Represents the security typically stock traded the symbol is called a ticker symbol for example Microsoft is MSFT
    price: u64, // 64-bit unsigned chosen to support a near infinite bidding price.  May not be optimized for all archs.
    quantity: u64, // 64-bit unsigned chosen to support a near infinite quantity
    side: Side, // Whether its a buy or sell order
    order_id: u64, // Represents the unique id for this order. u64 for maximum amount of orders
}

type ORDER_PRIMARY_KEY = (u64, u64); // Tuple representing the primary key of an Order which will be used when querying for a specific order

//Returned as a percent from 0-100 hence the unsigned byte return to save space
fn quoted_spread(ask_price: u64, bid_price: u64, midpoint_price: u64) -> u8 {
    (100 * ((ask_price - bid_price) / midpoint_price)) as u8 // Formula from: https://en.wikipedia.org/wiki/Bid%E2%80%93ask_spread
}

impl Order {
    fn new(
        status: TransactionStatus,
        client: u64,
        ticker: String,
        price: u64,
        quantity: u64,
        side: Side,
        order_id: u64,
    ) -> Self {
        Order {
            status: Cell::new(status),
            client: client,
            ticker: ticker,
            price,
            quantity,
            side: side,
            order_id: order_id,
        }
    }

    fn get_pk(&self) -> ORDER_PRIMARY_KEY {
        (self.client, self.order_id)
    }

    // Returns true if the Order primary key is a match
    fn pk_matches(&self, order_pk: ORDER_PRIMARY_KEY) -> bool {
        (self.client, self.order_id) == order_pk
    }

    //Used to update the Order status from New to Cancel for example
    fn set_order_status(&self, status: TransactionStatus) {
        self.status.set(status);
    }

    //Returns the current state of the Order
    fn get_order_status(&self) -> TransactionStatus {
        self.status.get()
    }

    // Parses the current CSV line will return None if current line is a comment, corrupted/invalid, or information is incomplete
    fn from(csv_line: &String) -> Option<Self> {
        if csv_line.starts_with(SKIP_LINE_CHAR) {
            None
        } else {
            //The columns where the data has not been transformed nor has been validated. We use map to convert the &str iterator to a String vector.
            let raw_column = csv_line
                .split(CSV_COLUMN_SEPARATOR)
                .map(|column| String::from(column))
                .collect::<Vec<String>>();
            None //TODO: Implement logic
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

        let ask = if self.side == Side::SELL {
            // Ensure we refer to the ask order object
            self
        } else {
            order
        };
        let bid = if self.side == Side::SELL {
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
    asks_ref: RefCell<Vec<Order>>,
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
                Side::BUY => {
                    //FIXME: Implement insertion logic -- currently inserting AS IS
                    let mut bids = self.bids_ref.borrow_mut();
                    bids.push(order);
                },
                Side::SELL => {
                    //FIXME: Implement insertion logic
                    let mut bids = self.bids_ref.borrow_mut();
                    bids.push(order);
                }
            }
        }
        else {
            debug_println("FAILED TO PARSE LINE".to_string());
        }
    }

    fn print_stdout(&self) {
        println!("Orderbook: {{}}");
    }

}

//Stores the application arguments passed by the user
#[derive(Default, Debug)]
struct OrderBookConfiguration {
    orderbook_path: Option<String>, // Input file that represents an orderbook in CSV format
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
        orderbook.print_stdout();

    }
}
