package com.github.jakvitov.dto.solver.twophase;

import com.github.jakvitov.dto.SimplexTableDto;
import com.github.jakvitov.dto.solver.basic.BasicSimplexIterationDto;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;

import java.util.ArrayList;
import java.util.List;

@Data
@Serdeable
public class TwoPhaseSimplexPhaseOneSolutionDto {

    private SimplexTableDto initialSimplexTable;
    private TwoPhaseSimplexObjectiveRowNormalizationDto artificialVariablesNormalization;
    private List<BasicSimplexIterationDto> iterations = new ArrayList<>();
    private SimplexTableDto finalSimplexTable;

}
