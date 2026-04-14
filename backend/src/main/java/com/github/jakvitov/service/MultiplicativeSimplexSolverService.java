package com.github.jakvitov.service;

import com.github.jakvitov.dto.SimplexTableDto;
import com.github.jakvitov.dto.solver.SolutionStatus;
import com.github.jakvitov.dto.solver.SolveLpRequestDto;
import com.github.jakvitov.dto.solver.config.SolverConfigurationDto;
import com.github.jakvitov.dto.solver.multiplicative.MultiplicativeSimplexIterationDto;
import com.github.jakvitov.dto.solver.multiplicative.MultiplicativeSimplexPhaseOneSolutionDto;
import com.github.jakvitov.dto.solver.multiplicative.MultiplicativeSimplexPhaseTwoSolutionDto;
import com.github.jakvitov.dto.solver.multiplicative.SolveLpMultiplicativeSimplexResponseDto;
import com.github.jakvitov.dto.solver.revised.NonBasicVariableCurrentReducedCostCalculationDto;
import com.github.jakvitov.dto.solver.twophase.TwoPhaseSimplexObjectiveRowNormalizationDto;
import com.github.jakvitov.math.IntWrapper;
import com.github.jakvitov.math.LinearAlgebraService;
import com.github.jakvitov.mps.MpsData;
import com.github.jakvitov.mps.MpsDataTransformedBounds;
import com.github.jakvitov.simplex.OptimisationTarget;
import com.github.jakvitov.simplex.SimplexTable;
import com.github.jakvitov.utils.MemoryUtils;
import io.micronaut.core.annotation.Nullable;
import jakarta.inject.Inject;
import jakarta.inject.Singleton;
import org.hipparchus.fraction.BigFraction;

import java.util.*;
import java.util.stream.Collectors;

import static com.github.jakvitov.service.SolverConfigurationService.SolverConfigurationConstants.MS_MAX_ITER;
import static com.github.jakvitov.service.SolverConfigurationService.SolverConfigurationConstants.RS_MAX_CYCLE;

@Singleton
public class MultiplicativeSimplexSolverService {

    @Inject
    private LinearAlgebraService linearAlgebraService;

    @Inject
    private BasicSimplexSolverService basicSimplexSolverService;

    @Inject
    private TwoPhaseSimplexSolverService twoPhaseSimplexSolverService;

    @Inject
    private RevisedSimplexSolverService revisedSimplexSolverService;

    @Inject
    private SolverConfigurationService configurationService;

    public SolveLpMultiplicativeSimplexResponseDto handleSolveMultiplicativeSimplexRequest(SolveLpRequestDto solveLpRequestDto) {
        MpsData mpsData = MpsData.parse(solveLpRequestDto.data());
        MpsDataTransformedBounds mpsDataTransformedBounds = new MpsDataTransformedBounds(mpsData);
        SimplexTable simplexTable = SimplexTable.fromMpsData(mpsDataTransformedBounds);

        if (solveLpRequestDto.optimisationTarget().equals(OptimisationTarget.MIN)) {
            basicSimplexSolverService.convertObjectiveRowForMinimalization(simplexTable);
        }

        SolveLpMultiplicativeSimplexResponseDto responseDto = new SolveLpMultiplicativeSimplexResponseDto();
        responseDto.setInitialSimplexTable(new SimplexTableDto(simplexTable));

        List<BigFraction> originalObjectiveRow = new ArrayList<>(simplexTable.objectiveFunctionRow);
        IntWrapper iterations = IntWrapper.of(0);
        Map<Integer, Integer> visitedBaseCount = new HashMap<>();

        if (simplexTable.containsArtificialVariables) {
            boolean continueToPhaseTwo = solveMultiplicativeSimplexPhaseOne(simplexTable,responseDto, visitedBaseCount, iterations, solveLpRequestDto.solverConfiguration());
            if (continueToPhaseTwo) {
                solveMultiplicativeSimplexPhaseTwo(simplexTable, solveLpRequestDto.optimisationTarget(), responseDto, visitedBaseCount, iterations, originalObjectiveRow, solveLpRequestDto.solverConfiguration());
            }
        } else {
            solveMultiplicativeSimplexPhaseTwo(simplexTable, solveLpRequestDto.optimisationTarget(), responseDto, visitedBaseCount, iterations, originalObjectiveRow, solveLpRequestDto.solverConfiguration());
        }

        return responseDto;
    }

