package com.github.jakvitov.service;

import com.github.jakvitov.dto.error.LastErrorResponseDto;
import io.micronaut.context.annotation.Value;
import io.micronaut.serde.jackson.JacksonJsonMapper;
import io.micronaut.serde.jackson.JacksonObjectMapper;
import jakarta.inject.Inject;
import jakarta.inject.Singleton;
import lombok.extern.slf4j.Slf4j;

import java.io.IOException;
import java.io.PrintWriter;
import java.io.StringWriter;
import java.time.OffsetDateTime;
import java.util.Optional;

/**
 * Service responsible for managing error data
 */
@Singleton
@Slf4j
public class ErrorManagementService {

    private Throwable lastException;
    private String lastExceptionRequest;
    private OffsetDateTime lastExceptionTime;

    @Inject
    private JacksonObjectMapper jacksonObjectMapper;

    @Value("${simpler.backend.version:UNKNOWN}")
    private String backendVersion;

    public void registerLastExceptionAndLog(Throwable exception, Object request) {
        this.lastException = exception;
        try {
            this.lastExceptionRequest = jacksonObjectMapper.writeValueAsString(request);
        } catch (IOException e) {
            this.lastExceptionRequest = lastExceptionRequest.toString();
        }
        this.lastExceptionTime = OffsetDateTime.now();
        log.error("Exception occurred.", exception);
    }

    /**
     * Return Optional of last error. Return optional empty if none is set.
     * @return
     */
    public Optional<LastErrorResponseDto> getLastErrorResponse() {
        if (lastException == null) {
            return Optional.empty();
        }

        LastErrorResponseDto lastErrorResponseDto = new LastErrorResponseDto();
        lastErrorResponseDto.setExceptionMessage(lastException.getMessage());
        lastErrorResponseDto.setRequest(lastExceptionRequest);
        lastErrorResponseDto.setExceptionTime(lastExceptionTime);
        lastErrorResponseDto.setVersion(backendVersion);

        try {
            StringWriter sw = new StringWriter();
            lastException.printStackTrace(new PrintWriter(sw));
            String stackTrace = sw.toString();
            lastErrorResponseDto.setStackTrace(stackTrace);
        } catch (Exception e) {
            lastErrorResponseDto.setStackTrace(lastException.toString());
        }

        return Optional.of(lastErrorResponseDto);
    }

}
