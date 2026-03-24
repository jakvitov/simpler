package com.github.jakvitov.service;

import com.github.jakvitov.dto.SimplexTableDto;
import com.github.jakvitov.dto.solver.SolutionStatus;
import com.github.jakvitov.dto.solver.SolveLpRequestDto;
import com.github.jakvitov.dto.solver.basic.BasicSimplexIterationDto;
import com.github.jakvitov.dto.solver.basic.SimplexTableLeavingEnteringVariableDto;
import com.github.jakvitov.dto.solver.basic.SimplexTableLeavingRowNormalisationDto;
import com.github.jakvitov.dto.solver.basic.SimplexTableRowsNormalizationDto;
import com.github.jakvitov.dto.solver.twophase.SolveLpTwoPhaseSimplexResponseDto;
import com.github.jakvitov.dto.solver.twophase.TwoPhaseSimplexObjectiveRowNormalizationDto;
import com.github.jakvitov.dto.solver.twophase.TwoPhaseSimplexPhaseSolutionDto;
import com.github.jakvitov.math.IntWrapper;
import com.github.jakvitov.mps.MpsData;
import com.github.jakvitov.mps.MpsDataTransformedBounds;
import com.github.jakvitov.simplex.OptimisationTarget;
import com.github.jakvitov.simplex.SimplexTable;
import io.micronaut.context.annotation.Value;
import jakarta.inject.Inject;
import jakarta.inject.Singleton;
import org.hipparchus.fraction.BigFraction;

import java.util.*;
import java.util.stream.IntStream;

@Singleton
public class TwoPhaseSimplexSolverService {

    @Value("${simpler.simplex.two-phase.max.iterations}")
    private Integer maxIterations;

    @Value("${simpler.simplex.two-phase.max.base.cycles}")
    private Integer maxCycles;

    @Inject
    private BasicSimplexSolverService basicSimplexSolverService;

    public SolveLpTwoPhaseSimplexResponseDto handleSolveTwoPhaseSimplexRequest(SolveLpRequestDto solveLpRequestDto) {
        MpsData mpsData = MpsData.parse(solveLpRequestDto.data());
        MpsDataTransformedBounds mpsDataTransformedBounds = new MpsDataTransformedBounds(mpsData);
        SimplexTable simplexTable = SimplexTable.fromMpsData(mpsDataTransformedBounds);

        return solveTwoPhaseSimplex(simplexTable, solveLpRequestDto.optimisationTarget());
    }

    private SolveLpTwoPhaseSimplexResponseDto solveTwoPhaseSimplex(SimplexTable simplexTable, OptimisationTarget optimisationTarget) {
        List<BigFraction> originalObjectiveRow = new ArrayList<>(simplexTable.objectiveFunctionRow);

        SolveLpTwoPhaseSimplexResponseDto responseDto = new SolveLpTwoPhaseSimplexResponseDto();

        responseDto.setInitialSimplexTable(new SimplexTableDto(simplexTable));

        IntWrapper iterations = IntWrapper.of(0);
        Map<Integer, Integer> visitedBaseCount = new HashMap<>();

        boolean continueToPhaseTwo = solveTwoPhaseSimplexPhaseOne(simplexTable, optimisationTarget, responseDto, iterations, visitedBaseCount);
        if (continueToPhaseTwo) {
            solveTwoPhaseSimplexPhaseTwo(simplexTable, optimisationTarget, responseDto, iterations, visitedBaseCount, originalObjectiveRow);
        }
        return responseDto;
    }

