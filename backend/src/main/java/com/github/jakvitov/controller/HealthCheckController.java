package com.github.jakvitov.controller;

import com.github.jakvitov.dto.HealthCheckResponseDto;
import io.micronaut.context.annotation.Value;
import io.micronaut.http.HttpResponse;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Get;

@Controller("/health")
public class HealthCheckController {

    @Value("${simpler.backend.version:UNKNOWN}")
    private String backendVersion;

    private String status = "UP";

    @Get
    public HttpResponse<HealthCheckResponseDto> getHealthCheckResponseDto() {
        var response =  new HealthCheckResponseDto(status, backendVersion);
        return HttpResponse.ok(response);
    }
}
