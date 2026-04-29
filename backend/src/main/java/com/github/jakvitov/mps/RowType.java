package com.github.jakvitov.mps;

public enum RowType {
    N,
    L,
    G,
    E;

    public RowType flip() {
        return switch (this) {
            case N -> N;
            case L -> G;
            case G -> L;
            case E -> E;
        };
    }
}