    /**
     * Perform phase one - simplex iterations until all artificial variables are eliminated from the basis
     * @param simplexTable
     * @param optimisationTarget
     * @param result
     * @param iteration
     * @param visitedBaseCount
     * @return true if it shall be proceeded to phase 2
     */
    private boolean solveTwoPhaseSimplexPhaseOne(SimplexTable simplexTable, OptimisationTarget optimisationTarget, SolveLpTwoPhaseSimplexResponseDto result, IntWrapper iteration, Map<Integer, Integer> visitedBaseCount) {
        visitedBaseCount.put(simplexTable.baseVariables.hashCode(), 1);
        //Make objective row artificial variables 1/1 and others 0
        setupObjectiveRowBeforePhaseOne(simplexTable);

        TwoPhaseSimplexPhaseSolutionDto simplexPhaseOneSolutionDto = new TwoPhaseSimplexPhaseSolutionDto();
        simplexPhaseOneSolutionDto.setInitialSimplexTable(new SimplexTableDto(simplexTable));

        //Add artificial variable values to the objective row adjusting them to the actual base
        TwoPhaseSimplexObjectiveRowNormalizationDto artificialVariablesNormalization = normalizeArtificialVariables(simplexTable);
        result.setArtificialVariablesNormalization(artificialVariablesNormalization);

        for (; ((iteration.value-1) < maxIterations) && (!basicSimplexSolverService.isSimplexTableSolved(simplexTable)); iteration.value ++) {

            if (visitedBaseCount.get(simplexTable.baseVariables.hashCode()) > maxCycles) {
                result.setSolutionStatus(SolutionStatus.CYCLE);
                simplexPhaseOneSolutionDto.setFinalSimplexTable(new SimplexTableDto(simplexTable));
                result.setPhaseOneSolutionDto(simplexPhaseOneSolutionDto);
                //Cycled solution - we shall not continue
                return false;
            }

            BasicSimplexIterationDto basicSimplexIterationDto = new BasicSimplexIterationDto();

            int enteringVariableIndex = basicSimplexSolverService.getEnteringVariableIndex(simplexTable);

            if (basicSimplexSolverService.isUnbounded(simplexTable, enteringVariableIndex)) {
                result.setSolutionStatus(SolutionStatus.UNBOUNDED);
                simplexPhaseOneSolutionDto.setFinalSimplexTable(new SimplexTableDto(simplexTable));
                SimplexTableLeavingEnteringVariableDto simplexTableLeavingEnteringVariableDto = new SimplexTableLeavingEnteringVariableDto();
                simplexTableLeavingEnteringVariableDto.setSimplexTableDto(new SimplexTableDto(simplexTable));
                simplexTableLeavingEnteringVariableDto.setLeavingVariableIndex(null);
                simplexTableLeavingEnteringVariableDto.setEnteringVariableIndex(enteringVariableIndex);
                basicSimplexIterationDto.setSimplexTableLeavingEnteringVariableDto(simplexTableLeavingEnteringVariableDto);
                BasicSimplexIterationDto simplexIterationDto = new BasicSimplexIterationDto();
                simplexIterationDto.setSimplexTableLeavingEnteringVariableDto(simplexTableLeavingEnteringVariableDto);
                simplexPhaseOneSolutionDto.getIterations().add(simplexIterationDto);
                result.setPhaseOneSolutionDto(simplexPhaseOneSolutionDto);
                //Unbounded - we should not continue
                return false;
            }

            List<Optional<BigFraction>> tVector = basicSimplexSolverService.computeTVector(enteringVariableIndex, simplexTable);

            int leavingVariableIndex = getLeavingVariableIndexForPhaseOne(tVector);

            SimplexTableLeavingEnteringVariableDto simplexTableLeavingEnteringVariableDto = new SimplexTableLeavingEnteringVariableDto();
            simplexTableLeavingEnteringVariableDto.setSimplexTableDto(new SimplexTableDto(simplexTable));
            simplexTableLeavingEnteringVariableDto.setTVector(tVector.stream().map(i -> i.orElse(BigFraction.ZERO)).toList());
            simplexTableLeavingEnteringVariableDto.setLeavingVariableIndex(leavingVariableIndex);
            simplexTableLeavingEnteringVariableDto.setEnteringVariableIndex(enteringVariableIndex);

            basicSimplexIterationDto.setSimplexTableLeavingEnteringVariableDto(simplexTableLeavingEnteringVariableDto);

            BigFraction normalizationCoefficient = basicSimplexSolverService.normaliseLeavingVariableRow(leavingVariableIndex, enteringVariableIndex, simplexTable);

            SimplexTableLeavingRowNormalisationDto simplexTableLeavingRowNormalisationDto = new SimplexTableLeavingRowNormalisationDto();
            simplexTableLeavingRowNormalisationDto.setRowNormalizationIndex(leavingVariableIndex);
            simplexTableLeavingRowNormalisationDto.setSimplexTableDto(new SimplexTableDto(simplexTable));
            simplexTableLeavingRowNormalisationDto.setBy(normalizationCoefficient);

            basicSimplexIterationDto.setSimplexTableLeavingRowNormalisationDto(simplexTableLeavingRowNormalisationDto);
            SimplexTableRowsNormalizationDto simplexTableRowsNormalizationDto = basicSimplexSolverService.normaliseRowsByLeavingVariableRow(leavingVariableIndex, enteringVariableIndex, simplexTable);

            basicSimplexIterationDto.setSimplexTableRowsNormalizationDto(simplexTableRowsNormalizationDto);

            basicSimplexSolverService.switchLeavingEnteringVariables(leavingVariableIndex, enteringVariableIndex, simplexTable);

            basicSimplexIterationDto.setSimplexTableAfterVariableSwitch(new SimplexTableDto(simplexTable));

            if (visitedBaseCount.containsKey(simplexTable.baseVariables.hashCode())) {
                visitedBaseCount.put(simplexTable.baseVariables.hashCode(), visitedBaseCount.get(simplexTable.baseVariables.hashCode()) + 1);
            } else {
                visitedBaseCount.put(simplexTable.baseVariables.hashCode(), 1);
            }
            simplexPhaseOneSolutionDto.getIterations().add(basicSimplexIterationDto);
        }

        //Not solved after loop means max iterations were achieved
        if (!basicSimplexSolverService.isSimplexTableSolved(simplexTable)) {
            simplexPhaseOneSolutionDto.setFinalSimplexTable(new SimplexTableDto(simplexTable));
            result.setSolutionStatus(SolutionStatus.MAX_ITERATIONS);
            result.setPhaseOneSolutionDto(simplexPhaseOneSolutionDto);
            return false;
        }

        simplexPhaseOneSolutionDto.setFinalSimplexTable(new SimplexTableDto(simplexTable));
        result.setPhaseOneSolutionDto(simplexPhaseOneSolutionDto);
        return true;
    }

