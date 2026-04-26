package com.github.jakvitov.service;

import com.github.jakvitov.dto.error.LastErrorResponseDto;
import io.micronaut.test.extensions.junit5.annotation.MicronautTest;
import jakarta.inject.Inject;
import org.junit.jupiter.api.Test;

import java.util.Optional;

@MicronautTest
public class ErrorManagementServiceTest {

    @Inject
    private ErrorManagementService errorManagementService;

    @Test
    public void register_and_get_error_succeeds() {
        errorManagementService.registerLastExceptionAndLog(new RuntimeException("my exception"), "request");
        Optional<LastErrorResponseDto> lastErrorResponseDto = errorManagementService.getLastErrorResponse();

        assert lastErrorResponseDto.isPresent();
        assert lastErrorResponseDto.get().getExceptionMessage().equals("my exception");
        assert !lastErrorResponseDto.get().getStackTrace().isEmpty();
        assert lastErrorResponseDto.get().getRequest().equals("\"request\"");
        assert lastErrorResponseDto.get().getExceptionTime() != null;
    }


}
