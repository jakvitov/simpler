package com.github.jakvitov.service;

import com.github.jakvitov.dto.SimplexTableDto;
import com.github.jakvitov.dto.solver.SolutionStatus;
import com.github.jakvitov.dto.solver.SolveLpRequestDto;
import com.github.jakvitov.dto.solver.basic.*;
import com.github.jakvitov.dto.solver.config.SolverConfigurationDto;
import com.github.jakvitov.mps.MpsData;
import com.github.jakvitov.mps.MpsDataTransformedBounds;
import com.github.jakvitov.simplex.OptimisationTarget;
import com.github.jakvitov.simplex.SimplexTable;
import com.github.jakvitov.simplex.SimplexTableTransformationError;
import io.micronaut.core.annotation.Nullable;
import jakarta.inject.Inject;
import jakarta.inject.Singleton;
import org.hipparchus.fraction.BigFraction;

import java.util.*;
import java.util.stream.Collectors;

import static com.github.jakvitov.service.SolverConfigurationService.SolverConfigurationConstants.BS_MAX_CYCLE;
import static com.github.jakvitov.service.SolverConfigurationService.SolverConfigurationConstants.BS_MAX_ITER;

@Singleton
public class BasicSimplexSolverService {

    @Inject
    private SolverConfigurationService configurationService;

    public SolveLpBasicSimplexResponseDto handleSolveBasicSimplexRequest(SolveLpRequestDto solveLpRequestDto) {
        MpsData mpsData = MpsData.parse(solveLpRequestDto.data());
        mpsData.validate();
        MpsDataTransformedBounds mpsDataTransformedBounds = new MpsDataTransformedBounds(mpsData);
        SimplexTable simplexTable = SimplexTable.fromMpsData(mpsDataTransformedBounds);

        if (simplexTable.containsArtificialVariables) {
            throw new SimplexTableTransformationError("Problem contains G/E rows, that are not supported in basic simplex algorithm.\nConsider using duality to transform the problem or try Two-phase simplex algorithm.");
        }

        return solveBasicSimplex(simplexTable, solveLpRequestDto.optimisationTarget(), solveLpRequestDto.solverConfiguration());
    }

