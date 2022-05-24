//CUSTOM TYPES/ENUMS PLACED HERE:
#[derive(Clone, Copy)]
pub enum TransactionStatus {
    NEW, // New order
    CAN, // Cancel order
    FLU, // Flush all orders
}

#[derive(Clone,Copy)]
pub enum OutputStatus {
    ACK, //Acknowledge Bid
    REJ, //Reject Bid
    BES, //Best or top of the book change
    TRA, // Trade
}


#[derive(Copy,Clone,PartialEq)] // So we can do equality comparison when we compare Order sides
pub enum Side {
    BUY,  // BID
    SELL, // ASK
}

#[derive(Clone,Copy)]
pub enum OrderType {
    MARKET_ORDER, // Buy at the current market price
    LIMIT_ORDER // Buy at that price level or below that amount
}

pub type ORDER_PRIMARY_KEY = (u64, u64); // Tuple representing the primary key of an Order which will be used when querying for a specific order

pub type ORDERTYPE_PRICE = (OrderType, u64); // Tuple representing the order type (MARKET/LIMIT) and price level expressed as a 64 bit unsigned integer
