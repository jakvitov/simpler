package com.github.jakvitov.dto.error;

import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;

import java.time.OffsetDateTime;

@Serdeable
@Data
public class LastErrorResponseDto {

    private OffsetDateTime exceptionTime;
    private String request;
    private String exceptionMessage;
    private String stackTrace;
    private String version;

}
