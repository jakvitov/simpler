package com.github.jakvitov.math;

import io.micronaut.test.extensions.junit5.annotation.MicronautTest;
import jakarta.inject.Inject;
import org.hipparchus.fraction.BigFraction;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

@MicronautTest
public class LinearAlgebraServiceTest {

    @Inject
    private LinearAlgebraService linearAlgebraService;

    @Test
    public void matrix_inverse_succeeds_for_invertible_matrix() {

        List<List<BigFraction>> testData = new ArrayList<>(3);
        List<BigFraction> testRow1 = List.of(BigFraction.ONE, BigFraction.TWO, new BigFraction(3));
        List<BigFraction> testRow2 = List.of(new BigFraction(4), new BigFraction(5), new BigFraction(6));
        List<BigFraction> testRow3 = List.of(new BigFraction(7), new BigFraction(11), new BigFraction(9));

        List<BigFraction> originalRow1 = new ArrayList<>(testRow1);
        List<BigFraction> originalRow2 = new ArrayList<>(testRow2);
        List<BigFraction> originalRow3 = new ArrayList<>(testRow3);

        testData.add(testRow1);
        testData.add(testRow2);
        testData.add(testRow3);

        Optional<List<List<BigFraction>>> inversionResult = linearAlgebraService.getMatrixInversion(testData);

        assert inversionResult.isPresent();

        List<List<BigFraction>> invertedMatrix = inversionResult.get();

        assert invertedMatrix.size() == 3;
        assert invertedMatrix.getFirst().size() == 3;

        assert invertedMatrix.getFirst().get(0).equals(new BigFraction(-7, 6));
        assert invertedMatrix.getFirst().get(1).equals(new BigFraction(5, 6));
        assert invertedMatrix.getFirst().get(2).equals(new BigFraction(-1, 6));


        assert invertedMatrix.get(1).get(0).equals(new BigFraction(2, 6));
        assert invertedMatrix.get(1).get(1).equals(new BigFraction(-4, 6));
        assert invertedMatrix.get(1).get(2).equals(new BigFraction(2, 6));


        assert invertedMatrix.get(2).get(0).equals(new BigFraction(3, 6));
        assert invertedMatrix.get(2).get(1).equals(new BigFraction(1, 6));
        assert invertedMatrix.get(2).get(2).equals(new BigFraction(-1, 6));

        assert testData.getFirst().equals(originalRow1);
        assert testData.get(1).equals(originalRow2);
        assert testData.get(2).equals(originalRow3);
    }

    @Test
    public void matrix_inverse_fails_for_singular_matrix() {
        List<List<BigFraction>> testData = new ArrayList<>(3);
        List<BigFraction> testRow1 = List.of(BigFraction.ONE, BigFraction.TWO, new BigFraction(3));
        List<BigFraction> testRow2 = List.of(new BigFraction(4), new BigFraction(5), new BigFraction(6));
        List<BigFraction> testRow3 = List.of(new BigFraction(7), new BigFraction(8), new BigFraction(9));

        testData.add(testRow1);
        testData.add(testRow2);
        testData.add(testRow3);

        Optional<List<List<BigFraction>>> inversionResult = linearAlgebraService.getMatrixInversion(testData);

        assert inversionResult.isEmpty();
    }

    @Test
    public void matrix_multiplication_succeeds_for_compatible_matrices() {

        List<List<BigFraction>> lhs = new ArrayList<>(3);
        List<BigFraction> testRow1 = List.of(BigFraction.ONE, BigFraction.TWO, new BigFraction(3));
        List<BigFraction> testRow2 = List.of(new BigFraction(4), new BigFraction(5), new BigFraction(6));
        List<BigFraction> testRow3 = List.of(new BigFraction(7), new BigFraction(8), new BigFraction(9));

        List<BigFraction> originalRow1 = new ArrayList<>(testRow1);
        List<BigFraction> originalRow2 = new ArrayList<>(testRow2);
        List<BigFraction> originalRow3 = new ArrayList<>(testRow3);

        lhs.add(testRow1);
        lhs.add(testRow2);
        lhs.add(testRow3);

        List<List<BigFraction>> rhs = List.of(List.of(BigFraction.ONE), List.of(BigFraction.TWO), List.of(new BigFraction(3)));

        Optional<List<List<BigFraction>>> result = linearAlgebraService.multiplyMatrices(lhs, rhs);
        assert result.isPresent();

        assert result.get().size() == 3;

        assert result.get().getFirst().getFirst().equals(new BigFraction(14));
        assert result.get().get(1).getFirst().equals(new BigFraction(32));
        assert result.get().get(2).getFirst().equals(new BigFraction(50));

        assert lhs.getFirst().equals(originalRow1);
        assert lhs.get(1).equals(originalRow2);
        assert lhs.get(2).equals(originalRow3);

        assert rhs.get(0).get(0).equals(BigFraction.ONE);
        assert rhs.get(1).get(0).equals(BigFraction.TWO);
        assert rhs.get(2).get(0).equals(new BigFraction(3));
    }

    @Test
    public void matrix_multiplication_fails_for_incompatible_matrices() {
        List<List<BigFraction>> lhs = new ArrayList<>(3);
        List<BigFraction> testRow1 = List.of(BigFraction.ONE, BigFraction.TWO, new BigFraction(3));
        List<BigFraction> testRow2 = List.of(new BigFraction(4), new BigFraction(5), new BigFraction(6));
        List<BigFraction> testRow3 = List.of(new BigFraction(7), new BigFraction(8), new BigFraction(9));

        List<List<BigFraction>> rhs = List.of(List.of(BigFraction.ONE), List.of(BigFraction.TWO));
        Optional<List<List<BigFraction>>> result = linearAlgebraService.multiplyMatrices(lhs, rhs);

        assert result.isEmpty();
    }

}