    private boolean solveMultiplicativeSimplexPhaseOne(SimplexTable originalSimplexTable, SolveLpMultiplicativeSimplexResponseDto responseDto, Map<Integer, Integer> visitedBaseCount, IntWrapper iterationCount, @Nullable SolverConfigurationDto solverConfigurationInput) {
        MultiplicativeSimplexPhaseOneSolutionDto multiplicativeSimplexPhaseOneSolutionDto = new MultiplicativeSimplexPhaseOneSolutionDto();

        twoPhaseSimplexSolverService.setupObjectiveRowBeforePhaseOne(originalSimplexTable);
        multiplicativeSimplexPhaseOneSolutionDto.setInitialSimplexTable(new SimplexTableDto(originalSimplexTable));

        //Add artificial variable rows to objective row to adjust them
        TwoPhaseSimplexObjectiveRowNormalizationDto artificialVariablesNormalization = twoPhaseSimplexSolverService.normalizeArtificialVariables(originalSimplexTable);
        multiplicativeSimplexPhaseOneSolutionDto.setArtificialVariablesNormalization(artificialVariablesNormalization);

        List<String> currentBasis = new ArrayList<>(originalSimplexTable.baseVariables);
        visitedBaseCount.put(currentBasis.hashCode(), 1);

        List<List<BigFraction>> inverseBasisMatrix = linearAlgebraService.createIdentityMatrix(originalSimplexTable.baseVariables.size());

        while (iterationCount.value < configurationService.getConfig(MS_MAX_ITER, solverConfigurationInput)) {

            if (visitedBaseCount.containsKey(currentBasis.hashCode()) && visitedBaseCount.get(currentBasis.hashCode()) > configurationService.getConfig(RS_MAX_CYCLE, solverConfigurationInput)) {
                responseDto.setSolutionStatus(SolutionStatus.CYCLE);
                responseDto.setMultiplicativeSimplexPhaseOneSolutionDto(multiplicativeSimplexPhaseOneSolutionDto);
            }

            MultiplicativeSimplexIterationDto iterationDto = new MultiplicativeSimplexIterationDto();
            iterationDto.setCurrentBasis(new ArrayList<>(currentBasis));

            //Get B given current basis
            List<List<BigFraction>> initialBasisMatrix = revisedSimplexSolverService.getBasisMatrix(originalSimplexTable, currentBasis);
            iterationDto.setInitialBasisMatrix(MemoryUtils.copyMatrix(initialBasisMatrix));

            //Set basis matrix inverse from last iteration
            iterationDto.setInitialBasisMatrixInverse(MemoryUtils.copyMatrix(inverseBasisMatrix));

            //Compute current basis solution (RHS) x_B
            List<List<BigFraction>> xB = linearAlgebraService.multiplyMatricesOrExc(inverseBasisMatrix, originalSimplexTable.getRhsInMatrixForm());
            iterationDto.setB(MemoryUtils.copyMatrix(originalSimplexTable.getRhsInMatrixForm()));
            iterationDto.setXB(MemoryUtils.copyMatrix(xB));

            //Get c_b^T
            List<List<BigFraction>> originalSimplexTableReducedCosts = revisedSimplexSolverService.getReducedCostsFromSimplexTable(originalSimplexTable, currentBasis);
            iterationDto.setOriginalSimplexTableReducedCosts(MemoryUtils.copyMatrix(originalSimplexTableReducedCosts));

            //Compute y^t = c_b^T * B^(-1)
            List<List<BigFraction>> yT = linearAlgebraService.multiplyMatricesOrExc(originalSimplexTableReducedCosts, inverseBasisMatrix);
            iterationDto.setYT(MemoryUtils.copyMatrix(yT));

            //Non-basic variable column index -> its current iteration reduced cost calculation dto
            Map<Integer, NonBasicVariableCurrentReducedCostCalculationDto> nonBasicVariablesCurrentReducedCostsCalculation = revisedSimplexSolverService.computeNonBasicVariablesCurrentReducedCosts(originalSimplexTable, currentBasis, yT);
            iterationDto.setNonBasicVariablesCurrentReducedCosts(nonBasicVariablesCurrentReducedCostsCalculation.values().stream().toList());
            //Non-basic variable column index -> its current iteration reduced cost
            Map<Integer, BigFraction> nonBasicVariablesCurrentReducedCosts = nonBasicVariablesCurrentReducedCostsCalculation.entrySet().stream().collect(Collectors.toMap(Map.Entry::getKey, i -> i.getValue().getResult(), (existing, replacement) -> replacement));

            //Choose entering variable x_j index
            Optional<Integer> enteringVariableIndex = revisedSimplexSolverService.getEnteringVariableIndex(nonBasicVariablesCurrentReducedCosts);

            //Optimal solution found
            if (enteringVariableIndex.isEmpty()) {
                multiplicativeSimplexPhaseOneSolutionDto.getIterations().add(iterationDto);
                multiplicativeSimplexPhaseOneSolutionDto.setResultBase(new ArrayList<>(currentBasis));
                responseDto.setMultiplicativeSimplexPhaseOneSolutionDto(multiplicativeSimplexPhaseOneSolutionDto);
                return true;
            }

            iterationDto.setEnteringVariableIndex(enteringVariableIndex.get());
            iterationDto.setEnteringVariableName(originalSimplexTable.variables.get(enteringVariableIndex.get()));

            //Compute the direction vector d = B^-1 * a_j (a_j being x_j column in original simplex table)
            List<List<BigFraction>> d = linearAlgebraService.multiplyMatricesOrExc(inverseBasisMatrix, originalSimplexTable.getDataColumnInMatrixForm(enteringVariableIndex.get()));
            iterationDto.setDirectionVector(MemoryUtils.copyMatrix(d));
            iterationDto.setEnteringVariableColumnInOriginalSimplexTable(originalSimplexTable.getDataColumnInMatrixForm(enteringVariableIndex.get()));

            //Unbounded solution
            if (revisedSimplexSolverService.isUnbounded(d)) {
                multiplicativeSimplexPhaseOneSolutionDto.getIterations().add(iterationDto);
                responseDto.setMultiplicativeSimplexPhaseOneSolutionDto(multiplicativeSimplexPhaseOneSolutionDto);
                responseDto.setSolutionStatus(SolutionStatus.UNBOUNDED);
                return false;
            }

            //Compute ratio vector test
            List<Optional<BigFraction>> ratioVector = revisedSimplexSolverService.computeRatioVector(originalSimplexTable, d, xB);
            iterationDto.setRatioVector(ratioVector.stream().map(i -> i.orElse(BigFraction.ZERO)).toList());

            int leavingVariableIndex = twoPhaseSimplexSolverService.getLeavingVariableIndexForPhaseOne(ratioVector);
            iterationDto.setLeavingVariableIndex(leavingVariableIndex);
            iterationDto.setLeavingVariableName(currentBasis.get(leavingVariableIndex));

            //Update basis
            currentBasis.set(leavingVariableIndex, originalSimplexTable.variables.get(enteringVariableIndex.get()));
            iterationDto.setUpdatedBasis(new ArrayList<>(currentBasis));

            //Updated visited base count
            if (visitedBaseCount.containsKey(currentBasis.hashCode())) {
                visitedBaseCount.put(currentBasis.hashCode(), visitedBaseCount.get(currentBasis.hashCode()) + 1);
            } else {
                visitedBaseCount.put(currentBasis.hashCode(), 1);
            }

            //Create E for given iteration
            List<List<BigFraction>> elementaryMatrix = createElementaryMatrix(d, leavingVariableIndex);
            //No need to copy, elementary matrix is not changed
            iterationDto.setElementaryMatrix(elementaryMatrix);

            List<List<BigFraction>> elementaryMatrixInverse = linearAlgebraService.getMatrixInversionOrExc(elementaryMatrix);
            //No need to copy, elementary matrix inverse is not changed
            iterationDto.setElementaryMatrixInverse(elementaryMatrixInverse);

            List<List<BigFraction>> nextIterationInverseBasisMatrix = linearAlgebraService.multiplyMatricesOrExc(elementaryMatrixInverse, inverseBasisMatrix);
            iterationDto.setNextIterationBasisInverse(MemoryUtils.copyMatrix(nextIterationInverseBasisMatrix));
            inverseBasisMatrix = nextIterationInverseBasisMatrix;

            multiplicativeSimplexPhaseOneSolutionDto.getIterations().add(iterationDto);
            iterationCount.value ++;
        }

        responseDto.setSolutionStatus(SolutionStatus.MAX_ITERATIONS);
        responseDto.setMultiplicativeSimplexPhaseOneSolutionDto(multiplicativeSimplexPhaseOneSolutionDto);
        return false;
    }