    private void solveTwoPhaseSimplexPhaseTwo(SimplexTable simplexTable, OptimisationTarget optimisationTarget, SolveLpTwoPhaseSimplexResponseDto result,  IntWrapper iteration, Map<Integer, Integer> visitedBaseCount, List<BigFraction> originalObjectiveRow) {
        //Remove artificial variables from phase I
        removeArtificialVariablesAfterPhaseOne(simplexTable, originalObjectiveRow);
        TwoPhaseSimplexPhaseSolutionDto simplexPhaseTwoSolutionDto = new TwoPhaseSimplexPhaseSolutionDto();
        simplexPhaseTwoSolutionDto.setInitialSimplexTable(new SimplexTableDto(simplexTable));

        simplexTable.objectiveFunctionRow = originalObjectiveRow;
        simplexTable.objectiveValue = BigFraction.ZERO;
        //Restore original objective row (cropped to fit new simplex table)
        simplexPhaseTwoSolutionDto.setSimplexTableWithRestoredObjectiveRow(new SimplexTableDto(simplexTable));

        TwoPhaseSimplexObjectiveRowNormalizationDto objectiveRowToBaseVariablesAdjustment = adjustObjectiveRowToCurrentBasisPhaseTwo(simplexTable);
        //Adjust objective row to contain 0 in base variable values
        simplexPhaseTwoSolutionDto.setObjectiveRowToBaseVariablesAdjustment(objectiveRowToBaseVariablesAdjustment);

        for (; ((iteration.value-1) < maxIterations) && (!basicSimplexSolverService.isSimplexTableSolved(simplexTable)); iteration.value ++) {

            if (visitedBaseCount.get(simplexTable.baseVariables.hashCode()) > maxCycles) {
                result.setSolutionStatus(SolutionStatus.CYCLE);
                simplexPhaseTwoSolutionDto.setFinalSimplexTable(new SimplexTableDto(simplexTable));
                result.setPhaseTwoSolutionDto(simplexPhaseTwoSolutionDto);
                return;
            }

            BasicSimplexIterationDto basicSimplexIterationDto = new BasicSimplexIterationDto();

            int enteringVariableIndex = basicSimplexSolverService.getEnteringVariableIndex(simplexTable);

            if (basicSimplexSolverService.isUnbounded(simplexTable, enteringVariableIndex)) {
                result.setSolutionStatus(SolutionStatus.UNBOUNDED);
                simplexPhaseTwoSolutionDto.setFinalSimplexTable(new SimplexTableDto(simplexTable));
                //Last iteration of unbounded solution contains only t-vec with LE 0 results
                SimplexTableLeavingEnteringVariableDto simplexTableLeavingEnteringVariableDto = new SimplexTableLeavingEnteringVariableDto();
                simplexTableLeavingEnteringVariableDto.setSimplexTableDto(new SimplexTableDto(simplexTable));
                simplexTableLeavingEnteringVariableDto.setLeavingVariableIndex(null);
                simplexTableLeavingEnteringVariableDto.setEnteringVariableIndex(enteringVariableIndex);
                basicSimplexIterationDto.setSimplexTableLeavingEnteringVariableDto(simplexTableLeavingEnteringVariableDto);
                BasicSimplexIterationDto simplexIterationDto = new BasicSimplexIterationDto();
                simplexIterationDto.setSimplexTableLeavingEnteringVariableDto(simplexTableLeavingEnteringVariableDto);
                simplexPhaseTwoSolutionDto.getIterations().add(simplexIterationDto);
                result.setPhaseTwoSolutionDto(simplexPhaseTwoSolutionDto);
                return;
            }

            List<Optional<BigFraction>> tVector = basicSimplexSolverService.computeTVector(enteringVariableIndex, simplexTable);
            int leavingVariableIndex = basicSimplexSolverService.getLeavingVariableIndex(tVector);

            SimplexTableLeavingEnteringVariableDto simplexTableLeavingEnteringVariableDto = new SimplexTableLeavingEnteringVariableDto();
            simplexTableLeavingEnteringVariableDto.setSimplexTableDto(new SimplexTableDto(simplexTable));
            simplexTableLeavingEnteringVariableDto.setTVector(tVector.stream().map(i -> i.orElse(BigFraction.ZERO)).toList());
            simplexTableLeavingEnteringVariableDto.setLeavingVariableIndex(leavingVariableIndex);
            simplexTableLeavingEnteringVariableDto.setEnteringVariableIndex(enteringVariableIndex);

            basicSimplexIterationDto.setSimplexTableLeavingEnteringVariableDto(simplexTableLeavingEnteringVariableDto);

            BigFraction normalizationCoefficient = basicSimplexSolverService.normaliseLeavingVariableRow(leavingVariableIndex, enteringVariableIndex, simplexTable);

            SimplexTableLeavingRowNormalisationDto simplexTableLeavingRowNormalisationDto = new SimplexTableLeavingRowNormalisationDto();
            simplexTableLeavingRowNormalisationDto.setRowNormalizationIndex(leavingVariableIndex);
            simplexTableLeavingRowNormalisationDto.setSimplexTableDto(new SimplexTableDto(simplexTable));
            simplexTableLeavingRowNormalisationDto.setBy(normalizationCoefficient);

            basicSimplexIterationDto.setSimplexTableLeavingRowNormalisationDto(simplexTableLeavingRowNormalisationDto);
            SimplexTableRowsNormalizationDto simplexTableRowsNormalizationDto = basicSimplexSolverService.normaliseRowsByLeavingVariableRow(leavingVariableIndex, enteringVariableIndex, simplexTable);

            basicSimplexIterationDto.setSimplexTableRowsNormalizationDto(simplexTableRowsNormalizationDto);

            basicSimplexSolverService.switchLeavingEnteringVariables(leavingVariableIndex, enteringVariableIndex, simplexTable);

            basicSimplexIterationDto.setSimplexTableAfterVariableSwitch(new SimplexTableDto(simplexTable));

            if (visitedBaseCount.containsKey(simplexTable.baseVariables.hashCode())) {
                visitedBaseCount.put(simplexTable.baseVariables.hashCode(), visitedBaseCount.get(simplexTable.baseVariables.hashCode()) + 1);
            } else {
                visitedBaseCount.put(simplexTable.baseVariables.hashCode(), 1);
            }
            simplexPhaseTwoSolutionDto.getIterations().add(basicSimplexIterationDto);
        }

        //Not solved after loop means max iterations were achieved
        if (!basicSimplexSolverService.isSimplexTableSolved(simplexTable)) {
            simplexPhaseTwoSolutionDto.setFinalSimplexTable(new SimplexTableDto(simplexTable));
            result.setSolutionStatus(SolutionStatus.MAX_ITERATIONS);
            result.setPhaseTwoSolutionDto(simplexPhaseTwoSolutionDto);
            return;
        }

        result.setSolutionStatus(SolutionStatus.SOLVED);
        result.setResultVariableValues(basicSimplexSolverService.getSolutionVariableValues(simplexTable));
        simplexPhaseTwoSolutionDto.setFinalSimplexTable(new SimplexTableDto(simplexTable));
        result.setPhaseTwoSolutionDto(simplexPhaseTwoSolutionDto);
    }

