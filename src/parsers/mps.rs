/// Sections in which is the MPS format devided
#[derive(PartialEq, Debug, Clone)]
pub enum Sections {
    NAME,
    ROWS,
    COLUMNS,
    RHS,
    BOUNDS,
    ENDATA
}


/// Constrains used in the MPS format
/// N - objective
/// L - less than or equal to constraint
/// G - greater than or equal to constraint
/// E - equality constraint
enum Constraints {
    N,
    L, // <
    G, // >
    E  // ==
}

pub struct Row {
    constraint: Constraints,
    name: String
}

pub struct Column {

}

pub struct Bound {

}

pub struct Rhs {

}

pub struct MpsModel {
    name: String,
    rows: Vec<Row>
}



