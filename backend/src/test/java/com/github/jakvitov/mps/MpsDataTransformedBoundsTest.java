package com.github.jakvitov.mps;

import org.hipparchus.fraction.BigFraction;
import org.junit.jupiter.api.Test;

public class MpsDataTransformedBoundsTest {


    @Test
    public void mps_data_transformed_bounds_succeeds() {
        final String mpsInput = """
                NAME          TESTPROB
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
        mpsData.validate();
        MpsDataTransformedBounds mpsDataTransformedBounds = new MpsDataTransformedBounds(mpsData);

        assert mpsDataTransformedBounds.rows.get("XONE-UP").equals(RowType.L);
        assert mpsDataTransformedBounds.rows.get("YTWO-UP").equals(RowType.L);
        assert mpsDataTransformedBounds.rows.get("YTWO-LO").equals(RowType.L);

        assert mpsDataTransformedBounds.columns.get("XONE").get("XONE-UP").equals(BigFraction.ONE);
        assert mpsDataTransformedBounds.columns.get("YTWO").get("YTWO-UP").equals(BigFraction.ONE);
        assert mpsDataTransformedBounds.columns.get("YTWO").get("YTWO-LO").equals(BigFraction.ONE.negate());

        assert mpsDataTransformedBounds.rhs.get("XONE-UP").equals(new BigFraction(4));
        assert mpsDataTransformedBounds.rhs.get("YTWO-UP").equals(BigFraction.ONE);
        assert mpsDataTransformedBounds.rhs.get("YTWO-LO").equals(BigFraction.ONE);
    }

    @Test
    public void mps_data_transformed_does_not_fail_on_no_bounds() {
        final String mpsInput = """
                NAME          TESTPROB
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
                ENDATA
                """;

        MpsData mpsData = MpsData.parse(mpsInput);
        mpsData.validate();
        MpsDataTransformedBounds mpsDataTransformedBounds = new MpsDataTransformedBounds(mpsData);
    }

}
