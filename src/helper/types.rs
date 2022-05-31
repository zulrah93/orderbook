//CUSTOM TYPES/ENUMS PLACED HERE:
#[derive(Clone, Copy)]
pub enum TransactionType {
    NEW, // New order
    CAN, // Cancel order
    FLU, // Flush all orders
}

#[derive(Clone, Copy)]
pub enum OutputStatus {
    ACK, //Acknowledge Bid
    REJ, //Reject Bid
    BES, //Best or top of the book change
    TRA, // Trade
}

#[derive(Copy, Clone)] // So we can do equality comparison when we compare Order sides
pub enum Side {
    BUY,  // BID -- Better price means lowest price
    SELL, // ASK -- Better price means highest price
}

#[derive(Clone, Copy)]
pub enum OrderType {
    MARKET_ORDER, // Buy at the current market price
    LIMIT_ORDER,  // Buy at that price level or below that amount
}

pub type ORDER_PRIMARY_KEY = (u64, u64); // Tuple representing the primary key of an Order which will be used when querying for a specific order

pub type ORDERTYPE_PRICE = (OrderType, u64); // Tuple representing the order type (MARKET/LIMIT) and price level expressed as a 64 bit unsigned integer

// Represents the columns in their serialized form wrapped in an option to take into account errors in parsing
pub type SERIALIZED_COLUMNS = (
    Option<TransactionType>,
    Option<u64>,
    Option<String>,
    Option<ORDERTYPE_PRICE>,
    Option<u64>,
    Option<Side>,
    Option<u64>,
);
