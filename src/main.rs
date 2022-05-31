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
use std::ops::SubAssign;

//TODO: Move interfaces into their own module

#[derive(Clone)]
struct Order {
    status: Cell<helper::types::TransactionType>, // Represents the order type; we use interior mutability to avoid having to pass a mutable reference as mutations have potential for uintended side effects
    client: u64, // Represents the client trading. ⚠️ 64-bit unsigned chosen to support a large amount of bids at the cost of memory and potentially using up more memory bandwith.
    ticker: String, // Represents the security typically stock traded the symbol is called a ticker symbol for example Microsoft is MSFT
    price: u64, // 64-bit unsigned chosen to support a near infinite bidding price.  May not be optimized for all archs.
    order_type: helper::types::OrderType, // Represents whether this order will bought at market price or a limit order will be set
    quantity: Cell<u64>, // 64-bit unsigned chosen to support a near infinite quantity 
    side: helper::types::Side, // Whether its a buy or sell order
    order_id: u64, // Represents the unique id for this order. u64 for maximum amount of orders
}

impl Order {
    fn new(
        status: helper::types::TransactionType,
        client: u64,
        ticker: String,
        price: u64,
        order_type: helper::types::OrderType,
        quantity: Cell<u64>,
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

    fn from_serialized_columns(
        serialized_columns: &helper::types::SERIALIZED_COLUMNS,
    ) -> Option<Self> {
        if helper::methods::has_invalid_column(&serialized_columns) {
            None
        } else {
            let (status, client, ticker, type_price, quantity, side, order_id) =
                serialized_columns.clone();
            let (order_type, price) = type_price.unwrap();
            helper::methods::debug_println(format!(
                "[Price: ${}, Type: {}]",
                price,
                helper::methods::order_type_to_string(order_type)
            ));
            Some(Order::new(
                status.unwrap(),
                client.unwrap(),
                ticker.unwrap(),
                price,
                order_type,
                Cell::new(quantity.unwrap()),
                side.unwrap(),
                order_id.unwrap(),
            ))
        }
    }

    fn is_pk(&self, pk: helper::types::ORDER_PRIMARY_KEY) -> bool {
        // Useful for filter higher order function (HOF)
        self.get_pk() == pk
    }

    fn get_pk(&self) -> helper::types::ORDER_PRIMARY_KEY {
        (self.client, self.order_id)
    }

    //Updates the quantity of the order
    fn update_quantity(&self, new_quantity : u64) {
        self.quantity.set(new_quantity);
    }

    //Checks if the order is completly fufilled
    fn is_empty(&self) -> bool {
        self.quantity.get() > 0
    }

    //Used to update the Order status from New to Cancel for example
    fn set_order_status(&self, status: helper::types::TransactionType) {
        self.status.set(status);
    }

    //Returns the current state of the Order
    fn get_order_status(&self) -> helper::types::TransactionType {
        self.status.get()
    }

    // Parses the current CSV line will return None if current line is a comment, corrupted/invalid, or information is incomplete
    // Note: Only parses a new order a cancel order is checked before calling this
    fn from(csv_line: &String) -> Option<Self> {
        if csv_line.starts_with(helper::constants::SKIP_LINE_CHAR) {
            None
        } else {
            //The columns where the data has not been transformed nor has been validated. We use map to convert the &str iterator to a String vector.
            let raw_columns = csv_line
                .split(helper::constants::CSV_COLUMN_SEPARATOR)
                .map(helper::methods::trim_string) // Avoid any whitespaces in our columns
                .collect::<Vec<String>>();
            helper::methods::debug_println(format!("raw_columns: {:?}", raw_columns));
            if raw_columns.len() == helper::constants::NEW_ORDER_COLUMN_COUNT {
                // Grab the indivdual column data in their raw format -- some we only care about the first character if the information is encoded in a byte
                let order_type_char = raw_columns[0].chars().next().unwrap_or_default();
                let client_name_raw = &raw_columns[1];
                let ticker = &raw_columns[2].to_string();
                let price_line_raw = &raw_columns[3];
                let quantity_raw = &raw_columns[4];
                let side_char = raw_columns[5].chars().next().unwrap_or_default();
                let order_id_raw = &raw_columns[6];
                let serialized_columns: helper::types::SERIALIZED_COLUMNS = (
                    helper::methods::char_to_transaction_type(order_type_char),
                    helper::methods::parse_u64_from(client_name_raw),
                    Some(ticker.to_string()),
                    helper::methods::parse_price_line(price_line_raw),
                    helper::methods::parse_u64_from(quantity_raw),
                    helper::methods::char_to_side(side_char),
                    helper::methods::parse_u64_from(order_id_raw),
                );
                Order::from_serialized_columns(&serialized_columns)
            } else {
                None
            }
        }
    }
}

/*
 So we can use vec.sort() and avoid having to use vec.sort_by()
*/
impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.get_pk().eq(&other.get_pk())
    }
}
impl Eq for Order {}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.price.cmp(&other.price)
    }
}