    private void solveMultiplicativeSimplexPhaseTwo(SimplexTable originalSimplexTable, OptimisationTarget optimisationTarget, SolveLpMultiplicativeSimplexResponseDto responseDto, Map<Integer, Integer> visitedBaseCount, IntWrapper iterationCount, List<BigFraction> originalObjectiveRow, @Nullable SolverConfigurationDto solverConfigurationInput) {
        MultiplicativeSimplexPhaseTwoSolutionDto multiplicativeSimplexPhaseTwoSolutionDto = new MultiplicativeSimplexPhaseTwoSolutionDto();

        twoPhaseSimplexSolverService.removeArtificialVariablesAfterPhaseOne(originalSimplexTable, originalObjectiveRow);
        //Use result base and use it in the cropped simplex table
        originalSimplexTable.objectiveFunctionRow = originalObjectiveRow;
        originalSimplexTable.objectiveValue = BigFraction.ZERO;

        List<String> currentBasis;
        if (responseDto.getMultiplicativeSimplexPhaseOneSolutionDto() == null) {
            currentBasis = new ArrayList<>(originalSimplexTable.baseVariables);
        } else {
            //Current initial basis is result basis from phase I
            currentBasis = new ArrayList<>(responseDto.getMultiplicativeSimplexPhaseOneSolutionDto().getResultBase());
        }

        multiplicativeSimplexPhaseTwoSolutionDto.setInitialFeasibleBase(new ArrayList<>(currentBasis));

        List<List<BigFraction>> inverseBasisMatrix = linearAlgebraService.createIdentityMatrix(originalSimplexTable.baseVariables.size());

        while (iterationCount.value < configurationService.getConfig(MS_MAX_ITER, solverConfigurationInput)) {
            if (visitedBaseCount.containsKey(currentBasis.hashCode()) && visitedBaseCount.get(currentBasis.hashCode()) > configurationService.getConfig(RS_MAX_CYCLE, solverConfigurationInput)) {
                responseDto.setSolutionStatus(SolutionStatus.CYCLE);
                responseDto.setMultiplicativeSimplexPhaseTwoSolutionDto(multiplicativeSimplexPhaseTwoSolutionDto);
                return;
            }

            MultiplicativeSimplexIterationDto iterationDto = new MultiplicativeSimplexIterationDto();
            iterationDto.setCurrentBasis(new ArrayList<>(currentBasis));

            //Get B given current basis
            List<List<BigFraction>> initialBasisMatrix = revisedSimplexSolverService.getBasisMatrix(originalSimplexTable, currentBasis);
            iterationDto.setInitialBasisMatrix(MemoryUtils.copyMatrix(initialBasisMatrix));

            //Set basis matrix inverse from last iteration
            iterationDto.setInitialBasisMatrixInverse(MemoryUtils.copyMatrix(inverseBasisMatrix));

            //Compute current basis solution (RHS) x_B
            List<List<BigFraction>> xB = linearAlgebraService.multiplyMatricesOrExc(inverseBasisMatrix, originalSimplexTable.getRhsInMatrixForm());
            iterationDto.setB(MemoryUtils.copyMatrix(originalSimplexTable.getRhsInMatrixForm()));
            iterationDto.setXB(MemoryUtils.copyMatrix(xB));

            //Get c_b^T
            List<List<BigFraction>> originalSimplexTableReducedCosts = revisedSimplexSolverService.getReducedCostsFromSimplexTable(originalSimplexTable, currentBasis);
            iterationDto.setOriginalSimplexTableReducedCosts(MemoryUtils.copyMatrix(originalSimplexTableReducedCosts));

            //Compute y^t = c_b^T * B^(-1)
            List<List<BigFraction>> yT = linearAlgebraService.multiplyMatricesOrExc(originalSimplexTableReducedCosts, inverseBasisMatrix);
            iterationDto.setYT(MemoryUtils.copyMatrix(yT));

            //Non-basic variable column index -> its current iteration reduced cost calculation dto
            Map<Integer, NonBasicVariableCurrentReducedCostCalculationDto> nonBasicVariablesCurrentReducedCostsCalculation = revisedSimplexSolverService.computeNonBasicVariablesCurrentReducedCosts(originalSimplexTable, currentBasis, yT);
            iterationDto.setNonBasicVariablesCurrentReducedCosts(nonBasicVariablesCurrentReducedCostsCalculation.values().stream().toList());
            //Non-basic variable column index -> its current iteration reduced cost
            Map<Integer, BigFraction> nonBasicVariablesCurrentReducedCosts = nonBasicVariablesCurrentReducedCostsCalculation.entrySet().stream().collect(Collectors.toMap(Map.Entry::getKey, i -> i.getValue().getResult(), (existing, replacement) -> replacement));

            //Choose entering variable x_j index
            Optional<Integer> enteringVariableIndex = revisedSimplexSolverService.getEnteringVariableIndex(nonBasicVariablesCurrentReducedCosts);

            //Optimal solution found
            if (enteringVariableIndex.isEmpty()) {
                multiplicativeSimplexPhaseTwoSolutionDto.getIterations().add(iterationDto);
                responseDto.setSolutionStatus(SolutionStatus.SOLVED);
                responseDto.setResultVariableValues(revisedSimplexSolverService.getResultVariableValues(xB, currentBasis));
                // 1x1 matrix with the objective function value
                List<List<BigFraction>> objectiveFunctionValueMatrixNegated = linearAlgebraService.multiplyMatricesOrExc(originalSimplexTableReducedCosts, xB);

                if (optimisationTarget.equals(OptimisationTarget.MIN)) {
                    responseDto.setSolutionObjectiveFunctionValue(objectiveFunctionValueMatrixNegated.getFirst().getFirst());
                } else {
                    responseDto.setSolutionObjectiveFunctionValue(objectiveFunctionValueMatrixNegated.getFirst().getFirst().negate());
                }
                responseDto.setMultiplicativeSimplexPhaseTwoSolutionDto(multiplicativeSimplexPhaseTwoSolutionDto);
                return;
            }

            iterationDto.setEnteringVariableIndex(enteringVariableIndex.get());
            iterationDto.setEnteringVariableName(originalSimplexTable.variables.get(enteringVariableIndex.get()));

            //Compute the direction vector d = B^-1 * a_j (a_j being x_j column in original simplex table)
            List<List<BigFraction>> d = linearAlgebraService.multiplyMatricesOrExc(inverseBasisMatrix, originalSimplexTable.getDataColumnInMatrixForm(enteringVariableIndex.get()));
            iterationDto.setDirectionVector(MemoryUtils.copyMatrix(d));
            iterationDto.setEnteringVariableColumnInOriginalSimplexTable(originalSimplexTable.getDataColumnInMatrixForm(enteringVariableIndex.get()));

            //Unbounded solution
            if (revisedSimplexSolverService.isUnbounded(d)) {
                multiplicativeSimplexPhaseTwoSolutionDto.getIterations().add(iterationDto);
                responseDto.setSolutionStatus(SolutionStatus.UNBOUNDED);
                responseDto.setMultiplicativeSimplexPhaseTwoSolutionDto(multiplicativeSimplexPhaseTwoSolutionDto);
                return;
            }

            //Compute ratio vector test
            List<Optional<BigFraction>> ratioVector = revisedSimplexSolverService.computeRatioVector(originalSimplexTable, d, xB);
            iterationDto.setRatioVector(ratioVector.stream().map(i -> i.orElse(BigFraction.ZERO)).toList());

            int leavingVariableIndex = twoPhaseSimplexSolverService.getLeavingVariableIndexForPhaseOne(ratioVector);
            iterationDto.setLeavingVariableIndex(leavingVariableIndex);
            iterationDto.setLeavingVariableName(currentBasis.get(leavingVariableIndex));

            //Update basis
            currentBasis.set(leavingVariableIndex, originalSimplexTable.variables.get(enteringVariableIndex.get()));
            iterationDto.setUpdatedBasis(new ArrayList<>(currentBasis));

            //Updated visited base count
            if (visitedBaseCount.containsKey(currentBasis.hashCode())) {
                visitedBaseCount.put(currentBasis.hashCode(), visitedBaseCount.get(currentBasis.hashCode()) + 1);
            } else {
                visitedBaseCount.put(currentBasis.hashCode(), 1);
            }

            //Create E for given iteration
            List<List<BigFraction>> elementaryMatrix = createElementaryMatrix(d, leavingVariableIndex);
            //No need to copy, elementary matrix is not changed
            iterationDto.setElementaryMatrix(elementaryMatrix);

            List<List<BigFraction>> elementaryMatrixInverse = linearAlgebraService.getMatrixInversionOrExc(elementaryMatrix);
            //No need to copy, elementary matrix inverse is not changed
            iterationDto.setElementaryMatrixInverse(elementaryMatrixInverse);

            List<List<BigFraction>> nextIterationInverseBasisMatrix = linearAlgebraService.multiplyMatricesOrExc(elementaryMatrixInverse, inverseBasisMatrix);
            iterationDto.setNextIterationBasisInverse(MemoryUtils.copyMatrix(nextIterationInverseBasisMatrix));
            inverseBasisMatrix = nextIterationInverseBasisMatrix;

            multiplicativeSimplexPhaseTwoSolutionDto.getIterations().add(iterationDto);
            iterationCount.value ++;
        }

        responseDto.setSolutionStatus(SolutionStatus.MAX_ITERATIONS);
        responseDto.setMultiplicativeSimplexPhaseTwoSolutionDto(multiplicativeSimplexPhaseTwoSolutionDto);
    }

    /**
     * Given direction vector d and leaving variable index, construct elementary matrix for given iteration
     * @param d
     * @param leavingVariableIndex
     * @return
     */
    private List<List<BigFraction>> createElementaryMatrix(List<List<BigFraction>> d, int leavingVariableIndex) {
        List<List<BigFraction>> res = linearAlgebraService.createIdentityMatrix(d.size());

        for (int rowIndex = 0; rowIndex < res.size(); rowIndex++) {
            res.get(rowIndex).set(leavingVariableIndex, d.get(rowIndex).getFirst());
        }
        return res;
    }



}
