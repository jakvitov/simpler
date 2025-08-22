/// Sections in which is the MPS format devided
enum Sections {
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

struct Row {
    constraint: Constraints,
    name: String
}

pub struct MpsModel {
    name: String,
    rows: Vec<Row>
}



