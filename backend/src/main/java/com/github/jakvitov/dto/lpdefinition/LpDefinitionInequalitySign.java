package com.github.jakvitov.dto.lpdefinition;

import com.github.jakvitov.mps.RowType;
import io.micronaut.serde.annotation.Serdeable;
import jakarta.annotation.Nonnull;

@Serdeable
public enum LpDefinitionInequalitySign {
    GE,
    LE,
    EQ,
    //Objective value
    N;

    public static LpDefinitionInequalitySign fromMpsRowType(@Nonnull RowType rowType) {
        return switch (rowType) {
            case E -> EQ;
            case L -> LE;
            case G -> GE;
            case N -> N;
        };
    }

}