    private SolveLpBasicSimplexResponseDto solveBasicSimplex(SimplexTable simplexTable, OptimisationTarget optimisationTarget, @Nullable SolverConfigurationDto solverConfigurationInput) {
        SolveLpBasicSimplexResponseDto result = new SolveLpBasicSimplexResponseDto();

        if (optimisationTarget.equals(OptimisationTarget.MIN)) {
            convertObjectiveRowForMinimalization(simplexTable);
        }
        result.setInitialSimplexTable(new SimplexTableDto(simplexTable));

        //Visited base hash -> number of visits
        HashMap<Integer, Integer> visitedBaseCount = new HashMap<>();
        visitedBaseCount.put(simplexTable.baseVariables.hashCode(), 1);

        for (int iteration = 1; ((iteration-1) < configurationService.getConfig(BS_MAX_ITER, solverConfigurationInput)) && (!isSimplexTableSolved(simplexTable)); iteration ++) {

            if (visitedBaseCount.get(simplexTable.baseVariables.hashCode()) > configurationService.getConfig(BS_MAX_CYCLE, solverConfigurationInput)) {
                result.setSolutionStatus(SolutionStatus.CYCLE);
                result.setFinalSimplexTable(new SimplexTableDto(simplexTable));
                return result;
            }

            BasicSimplexIterationDto basicSimplexIterationDto = new BasicSimplexIterationDto();

            int enteringVariableIndex = getEnteringVariableIndex(simplexTable);

            if (isUnbounded(simplexTable, enteringVariableIndex)) {
                result.setSolutionStatus(SolutionStatus.UNBOUNDED);
                result.setFinalSimplexTable(new SimplexTableDto(simplexTable));
                //Last iteration of unbounded solution contains only t-vec with LE 0 results
                SimplexTableLeavingEnteringVariableDto simplexTableLeavingEnteringVariableDto = new SimplexTableLeavingEnteringVariableDto();
                simplexTableLeavingEnteringVariableDto.setSimplexTableDto(new SimplexTableDto(simplexTable));
                simplexTableLeavingEnteringVariableDto.setLeavingVariableIndex(null);
                simplexTableLeavingEnteringVariableDto.setEnteringVariableIndex(enteringVariableIndex);
                basicSimplexIterationDto.setSimplexTableLeavingEnteringVariableDto(simplexTableLeavingEnteringVariableDto);
                BasicSimplexIterationDto simplexIterationDto = new BasicSimplexIterationDto();
                simplexIterationDto.setSimplexTableLeavingEnteringVariableDto(simplexTableLeavingEnteringVariableDto);
                result.getIterations().add(simplexIterationDto);
                return result;
            }

            List<Optional<BigFraction>> tVector = computeTVector(enteringVariableIndex, simplexTable);
            int leavingVariableIndex = getLeavingVariableIndex(tVector);

            SimplexTableLeavingEnteringVariableDto simplexTableLeavingEnteringVariableDto = new SimplexTableLeavingEnteringVariableDto();
            simplexTableLeavingEnteringVariableDto.setSimplexTableDto(new SimplexTableDto(simplexTable));
            simplexTableLeavingEnteringVariableDto.setTVector(tVector.stream().map(i -> i.orElse(BigFraction.ZERO)).toList());
            simplexTableLeavingEnteringVariableDto.setLeavingVariableIndex(leavingVariableIndex);
            simplexTableLeavingEnteringVariableDto.setEnteringVariableIndex(enteringVariableIndex);

            basicSimplexIterationDto.setSimplexTableLeavingEnteringVariableDto(simplexTableLeavingEnteringVariableDto);

            BigFraction normalizationCoefficient = normaliseLeavingVariableRow(leavingVariableIndex, enteringVariableIndex, simplexTable);

            SimplexTableLeavingRowNormalisationDto simplexTableLeavingRowNormalisationDto = new SimplexTableLeavingRowNormalisationDto();
            simplexTableLeavingRowNormalisationDto.setRowNormalizationIndex(leavingVariableIndex);
            simplexTableLeavingRowNormalisationDto.setSimplexTableDto(new SimplexTableDto(simplexTable));
            simplexTableLeavingRowNormalisationDto.setBy(normalizationCoefficient);

            basicSimplexIterationDto.setSimplexTableLeavingRowNormalisationDto(simplexTableLeavingRowNormalisationDto);
            SimplexTableRowsNormalizationDto simplexTableRowsNormalizationDto = normaliseRowsByLeavingVariableRow(leavingVariableIndex, enteringVariableIndex, simplexTable);

            basicSimplexIterationDto.setSimplexTableRowsNormalizationDto(simplexTableRowsNormalizationDto);

            switchLeavingEnteringVariables(leavingVariableIndex, enteringVariableIndex, simplexTable);

            basicSimplexIterationDto.setSimplexTableAfterVariableSwitch(new SimplexTableDto(simplexTable));

            if (visitedBaseCount.containsKey(simplexTable.baseVariables.hashCode())) {
                visitedBaseCount.put(simplexTable.baseVariables.hashCode(), visitedBaseCount.get(simplexTable.baseVariables.hashCode()) + 1);
            } else {
                visitedBaseCount.put(simplexTable.baseVariables.hashCode(), 1);
            }
            result.getIterations().add(basicSimplexIterationDto);
        }

        //Not solved after loop means max iterations were achieved
        if (!isSimplexTableSolved(simplexTable)) {
            result.setFinalSimplexTable(new SimplexTableDto(simplexTable));
            result.setSolutionStatus(SolutionStatus.MAX_ITERATIONS);
            return result;
        }

        result.setSolutionStatus(SolutionStatus.SOLVED);
        result.setResultVariableValues(getSolutionVariableValues(simplexTable));
        result.setFinalSimplexTable(new SimplexTableDto(simplexTable));

        if (optimisationTarget.equals(OptimisationTarget.MIN)) {
            result.setSolutionObjectiveFunctionValue(simplexTable.objectiveValue.negate());
        } else {
            result.setSolutionObjectiveFunctionValue(simplexTable.objectiveValue);
        }
        return result;
    }

