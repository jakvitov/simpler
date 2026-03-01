package com.github.jakvitov.mps;

import org.hipparchus.fraction.BigFraction;
import org.junit.jupiter.api.Test;

public class MpsDataTest {

    @Test
    public void parse_correct_mps_succeeds() {
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

        assert mpsData.rows.get("COST").equals(RowType.N);
        assert mpsData.rows.get("LIM1").equals(RowType.L);
        assert mpsData.rows.get("LIM2").equals(RowType.G);
        assert mpsData.rows.get("MYEQN").equals(RowType.E);

        assert mpsData.columns.containsKey("XONE");
        assert mpsData.columns.containsKey("YTWO");
        assert mpsData.columns.containsKey("ZTHREE");

        assert mpsData.columns.get("XONE").get("COST").equals(BigFraction.ONE_HALF);
        assert mpsData.columns.get("XONE").get("LIM1").equals(BigFraction.ONE);
        assert mpsData.columns.get("XONE").get("LIM2").equals(BigFraction.ONE);

        assert mpsData.columns.get("YTWO").get("COST").equals(new BigFraction(4));
        assert mpsData.columns.get("YTWO").get("LIM1").equals(BigFraction.ONE);
        assert mpsData.columns.get("YTWO").get("MYEQN").equals(BigFraction.ONE_HALF.negate());

        assert mpsData.columns.get("ZTHREE").get("COST").equals(new BigFraction(9));
        assert mpsData.columns.get("ZTHREE").get("LIM2").equals(BigFraction.ONE);
        assert mpsData.columns.get("ZTHREE").get("MYEQN").equals(BigFraction.ONE);

        assert mpsData.rhs.size() == 1;
        assert mpsData.rhs.get("RHS1") != null;

        assert mpsData.rhs.get("RHS1").get("LIM1").equals(new BigFraction(5));
        assert mpsData.rhs.get("RHS1").get("LIM2").equals(new BigFraction(10));
        assert mpsData.rhs.get("RHS1").get("MYEQN").equals(new BigFraction(7));

        assert mpsData.bounds.size() == 1;
        assert mpsData.bounds.get("BND1").size() == 2;
        assert mpsData.bounds.get("BND1").get("XONE").size() == 1;
        assert mpsData.bounds.get("BND1").get("XONE").get(BoundType.UP).equals(new BigFraction(4));
        assert mpsData.bounds.get("BND1").get("YTWO").get(BoundType.LO).equals(new BigFraction(-1));
        assert mpsData.bounds.get("BND1").get("YTWO").get(BoundType.UP).equals(new BigFraction(1));
    }

    @Test
    public void verify_correct_mps_succeeds() {
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
        mpsData.validate();
    }

}