    /**
     * After removal of artificial variables in phase II and restoration of original objective row,
     * adjust objective row to basis by eliminating basic variables to 0
     * @param simplexTable
     * @return
     */
    private TwoPhaseSimplexObjectiveRowNormalizationDto adjustObjectiveRowToCurrentBasisPhaseTwo(SimplexTable simplexTable) {
        Map<Integer, BigFraction> coefficients = new HashMap<>();

        //Create map index in base variables -> Index in objective row
        Map<Integer, Integer> baseVariablesObjectiveRowIndexes = new LinkedHashMap<>();
            IntStream.range(0, simplexTable.baseVariables.size()).forEach(baseVariableIndex -> {
            int baseVariableIndexInObjectiveRow = simplexTable.variables.indexOf(simplexTable.baseVariables.get(baseVariableIndex));
            if (baseVariableIndexInObjectiveRow == -1) {
                throw new IllegalStateException("Base variable " + simplexTable.baseVariables.get(baseVariableIndex) + " not found among variables! Variables: " + simplexTable.variables.toString());
            }
            baseVariablesObjectiveRowIndexes.put(baseVariableIndex, baseVariableIndexInObjectiveRow);
        });

        //Go over base variables in objective row and make them zero if necessary
        for (Map.Entry<Integer, Integer> baseVariableObjectiveRowIndex: baseVariablesObjectiveRowIndexes.entrySet()) {
            Integer baseIndex = baseVariableObjectiveRowIndex.getKey();
            Integer objectiveRowIndex = baseVariableObjectiveRowIndex.getValue();

            //Base variable does not have 0 in objective row. It needs to be fixed
            if (!simplexTable.objectiveFunctionRow.get(objectiveRowIndex).equals(BigFraction.ZERO)) {
                //Safe divide, since variable is in base, its value in that row must be 1
                BigFraction coefficient = simplexTable.objectiveFunctionRow.get(objectiveRowIndex).divide(simplexTable.data.get(baseIndex).get(objectiveRowIndex));

                //Add coefficient * base variable row to objective row
                for (int i = 0; i < simplexTable.variables.size(); i ++) {
                    simplexTable.objectiveFunctionRow.set(i, simplexTable.objectiveFunctionRow.get(i).add(simplexTable.data.get(baseIndex).get(i)).multiply(coefficient));
                }
                simplexTable.objectiveValue = simplexTable.objectiveValue.add(simplexTable.rhs.get(baseIndex).multiply(coefficient));
                coefficients.put(baseIndex, coefficient);
            }
        }
        TwoPhaseSimplexObjectiveRowNormalizationDto objectiveRowNormalizationDto = new TwoPhaseSimplexObjectiveRowNormalizationDto();
        objectiveRowNormalizationDto.setCoefficients(coefficients);
        objectiveRowNormalizationDto.setSimplexTableDto(new SimplexTableDto(simplexTable));
        return objectiveRowNormalizationDto;
    }

