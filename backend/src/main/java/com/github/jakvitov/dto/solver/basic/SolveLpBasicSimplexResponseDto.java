package com.github.jakvitov.dto.solver.basic;

import com.github.jakvitov.dto.SimplexTableDto;
import com.github.jakvitov.dto.solver.AbstractSolutionResponseDto;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;
import lombok.EqualsAndHashCode;

import java.util.ArrayList;
import java.util.List;

@EqualsAndHashCode(callSuper = true)
@Serdeable
@Data
public class SolveLpBasicSimplexResponseDto extends AbstractSolutionResponseDto {

    private List<BasicSimplexIterationDto> iterations = new ArrayList<>();

    private SimplexTableDto finalSimplexTable;

}