// Represents a traditional order book found in a stock exchange like NYSE
struct OrderBook {
    //Using interior mutability but using RefCell since we are going to need a mutable reference to update the bids/asks and store the result for standard output
    bids_ref: RefCell<Vec<Order>>,
    asks_ref: RefCell<Vec<Order>>,
    std_out: RefCell<String>,
}

impl Default for OrderBook {
    fn default() -> Self {
        Self {
            bids_ref: RefCell::new(vec![]),
            asks_ref: RefCell::new(vec![]),
            std_out: RefCell::new(String::default()),
        }
    }
}

impl OrderBook {
    //Constructs an empty order book no bids or asks placed
    fn new() -> Self {
        OrderBook::default()
    }

    fn build(&self, csv_line: &String) {
        helper::methods::debug_println(format!("Current CSV Line: {}", csv_line));
        if let Some(order) = Order::from(csv_line) {
            match order.side {
                helper::types::Side::BUY => {
                    //TODO: Use a more efficient algorithm if time permits
                    let mut bids = self.bids_ref.borrow_mut();
                    bids.push(order);
                    bids.sort() // This sort is stable (i.e., does not reorder equal elements) and O(n * log(n)) worst-case. Source: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.sort
                }
                helper::types::Side::SELL => {
                    //TODO: Use a more efficient algorithm if time permits
                    let mut asks = self.asks_ref.borrow_mut();
                    asks.push(order);
                    asks.sort(); // This sort is stable (i.e., does not reorder equal elements) and O(n * log(n)) worst-case. Source: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.sort
                }
            }
        } else {
            helper::methods::debug_println("FAILED TO PARSE LINE".to_string());
        }
    }

    fn reformat(&self) {
        // Flushes the order book
        self.bids_ref.borrow_mut().clear();
        self.asks_ref.borrow_mut().clear();
    }

    //Used for final results
    fn std_output(&self) -> String {
        self.std_out.borrow().clone() // We don't want to move the shared reference out so we clone. Space complexitiy is O(L) where L is the length of the string
    }

    fn cancel_order(&self, pk: helper::types::ORDER_PRIMARY_KEY) {
        // Cancels the order given a primary key
        let mut bids = self.bids_ref.borrow_mut();
        let mut asks = self.asks_ref.borrow_mut();
        if let Some(deletion_index) = bids
            .iter()
            .enumerate()
            .filter(|o| o.1.is_pk(pk))
            .map(|x| x.0)
            .next()
        {
            bids.remove(deletion_index);
        } else if let Some(deletion_index) = asks
            .iter()
            .enumerate()
            .filter(|o| o.1.is_pk(pk))
            .map(|x| x.0)
            .next()
        {
            asks.remove(deletion_index);
        }
    }

}

//Stores the application arguments passed by the user
#[derive(Default, Debug)]
struct OrderBookConfiguration {
    orderbook_path: Option<String>, // Input file that represents an orderbook in CSV format
}

impl OrderBookConfiguration {
    fn open(&self) -> Result<File, std::io::Error> {
        if let Some(path) = self.orderbook_path.as_ref() {
            File::open(path)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found or invalid path was given!",
            ))
        }
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
    helper::methods::debug_println(format!("args = {:?}", args));
    if let Ok(csv_file) = args.open() {
        let reader = BufReader::new(csv_file);
        let orderbook = OrderBook::new();
        for result in reader.lines() {
            if let Ok(csv_line) = result.as_ref() {
                orderbook.build(csv_line);
            }
        }
    }
}