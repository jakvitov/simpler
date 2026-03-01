package com.github.jakvitov.controller;

import com.github.jakvitov.dto.solver.SolveLpBasicSimplexResponseDto;
import com.github.jakvitov.dto.verification.MpsVerificationResponseDto;
import com.github.jakvitov.dto.solver.SolveLpRequestDto;
import com.github.jakvitov.service.BasicSimplexSolverService;
import io.micronaut.http.HttpResponse;
import io.micronaut.http.annotation.Body;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Post;
import jakarta.inject.Inject;

@Controller("/solve-lp/basic")
public class SolveBasicSimplexController {

    @Inject
    private final BasicSimplexSolverService basicSimplexSolverService;

    @Post
    public HttpResponse<SolveLpBasicSimplexResponseDto> solveBasicSimplex(@Body SolveLpRequestDto solveLpRequestDto) {

    }


}
