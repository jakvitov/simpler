package com.github.jakvitov.mps;

import org.junit.jupiter.api.Test;

public class MpsDataTransformedBoundsTest {


    @Test
    public void mps_data_transformed_bounds_constructor_succeeds() {
        final String mpsInput = """
                NAME          TESTPROB
                OBJSENSE    MAX
                ROWS
                 N  COST
                 L  LIM1
                 G  LIM2
                 E  MYEQN
                COLUMNS
                    XONE      COST                 1/2   LIM1                 1
                    XONE      LIM2                 1
                    YTWO      COST                 4   LIM1                 1
                    YTWO      MYEQN               -1/2
                    ZTHREE    COST                 9   LIM2                 1
                    ZTHREE    MYEQN                1
                RHS
                    RHS1      LIM1                 5   LIM2                10
                    RHS1      MYEQN                7
                BOUNDS
                 UP BND1      XONE                 4
                 LO BND1      YTWO                -1
                 UP BND1      YTWO                 1
                ENDATA
                """;

        MpsData mpsData = MpsData.parse(mpsInput);

        MpsDataTransformedBounds mpsDataTransformedBounds = new MpsDataTransformedBounds(mpsData);
        System.out.println(mpsDataTransformedBounds);
    }

}
