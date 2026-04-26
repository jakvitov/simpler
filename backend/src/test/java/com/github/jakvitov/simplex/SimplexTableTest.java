package com.github.jakvitov.simplex;

import com.github.jakvitov.mps.MpsData;
import com.github.jakvitov.mps.MpsDataTransformedBounds;
import org.hipparchus.fraction.BigFraction;
import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

import java.util.List;

public class SimplexTableTest {

    @Test
    public void simplex_table_from_mps_string_input_succeeds() {
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
        MpsDataTransformedBounds mpsDataTransformedBounds = new MpsDataTransformedBounds(mpsData);

        SimplexTable simplexTable = SimplexTable.fromMpsData(mpsDataTransformedBounds);
        List<String> correctBaseVariables = List.of("S_1", "A_1", "A_2", "S_3", "S_4", "A_3");
        assert simplexTable.baseVariables.equals(correctBaseVariables);
        simplexTable.data.forEach(row -> {
            assert row.size() == 11;
        });
        assert simplexTable.rhs.equals(List.of(new BigFraction(5), new BigFraction(10), new BigFraction(7), new BigFraction(4), new BigFraction(1), new BigFraction(-1)));
        assert simplexTable.objectiveFunctionRow.size() == 11;
        assert simplexTable.objectiveValue.equals(BigFraction.ZERO);
    }

    @Test
    public void simplex_table_deep_copy_succeeds() {
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
        MpsDataTransformedBounds mpsDataTransformedBounds = new MpsDataTransformedBounds(mpsData);

        SimplexTable simplexTable = SimplexTable.fromMpsData(mpsDataTransformedBounds);

        assert simplexTable.data.getFirst().size() == 11;
        assert simplexTable.data.getFirst().get(0).equals(BigFraction.ONE);

        SimplexTable simplexTableCopy = new SimplexTable(simplexTable);

        simplexTable.data.getFirst().set(0, BigFraction.MINUS_ONE);
        simplexTable.data.getFirst().add(BigFraction.ZERO);

        assert simplexTableCopy.data.getFirst().size() == 11;
        assert simplexTableCopy.data.getFirst().get(0).equals(BigFraction.ONE);

        assert simplexTable.data.getFirst().size() == 12;
        assert simplexTable.data.getFirst().get(0).equals(BigFraction.MINUS_ONE);
    }

    @Test
    @Disabled
    public void simplex_table_print_succeeds() {
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
        MpsDataTransformedBounds mpsDataTransformedBounds = new MpsDataTransformedBounds(mpsData);

        SimplexTable simplexTable = SimplexTable.fromMpsData(mpsDataTransformedBounds);
        simplexTable.print();
    }

}
