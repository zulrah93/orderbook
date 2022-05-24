//HELPER CONSTANTS
pub static SKIP_LINE_CHAR: char = '#'; // Since CSV files do not allow comments we shall ignore lines that start with the pound/hashtag symbol.
pub static CSV_COLUMN_SEPARATOR: char = ',';
pub static ORDER_COLUMN_COUNT: usize = 7;

//EXPECTED CONSTANTS USED BY UNIT TESTS ONLY
pub static EXPECTED_EMPTY_ORDERBOOK: &str = "";

pub static EXPECTED_SCENARIO_1 : &str = 
r#"
A, 1, 1
B, B, 10, 100
A, 1, 2
B, S, 12, 100
A, 2, 101
A, 2, 102
B, S, 11, 100
R, 1, 3
R, 2, 103
A, 1, 4
B, B, 10, 200
A, 2, 104
B, S, 11, 200
"#;

pub static EXPECTED_SCENARIO_2 : &str = 
r#"
A, 1, 1
B, B, 10, 100
A, 1, 2
B, S, 12, 100
A, 2, 101
A, 2, 102
B, S, 11, 100
R, 1, 3
R, 2, 103
A, 1, 4
B, B, 10, 200
A, 2, 104
B, S, 11, 200
"#;

pub static EXPECTED_SCENARIO_3 : &str = 
r#"
A, 1, 1
B, B, 10, 100
A, 1, 2
B, S, 12, 100
A, 2, 101
A, 2, 102
B, S, 11, 100
R, 1, 3
R, 2, 103
A, 1, 4
B, B, 10, 200
A, 2, 104
B, S, 11, 200
"#;

pub static EXPECTED_SCENARIO_4 : &str = 
r#"
A, 1, 1
B, B, 10, 100
A, 1, 2
B, S, 12, 100
A, 2, 101
A, 2, 102
B, S, 11, 100
R, 1, 3
R, 2, 103
A, 1, 4
B, B, 10, 200
A, 2, 104
B, S, 11, 200
"#;

pub static EXPECTED_SCENARIO_5 : &str = 
r#"
A, 1, 1
B, B, 10, 100
A, 1, 2
B, S, 12, 100
A, 2, 101
A, 2, 102
B, S, 11, 100
R, 1, 3
R, 2, 103
A, 1, 4
B, B, 10, 200
A, 2, 104
B, S, 11, 200
"#;

pub static EXPECTED_SCENARIO_6 : &str = 
r#"
A, 1, 1
B, B, 10, 100
A, 1, 2
B, S, 12, 100
A, 2, 101
A, 2, 102
B, S, 11, 100
R, 1, 3
R, 2, 103
A, 1, 4
B, B, 10, 200
A, 2, 104
B, S, 11, 200
"#;

pub static EXPECTED_SCENARIO_7 : &str = 
r#"
A, 1, 1
B, B, 10, 100
A, 1, 2
B, S, 12, 100
A, 2, 101
A, 2, 102
B, S, 11, 100
R, 1, 3
R, 2, 103
A, 1, 4
B, B, 10, 200
A, 2, 104
B, S, 11, 200
"#;

pub static EXPECTED_SCENARIO_8 : &str = 
r#"
A, 1, 1
B, B, 10, 100
A, 1, 2
B, S, 12, 100
A, 2, 101
A, 2, 102
B, S, 11, 100
R, 1, 3
R, 2, 103
A, 1, 4
B, B, 10, 200
A, 2, 104
B, S, 11, 200
"#;

pub static EXPECTED_SCENARIO_9 : &str = 
r#"
A, 1, 1
B, B, 10, 100
A, 1, 2
B, S, 12, 100
A, 2, 101
A, 2, 102
B, S, 11, 100
R, 1, 3
R, 2, 103
A, 1, 4
B, B, 10, 200
A, 2, 104
B, S, 11, 200
"#;

pub static EXPECTED_SCENARIO_10 : &str = 
r#"
A, 1, 1
B, B, 10, 100
A, 1, 2
B, S, 12, 100
A, 2, 101
A, 2, 102
B, S, 11, 100
R, 1, 3
R, 2, 103
A, 1, 4
B, B, 10, 200
A, 2, 104
B, S, 11, 200
"#;

pub static EXPECTED_SCENARIO_11 : &str = 
r#"
A, 1, 1
B, B, 10, 100
A, 1, 2
B, S, 12, 100
A, 2, 101
A, 2, 102
B, S, 11, 100
R, 1, 3
R, 2, 103
A, 1, 4
B, B, 10, 200
A, 2, 104
B, S, 11, 200
"#;

pub static EXPECTED_SCENARIO_12 : &str = 
r#"
A, 1, 1
B, B, 10, 100
A, 1, 2
B, S, 12, 100
A, 2, 101
A, 2, 102
B, S, 11, 100
R, 1, 3
R, 2, 103
A, 1, 4
B, B, 10, 200
A, 2, 104
B, S, 11, 200
"#;