    /**
     * Get index of the leaving variable in base for Phase I simplex. Instead of basic simplex or phase two,
     * we do allow ratios to be negative
     * @param tVec
     * @return
     */
    private int getLeavingVariableIndexForPhaseOne(List<Optional<BigFraction>> tVec) {
        Optional<Integer> minimumIndex = Optional.empty();
        for (int i = 0; i < tVec.size(); i ++) {
            if (tVec.get(i).isEmpty()) {
                continue;
            }
            if (minimumIndex.isEmpty()) {
                minimumIndex = Optional.of(i);
            } else if (tVec.get(i).get().compareTo(tVec.get(minimumIndex.get()).get() /* safe since this index must have been set after check*/) < 0) {
                minimumIndex = Optional.of(i);
            }
        }

        if (minimumIndex.isEmpty()) {
            throw new IllegalStateException("No valid entry in T-vector found.");
        }
        return minimumIndex.get();
    }

    /**
     * Removes all artificial variables from given simplex table and crops original objective row accordingly
     * @param simplexTable
     */
    private void removeArtificialVariablesAfterPhaseOne(SimplexTable simplexTable, List<BigFraction> originalObjectiveRow) {
        OptionalInt artificialVariableBeginningIndex = IntStream.range(0, simplexTable.variables.size())
                .filter(i -> simplexTable.variables.get(i).startsWith("A_")).findFirst();
        if (artificialVariableBeginningIndex.isEmpty()) {
            return;
        }

        simplexTable.variables = simplexTable.variables.subList(0, artificialVariableBeginningIndex.getAsInt());

        for (int i = 0; i < simplexTable.data.size(); i ++) {
            simplexTable.data.set(i, simplexTable.data.get(i).subList(0, artificialVariableBeginningIndex.getAsInt()));
        }
        simplexTable.objectiveFunctionRow = simplexTable.objectiveFunctionRow.subList(0, artificialVariableBeginningIndex.getAsInt());
        originalObjectiveRow.subList(artificialVariableBeginningIndex.getAsInt(), originalObjectiveRow.size()).clear();
    }

