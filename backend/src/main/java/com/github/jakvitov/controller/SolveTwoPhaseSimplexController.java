package com.github.jakvitov.controller;

import com.github.jakvitov.dto.solver.SolveLpErrorResponse;
import com.github.jakvitov.dto.solver.SolveLpRequestDto;
import com.github.jakvitov.mps.MpsParsingException;
import com.github.jakvitov.service.ErrorManagementService;
import com.github.jakvitov.service.TwoPhaseSimplexSolverService;
import com.github.jakvitov.simplex.SimplexTableTransformationError;
import io.micronaut.http.HttpResponse;
import io.micronaut.http.annotation.Body;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Post;
import jakarta.inject.Inject;
import lombok.extern.slf4j.Slf4j;

import java.util.ArrayList;
import java.util.List;

@Controller("/be/simpler/solve-lp/two-phase")
@Slf4j
public class SolveTwoPhaseSimplexController {

    @Inject
    private TwoPhaseSimplexSolverService twoPhaseSimplexSolverService;

    @Inject
    private ErrorManagementService errorManagementService;

    @Post
    public HttpResponse<?> solveTwoPhaseSimplex(@Body SolveLpRequestDto solveLpRequestDto) {
        try {
            return HttpResponse.ok(twoPhaseSimplexSolverService.handleSolveTwoPhaseSimplexRequest(solveLpRequestDto));
        }
        catch (MpsParsingException mpe) {
            SolveLpErrorResponse errorResponse = new SolveLpErrorResponse(List.of(mpe.reasons), false);
            return HttpResponse.badRequest(errorResponse);
        } catch (SimplexTableTransformationError stte) {
            SolveLpErrorResponse errorResponse = new SolveLpErrorResponse(List.of(stte.getMessage()), false);
            return HttpResponse.badRequest(errorResponse);
        }
        catch (Exception e) {
            errorManagementService.registerLastExceptionAndLog(e, solveLpRequestDto);
            List<String> errors = new ArrayList<>(1);
            errors.add(e.getMessage());
            SolveLpErrorResponse errorResponse = new SolveLpErrorResponse(errors, false);
            return HttpResponse.serverError(errorResponse);
        }
    }

}
