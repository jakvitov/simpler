package com.github.jakvitov.service;

import io.micronaut.test.extensions.junit5.annotation.MicronautTest;
import jakarta.inject.Inject;
import org.hipparchus.fraction.BigFraction;
import org.junit.jupiter.api.Test;

import java.util.List;

@MicronautTest
public class MultiplicativeSimplexSolverServiceTest {

    @Inject
    private MultiplicativeSimplexSolverService multiplicativeSimplexSolverService;

    @Test
    public void create_elementary_matrix_succeeds() {
        List<List<BigFraction>> d = List.of(List.of(BigFraction.TWO), List.of(BigFraction.TWO));
        List<List<BigFraction>> elementaryMatrix = List.of(List.of(BigFraction.ONE, BigFraction.ZERO), List.of(BigFraction.ZERO, BigFraction.ONE));

        List<List<BigFraction>> correctResult = List.of(List.of(BigFraction.TWO, BigFraction.ZERO), List.of(BigFraction.TWO, BigFraction.ONE));

        assert multiplicativeSimplexSolverService.createElementaryMatrix(d, 0).equals(correctResult);
    }

}