    /**
     * Setup objective row before phase I, making artificial variables 1/1 and all other 0
     * @param simplexTable
     */
    private void setupObjectiveRowBeforePhaseOne(SimplexTable simplexTable) {
        IntStream.range(0, simplexTable.variables.size()).forEach(i -> {
            if (simplexTable.variables.get(i).startsWith("A_")) {
                simplexTable.objectiveFunctionRow.set(i, BigFraction.ONE);
            } else {
                simplexTable.objectiveFunctionRow.set(i, BigFraction.ZERO);
            }
        });
    }

    /**
     * Make objective row artificial variables zero by adding their rows to the objective row
     * Step one in two phase simplex
     * @param simplexTable
     * @return ArtificialVariablesObjectiveRowNormalizationDto
     */
    private TwoPhaseSimplexObjectiveRowNormalizationDto normalizeArtificialVariables(SimplexTable simplexTable) {
        List<Integer> artificialVariableRowIndexes = IntStream.range(0, simplexTable.baseVariables.size()).mapToObj((baseVariableIndex) -> {
            if (simplexTable.baseVariables.get(baseVariableIndex).startsWith("A_")) {
                return baseVariableIndex;
            }
            return null;
        }).filter(Objects::nonNull).toList();

        Map<Integer, BigFraction> coefficients = new HashMap<>(artificialVariableRowIndexes.size());

        for (int artificialVariableRowIndex : artificialVariableRowIndexes) {
            BigFraction coefficient = BigFraction.ONE.negate();
            //Add the row to the objective function row
            for (int i = 0; i < simplexTable.objectiveFunctionRow.size(); i++) {
                BigFraction newObjectiveRowValue = simplexTable.objectiveFunctionRow
                        .get(i).add(simplexTable.data.get(artificialVariableRowIndex).get(i).multiply(coefficient));
                simplexTable.objectiveFunctionRow.set(i, newObjectiveRowValue);
            }
            //Update the objective value
            simplexTable.objectiveValue = simplexTable.objectiveValue.add(simplexTable.rhs.get(artificialVariableRowIndex).multiply(coefficient));
            coefficients.put(artificialVariableRowIndex, coefficient);
        }

        TwoPhaseSimplexObjectiveRowNormalizationDto artificialVariablesNormalization = new TwoPhaseSimplexObjectiveRowNormalizationDto();
        artificialVariablesNormalization.setCoefficients(coefficients);
        artificialVariablesNormalization.setSimplexTableDto(new SimplexTableDto(simplexTable));
        return artificialVariablesNormalization;
    }


}
