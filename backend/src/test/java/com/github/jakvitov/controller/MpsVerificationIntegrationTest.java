package com.github.jakvitov.controller;

import com.github.jakvitov.dto.verification.MpsVerificationInputDto;
import com.github.jakvitov.dto.verification.MpsVerificationResponseDto;
import com.github.jakvitov.dto.verification.MpsVerificationStatus;
import io.micronaut.http.HttpResponse;
import io.micronaut.http.HttpStatus;
import io.micronaut.test.extensions.junit5.annotation.MicronautTest;
import jakarta.inject.Inject;
import org.junit.jupiter.api.Test;

@MicronautTest
public class MpsVerificationIntegrationTest {

    @Inject
    private MpsVerificationController mpsVerificationController;

    @Test
    public void verification_of_mps_succeeds() {
        final String testInput = """
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

        MpsVerificationInputDto mpsVerificationInputDto = new MpsVerificationInputDto(testInput);
        HttpResponse<MpsVerificationResponseDto> res = mpsVerificationController.verifyMps(mpsVerificationInputDto);
        assert res.status() == HttpStatus.OK;
        assert res.body().status().equals(MpsVerificationStatus.OK);
        assert res.body().parsedLpDefinition().warningMessage().isEmpty();
        assert res.body().parsedLpDefinition().lines().size() == 4;
        assert res.body().parsedLpDefinition().bounds().size() == 2;
    }

    @Test
    public void verification_of_mps_fails_for_invalid_mps() {
        final String testInput = """
                NAME          TESTPROB
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

        MpsVerificationInputDto mpsVerificationInputDto = new MpsVerificationInputDto(testInput);
        HttpResponse<MpsVerificationResponseDto> res = mpsVerificationController.verifyMps(mpsVerificationInputDto);
        assert res.status() == HttpStatus.OK;
        assert res.body().status().equals(MpsVerificationStatus.VERIFICATION_FAILED);
        assert res.body().parsedLpDefinition() == null;
        assert !res.body().errors().isEmpty();
    }

}