    /**
     * Given Simplex table constructed for maximization, adjust it for minimalization problem by
     * negating the original variables in the objective row
     * @param simplexTable
     */
    protected void convertObjectiveRowForMinimalization(SimplexTable simplexTable) {
        for (int i = 0; i < simplexTable.variables.size(); i ++) {
            if (!(simplexTable.variables.get(i).startsWith("S_") || simplexTable.variables.get(i).startsWith("A_"))) {
                simplexTable.objectiveFunctionRow.set(i, simplexTable.objectiveFunctionRow.get(i).negate());
            }
        }
    }

    /**
     * Check if SimplexTable shows unbounded solution for the given iteration
     * @param simplexTable
     * @param enteringVariableIndex
     * @return
     */
    protected boolean isUnbounded(SimplexTable simplexTable, int enteringVariableIndex) {
        boolean unbounded = true;
        for (int i = 0; i < simplexTable.baseVariables.size(); i++) {
            if (simplexTable.data.get(i).get(enteringVariableIndex).signum() > 0) {
                unbounded = false;
                break;
            }
        }
        return unbounded;
    }

    /**
     * Given solved Simplex table, return values of all non-zero original variables
     * @param simplexTable
     * @return
     */
    protected Map<String, BigFraction> getSolutionVariableValues(SimplexTable simplexTable) {
        Map<String, BigFraction> result = new HashMap<>();
        for (int i = 0; i < simplexTable.baseVariables.size(); i ++) {
            String variableName = simplexTable.baseVariables.get(i);
            result.put(variableName, simplexTable.rhs.get(i));

        }
        return result;
    }

    protected void switchLeavingEnteringVariables(int leavingVariableRow, int enteringVariableRow, SimplexTable simplexTable) {
        String enteringVariableName = simplexTable.variables.get(enteringVariableRow);
        simplexTable.baseVariables.set(leavingVariableRow, enteringVariableName);
    }

    /**
     * Normalize all non-leaving variable rows using gauss full elimination method.
     * Returns DTO containing information about coefficients and result
     * @param leavingVariableRow
     * @param enteringVariableColumn
     * @param simplexTable
     * @return SimplexTableRowsNormalizationDto
     */
    protected SimplexTableRowsNormalizationDto normaliseRowsByLeavingVariableRow(int leavingVariableRow, int enteringVariableColumn, SimplexTable simplexTable) {
        SimplexTableRowsNormalizationDto simplexTableRowsNormalizationDto = new SimplexTableRowsNormalizationDto();
        simplexTableRowsNormalizationDto.setLeavingVariableIndex(leavingVariableRow);
        //Normalize standard table rows
        for (int rowIndex = 0; rowIndex < simplexTable.data.size(); rowIndex ++) {
            if (rowIndex == leavingVariableRow) {
                continue;
            }

            BigFraction coefficient = simplexTable.data.get(rowIndex).get(enteringVariableColumn).negate();

            for (int columnIndex = 0; columnIndex < simplexTable.data.get(rowIndex).size(); columnIndex ++) {
                BigFraction addtitionValue = simplexTable.data.get(leavingVariableRow).get(columnIndex).multiply(coefficient);
                simplexTable.data.get(rowIndex).set(columnIndex, simplexTable.data.get(rowIndex).get(columnIndex).add(addtitionValue));
            }

            simplexTable.rhs.set(rowIndex, simplexTable.rhs.get(rowIndex).add(simplexTable.rhs.get(leavingVariableRow).multiply(coefficient)));
            simplexTableRowsNormalizationDto.getCoefficients().put(rowIndex, coefficient);
        }

        //Normalize objective row
        BigFraction coefficient = simplexTable.objectiveFunctionRow.get(enteringVariableColumn).negate();
        for (int columnIndex = 0; columnIndex < simplexTable.objectiveFunctionRow.size(); columnIndex ++) {
            simplexTable.objectiveFunctionRow.set(columnIndex, simplexTable.objectiveFunctionRow.get(columnIndex).add(simplexTable.data.get(leavingVariableRow).get(columnIndex).multiply(coefficient)));
        }

        //Normalize targetValue
        simplexTable.objectiveValue = simplexTable.objectiveValue.add(simplexTable.rhs.get(leavingVariableRow).multiply(coefficient));

        simplexTableRowsNormalizationDto.setObjectiveRowCoefficient(coefficient);

        simplexTableRowsNormalizationDto.setSimplexTableDto(new SimplexTableDto(simplexTable));
        return simplexTableRowsNormalizationDto;
    }

