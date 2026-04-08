package com.github.jakvitov.dto.solver.revised;

import com.github.jakvitov.dto.SimplexTableDto;
import com.github.jakvitov.dto.solver.twophase.TwoPhaseSimplexObjectiveRowNormalizationDto;
import io.micronaut.core.annotation.Nullable;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;

import java.util.ArrayList;
import java.util.List;

@Data
@Serdeable
public class RevisedSimplexPhaseOneSolutionDto {

    private SimplexTableDto initialSimplexTable;
    private TwoPhaseSimplexObjectiveRowNormalizationDto artificialVariablesNormalization;
    private List<RevisedSimplexIterationDto> iterations = new ArrayList<>();
    @Nullable
    private List<String> resultBase;

}
