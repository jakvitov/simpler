package com.github.jakvitov.controller;

import com.github.jakvitov.dto.HealthCheckResponseDto;
import com.github.jakvitov.dto.error.LastErrorResponseDto;
import com.github.jakvitov.dto.solver.SolveLpRequestDto;
import com.github.jakvitov.service.ErrorManagementService;
import com.github.jakvitov.simplex.OptimisationTarget;
import com.github.jakvitov.simplex.SimplexVariant;
import io.micronaut.http.HttpResponse;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Get;
import io.micronaut.http.annotation.Post;
import jakarta.inject.Inject;

import java.util.Optional;

@Controller("/be/simpler/error/last")
public class LastErrorController {

    @Inject
    private ErrorManagementService errorManagementService;

    @Get
    public HttpResponse<LastErrorResponseDto> getHealthCheckResponseDto() {
        Optional<LastErrorResponseDto> lastErrorResponseDto = errorManagementService.getLastErrorResponse();
        if (lastErrorResponseDto.isPresent()) {
            return HttpResponse.ok(lastErrorResponseDto.get());
        } else {
            return HttpResponse.notFound();
        }
    }

    @Post("/generate")
    public HttpResponse<?> generateTestLastError() {
        SolveLpRequestDto solveLpRequestDto = new SolveLpRequestDto("Error generation test data.", OptimisationTarget.MAX, SimplexVariant.BASIC_SIMPLEX, null);
        Throwable a = new RuntimeException("Test error exception");
        errorManagementService.registerLastExceptionAndLog(a, solveLpRequestDto);
        return HttpResponse.ok();
    }

}