    /**
     * Normalize row with leaving base variable making it effectively 1 in leaving x entering variable point.
     * Return coefficient of transformation
     * @param leavingVariableRow
     * @param simplexTable
     * @return
     */
    protected BigFraction normaliseLeavingVariableRow(int leavingVariableRow, int enteringVariableColumn, SimplexTable simplexTable) {
        //Shall never be zero, since we skip degenerate rows in t-vector
        BigFraction targetVariableValue = simplexTable.data.get(leavingVariableRow).get(enteringVariableColumn);

        BigFraction coefficient = BigFraction.ONE.divide(targetVariableValue);

        //Multiply data matrix row by coefficient
        simplexTable.data.set(leavingVariableRow, simplexTable.data.get(leavingVariableRow).stream().map(i -> i.multiply(coefficient)).collect(Collectors.toCollection(ArrayList::new)));
        simplexTable.rhs.set(leavingVariableRow, simplexTable.rhs.get(leavingVariableRow).multiply(coefficient));
        return coefficient;
    }

    /**
     * Return leaving variable index if computable. Ignore degenerate t-vec items that are lower than 0.
     * If all values are < 0, then return empty (unbounded solution)
     * @param tVec
     * @return
     */
        protected int getLeavingVariableIndex(List<Optional<BigFraction>> tVec) {
        Optional<Integer> minimumIndex = Optional.empty();
        for (int i = 0; i < tVec.size(); i ++) {
            if (tVec.get(i).isEmpty()) {
                continue;
            }
            if ((tVec.get(i).get().signum() >= 0) && minimumIndex.isEmpty()) {
                minimumIndex = Optional.of(i);
            } else if (minimumIndex.isPresent() && tVec.get(i).get().signum() >= 0 && tVec.get(i).get().compareTo(tVec.get(minimumIndex.get()).get() /* safe since this index must have been set after check*/) < 0) {
                minimumIndex = Optional.of(i);
            }
        }

        if (minimumIndex.isEmpty()) {
            throw new IllegalStateException("No valid entry in T-vector found.");
        }
        return minimumIndex.get();
    }

    /**
     * Compute T vector given entering variable index
     * @return
     */
    protected List<Optional<BigFraction>> computeTVector(int enteringVariableIndex, SimplexTable simplexTable) {
        List<Optional<BigFraction>> tVec = new ArrayList<>(simplexTable.rhs.size());
        for (int i = 0; i < simplexTable.rhs.size(); i ++) {
            if (simplexTable.data.get(i).get(enteringVariableIndex).signum() <= 0) {
                tVec.add(Optional.empty());
                continue;
            }
            tVec.add(Optional.of(simplexTable.rhs.get(i).divide(simplexTable.data.get(i).get(enteringVariableIndex))));
        }
        return tVec;
    }

    /**
     * Given non solved simplex table, return index of the entering variable in variables list
     * @return
     */
    protected int getEnteringVariableIndex(SimplexTable simplexTable) {
        int minIndex = 0;
        BigFraction min = simplexTable.objectiveFunctionRow.getFirst();
        for (int i = 1; i < simplexTable.objectiveFunctionRow.size(); i ++) {
            if (simplexTable.objectiveFunctionRow.get(i).compareTo(min) < 0) {
                minIndex = i;
                min = simplexTable.objectiveFunctionRow.get(i);
            }
        }
        return minIndex;
    }

    /**
     * Return true if given SimplexTable is solved
     * @param simplexTable
     * @return
     */
    protected boolean isSimplexTableSolved(SimplexTable simplexTable) {
        long negativeCount = simplexTable.objectiveFunctionRow.stream().filter(i -> i.signum() < 0).count();
        return negativeCount == 0;
    }

}
