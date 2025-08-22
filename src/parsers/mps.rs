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
enum Constraints {
    N, // !=
    L, // <
    G, // >
    E  // ==
}

struct Row {
    constraint: Constraints,
    name: String
}

struct Mps {
    name: String,
    rows: Vec<Row>
}



