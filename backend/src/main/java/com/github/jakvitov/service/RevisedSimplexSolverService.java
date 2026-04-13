package com.github.jakvitov.service;

import com.github.jakvitov.dto.SimplexTableDto;
import com.github.jakvitov.dto.solver.SolutionStatus;
import com.github.jakvitov.dto.solver.SolveLpRequestDto;
import com.github.jakvitov.dto.solver.revised.*;
import com.github.jakvitov.dto.solver.twophase.TwoPhaseSimplexObjectiveRowNormalizationDto;
import com.github.jakvitov.math.IntWrapper;
import com.github.jakvitov.math.LinearAlgebraService;
import com.github.jakvitov.mps.MpsData;
import com.github.jakvitov.mps.MpsDataTransformedBounds;
import com.github.jakvitov.simplex.OptimisationTarget;
import com.github.jakvitov.simplex.SimplexTable;
import com.github.jakvitov.utils.MemoryUtils;
import io.micronaut.context.annotation.Value;
import jakarta.inject.Inject;
import jakarta.inject.Singleton;
import org.hipparchus.fraction.BigFraction;

import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

@Singleton
public class RevisedSimplexSolverService {

    @Inject
    private LinearAlgebraService linearAlgebraService;

    @Inject
    private BasicSimplexSolverService basicSimplexSolverService;

    @Inject
    private TwoPhaseSimplexSolverService twoPhaseSimplexSolverService;

    @Value("${simpler.simplex.revised.max.iterations}")
    private Integer maxIterations;

    @Value("${simpler.simplex.revised.max.base.cycles}")
    private Integer maxCycles;

    public SolveLpRevisedSimlexResponseDto handleSolveRevisedSimplexRequest(SolveLpRequestDto solveLpRequestDto) {
        MpsData mpsData = MpsData.parse(solveLpRequestDto.data());
        MpsDataTransformedBounds mpsDataTransformedBounds = new MpsDataTransformedBounds(mpsData);
        SimplexTable simplexTable = SimplexTable.fromMpsData(mpsDataTransformedBounds);

        SolveLpRevisedSimlexResponseDto responseDto = new SolveLpRevisedSimlexResponseDto();
        responseDto.setInitialSimplexTable(new SimplexTableDto(simplexTable));

        List<BigFraction> originalObjectiveRow = new ArrayList<>(simplexTable.objectiveFunctionRow);
        IntWrapper iterations = IntWrapper.of(0);
        Map<Integer, Integer> visitedBaseCount = new HashMap<>();

        if (simplexTable.containsArtificialVariables) {
            boolean continueToPhaseTwo = solveRevisedSimplexPhaseOne(simplexTable, solveLpRequestDto.optimisationTarget(), responseDto, visitedBaseCount, iterations);
            if (continueToPhaseTwo) {
                solveRevisedSimplexPhaseTwo(simplexTable, solveLpRequestDto.optimisationTarget(), responseDto, visitedBaseCount, iterations, originalObjectiveRow);
            }
        } else {
            solveRevisedSimplexPhaseTwo(simplexTable, solveLpRequestDto.optimisationTarget(), responseDto, visitedBaseCount, iterations, originalObjectiveRow);
        }

        return responseDto;
    }

    private boolean solveRevisedSimplexPhaseOne(SimplexTable originalSimplexTable, OptimisationTarget optimisationTarget, SolveLpRevisedSimlexResponseDto responseDto, Map<Integer, Integer> visitedBaseCount, IntWrapper iterationCount) {

        RevisedSimplexPhaseOneSolutionDto revisedSimplexPhaseOneSolutionDto = new RevisedSimplexPhaseOneSolutionDto();

        //Make artificial variables 1 and all other 0
        twoPhaseSimplexSolverService.setupObjectiveRowBeforePhaseOne(originalSimplexTable);
        revisedSimplexPhaseOneSolutionDto.setInitialSimplexTable(new SimplexTableDto(originalSimplexTable));

        //Add artificial variable rows to objective row to adjust them
        TwoPhaseSimplexObjectiveRowNormalizationDto artificialVariablesNormalization = twoPhaseSimplexSolverService.normalizeArtificialVariables(originalSimplexTable);
        revisedSimplexPhaseOneSolutionDto.setArtificialVariablesNormalization(artificialVariablesNormalization);

        List<String> currentBasis = new ArrayList<>(originalSimplexTable.baseVariables);
        visitedBaseCount.put(currentBasis.hashCode(), 1);

        //Revised simplex iterations solving phase one
        while (iterationCount.value < maxIterations) {

            if (visitedBaseCount.containsKey(currentBasis.hashCode()) && visitedBaseCount.get(currentBasis.hashCode()) > maxCycles) {
                responseDto.setSolutionStatus(SolutionStatus.CYCLE);
                responseDto.setRevisedSimplexPhaseOneSolution(revisedSimplexPhaseOneSolutionDto);
                return false;
            }

            RevisedSimplexIterationDto iterationDto = new RevisedSimplexIterationDto();
            iterationDto.setCurrentBasis(new ArrayList<>(currentBasis));

            //Get B given current basis
            List<List<BigFraction>> initialBasisMatrix = getBasisMatrix(originalSimplexTable, currentBasis);
            iterationDto.setInitialBasisMatrix(MemoryUtils.copyMatrix(initialBasisMatrix));

            //Compute B^(-1)
            List<List<BigFraction>> initialBasisMatrixInverse = linearAlgebraService.getMatrixInversionOrExc(initialBasisMatrix);
            iterationDto.setInitialBasisMatrixInverse(MemoryUtils.copyMatrix(initialBasisMatrixInverse));

            //Compute current basis solution (RHS) x_B
            List<List<BigFraction>> xB = linearAlgebraService.multiplyMatricesOrExc(initialBasisMatrixInverse, originalSimplexTable.getRhsInMatrixForm());
            iterationDto.setB(MemoryUtils.copyMatrix(originalSimplexTable.getRhsInMatrixForm()));
            iterationDto.setXB(MemoryUtils.copyMatrix(xB));

            //Get c_b^T
            List<List<BigFraction>> originalSimplexTableReducedCosts = getReducedCostsFromSimplexTable(originalSimplexTable, currentBasis);
            iterationDto.setOriginalSimplexTableReducedCosts(MemoryUtils.copyMatrix(originalSimplexTableReducedCosts));

            //Compute y^t = c_b^T * B^(-1)
            List<List<BigFraction>> yT = linearAlgebraService.multiplyMatricesOrExc(originalSimplexTableReducedCosts, initialBasisMatrixInverse);
            iterationDto.setYT(MemoryUtils.copyMatrix(yT));

            //Non-basic variable column index -> its current iteration reduced cost calculation dto
            Map<Integer, NonBasicVariableCurrentReducedCostCalculationDto> nonBasicVariablesCurrentReducedCostsCalculation = computeNonBasicVariablesCurrentReducedCosts(originalSimplexTable, currentBasis, yT);
            iterationDto.setNonBasicVariablesCurrentReducedCosts(nonBasicVariablesCurrentReducedCostsCalculation.values().stream().toList());
            //Non-basic variable column index -> its current iteration reduced cost
            Map<Integer, BigFraction> nonBasicVariablesCurrentReducedCosts = nonBasicVariablesCurrentReducedCostsCalculation.entrySet().stream().collect(Collectors.toMap(Map.Entry::getKey, i -> i.getValue().getResult(), (existing, replacement) -> replacement));

            //Choose entering variable x_j index
            Optional<Integer> enteringVariableIndex = getEnteringVariableIndex(nonBasicVariablesCurrentReducedCosts);

            //Optimal solution found
            if (enteringVariableIndex.isEmpty()) {
                revisedSimplexPhaseOneSolutionDto.getIterations().add(iterationDto);
                revisedSimplexPhaseOneSolutionDto.setResultBase(new ArrayList<>(currentBasis));
                responseDto.setRevisedSimplexPhaseOneSolution(revisedSimplexPhaseOneSolutionDto);
                return true;
            }

            iterationDto.setEnteringVariableIndex(enteringVariableIndex.get());
            iterationDto.setEnteringVariableName(originalSimplexTable.variables.get(enteringVariableIndex.get()));

            //Compute the direction vector d = B^-1 * a_j (a_j being x_j column in original simplex table)
            List<List<BigFraction>> d = linearAlgebraService.multiplyMatricesOrExc(initialBasisMatrixInverse, originalSimplexTable.getDataColumnInMatrixForm(enteringVariableIndex.get()));
            iterationDto.setDirectionVector(MemoryUtils.copyMatrix(d));
            iterationDto.setEnteringVariableColumnInOriginalSimplexTable(originalSimplexTable.getDataColumnInMatrixForm(enteringVariableIndex.get()));

            //Unbounded solution
            if (isUnbounded(d)) {
                revisedSimplexPhaseOneSolutionDto.getIterations().add(iterationDto);
                responseDto.setRevisedSimplexPhaseOneSolution(revisedSimplexPhaseOneSolutionDto);
                responseDto.setSolutionStatus(SolutionStatus.UNBOUNDED);
                return false;
            }

            //Compute ratio vector test
            List<Optional<BigFraction>> ratioVector = computeRatioVector(originalSimplexTable, d, xB);
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

            revisedSimplexPhaseOneSolutionDto.getIterations().add(iterationDto);
            iterationCount.value ++;
        }

        responseDto.setSolutionStatus(SolutionStatus.MAX_ITERATIONS);
        responseDto.setRevisedSimplexPhaseOneSolution(revisedSimplexPhaseOneSolutionDto);
        return false;
    }

    private void solveRevisedSimplexPhaseTwo(SimplexTable originalSimplexTable, OptimisationTarget optimisationTarget, SolveLpRevisedSimlexResponseDto responseDto, Map<Integer, Integer> visitedBaseCount, IntWrapper iterationCount, List<BigFraction> originalObjectiveRow) {
        RevisedSimplexPhaseTwoSolutionDto revisedSimplexPhaseTwoSolutionDto = new RevisedSimplexPhaseTwoSolutionDto();

        twoPhaseSimplexSolverService.removeArtificialVariablesAfterPhaseOne(originalSimplexTable, originalObjectiveRow);
        //Use result base and use it in the cropped simplex table
        originalSimplexTable.objectiveFunctionRow = originalObjectiveRow;
        originalSimplexTable.objectiveValue = BigFraction.ZERO;

        List<String> currentBasis;
        if (responseDto.getRevisedSimplexPhaseOneSolution() == null) {
            currentBasis = new ArrayList<>(originalSimplexTable.baseVariables);
        } else {
            //Current initial basis is result basis from phase I
            currentBasis = new ArrayList<>(responseDto.getRevisedSimplexPhaseOneSolution().getResultBase());
        }
        revisedSimplexPhaseTwoSolutionDto.setInitialFeasibleBase(new ArrayList<>(currentBasis));

        while (iterationCount.value < maxIterations) {
            if (visitedBaseCount.containsKey(currentBasis.hashCode()) && visitedBaseCount.get(currentBasis.hashCode()) > maxCycles) {
                responseDto.setSolutionStatus(SolutionStatus.CYCLE);
                responseDto.setRevisedSimplexPhaseTwoSolutionDto(revisedSimplexPhaseTwoSolutionDto);
                return;
            }

            RevisedSimplexIterationDto iterationDto = new RevisedSimplexIterationDto();
            iterationDto.setCurrentBasis(new ArrayList<>(currentBasis));

            //Get B given current basis
            List<List<BigFraction>> initialBasisMatrix = getBasisMatrix(originalSimplexTable, currentBasis);
            iterationDto.setInitialBasisMatrix(MemoryUtils.copyMatrix(initialBasisMatrix));

            //Compute B^(-1)
            List<List<BigFraction>> initialBasisMatrixInverse = linearAlgebraService.getMatrixInversionOrExc(initialBasisMatrix);
            iterationDto.setInitialBasisMatrixInverse(MemoryUtils.copyMatrix(initialBasisMatrixInverse));

            //Compute current basis solution (RHS) x_B
            List<List<BigFraction>> xB = linearAlgebraService.multiplyMatricesOrExc(initialBasisMatrixInverse, originalSimplexTable.getRhsInMatrixForm());
            iterationDto.setB(MemoryUtils.copyMatrix(originalSimplexTable.getRhsInMatrixForm()));
            iterationDto.setXB(MemoryUtils.copyMatrix(xB));

            //Get c_b^T
            List<List<BigFraction>> originalSimplexTableReducedCosts = getReducedCostsFromSimplexTable(originalSimplexTable, currentBasis);
            iterationDto.setOriginalSimplexTableReducedCosts(MemoryUtils.copyMatrix(originalSimplexTableReducedCosts));

            //Compute y^t = c_b^T * B^(-1)
            List<List<BigFraction>> yT = linearAlgebraService.multiplyMatricesOrExc(originalSimplexTableReducedCosts, initialBasisMatrixInverse);
            iterationDto.setYT(MemoryUtils.copyMatrix(yT));

            //Non-basic variable column index -> its current iteration reduced cost calculation dto
            Map<Integer, NonBasicVariableCurrentReducedCostCalculationDto> nonBasicVariablesCurrentReducedCostsCalculation = computeNonBasicVariablesCurrentReducedCosts(originalSimplexTable, currentBasis, yT);
            iterationDto.setNonBasicVariablesCurrentReducedCosts(nonBasicVariablesCurrentReducedCostsCalculation.values().stream().toList());
            //Non-basic variable column index -> its current iteration reduced cost
            Map<Integer, BigFraction> nonBasicVariablesCurrentReducedCosts = nonBasicVariablesCurrentReducedCostsCalculation.entrySet().stream().collect(Collectors.toMap(Map.Entry::getKey, i -> i.getValue().getResult(), (existing, replacement) -> replacement));

            //Choose entering variable x_j index
            Optional<Integer> enteringVariableIndex = getEnteringVariableIndex(nonBasicVariablesCurrentReducedCosts);

            //Optimal solution found
            if (enteringVariableIndex.isEmpty()) {
                revisedSimplexPhaseTwoSolutionDto.getIterations().add(iterationDto);
                responseDto.setSolutionStatus(SolutionStatus.SOLVED);
                responseDto.setResultVariableValues(getResultVariableValues(xB, currentBasis));
                // 1x1 matrix with the objective function value
                List<List<BigFraction>> objectiveFunctionValueMatrixNegated = linearAlgebraService.multiplyMatricesOrExc(originalSimplexTableReducedCosts, xB);
                responseDto.setSolutionObjectiveFunctionValue(objectiveFunctionValueMatrixNegated.getFirst().getFirst().negate());
                responseDto.setRevisedSimplexPhaseTwoSolutionDto(revisedSimplexPhaseTwoSolutionDto);
                return;
            }

            iterationDto.setEnteringVariableIndex(enteringVariableIndex.get());
            iterationDto.setEnteringVariableName(originalSimplexTable.variables.get(enteringVariableIndex.get()));

            //Compute the direction vector d = B^-1 * a_j (a_j being x_j column in original simplex table)
            List<List<BigFraction>> d = linearAlgebraService.multiplyMatricesOrExc(initialBasisMatrixInverse, originalSimplexTable.getDataColumnInMatrixForm(enteringVariableIndex.get()));
            iterationDto.setDirectionVector(MemoryUtils.copyMatrix(d));
            iterationDto.setEnteringVariableColumnInOriginalSimplexTable(originalSimplexTable.getDataColumnInMatrixForm(enteringVariableIndex.get()));

            //Unbounded solution
            if (isUnbounded(d)) {
                revisedSimplexPhaseTwoSolutionDto.getIterations().add(iterationDto);
                responseDto.setSolutionStatus(SolutionStatus.UNBOUNDED);
                responseDto.setRevisedSimplexPhaseTwoSolutionDto(revisedSimplexPhaseTwoSolutionDto);
                return;
            }

            //Compute ratio vector test
            List<Optional<BigFraction>> ratioVector = computeRatioVector(originalSimplexTable, d, xB);
            iterationDto.setRatioVector(ratioVector.stream().map(i -> i.orElse(BigFraction.ZERO)).toList());

            int leavingVariableIndex = basicSimplexSolverService.getLeavingVariableIndex(ratioVector);
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

            revisedSimplexPhaseTwoSolutionDto.getIterations().add(iterationDto);
            iterationCount.value ++;
        }

        responseDto.setSolutionStatus(SolutionStatus.MAX_ITERATIONS);
        responseDto.setRevisedSimplexPhaseTwoSolutionDto(revisedSimplexPhaseTwoSolutionDto);
    }

    /**
     * Given final xB (RHS) and current basis, return Map variable name -> optimal value
     * @param xB
     * @param currentBasis
     * @return
     */
    private Map<String, BigFraction> getResultVariableValues(List<List<BigFraction>> xB, List<String> currentBasis) {
        Map<String, BigFraction> result = new HashMap<>(currentBasis.size());
        IntStream.range(0, currentBasis.size()).boxed().forEach((i) -> {
            result.put(currentBasis.get(i), xB.get(i).getFirst());
        });
        return result;
    }

    /**
     * Given current d (pivot column) vector, return true if solution is unbounded
     * @param d
     * @return
     */
    private boolean isUnbounded(List<List<BigFraction>> d) {
        boolean unbounded = true;
        for (List<BigFraction> row : d) {
            for (BigFraction value : row) {
                if (value.signum() > 0) {
                    unbounded = false;
                    break;
                }
            }
        }
        return unbounded;
    }

    /**
     * Calculate ratio vector (t-vec)
     * @param originalSimplexTable
     * @param d
     * @param xB
     * @return
     */
    private List<Optional<BigFraction>> computeRatioVector(SimplexTable originalSimplexTable, List<List<BigFraction>> d, List<List<BigFraction>> xB) {
        List<Optional<BigFraction>> ratioVector = new ArrayList<>(originalSimplexTable.baseVariables.size());
        IntStream.range(0, originalSimplexTable.baseVariables.size()).boxed().forEach((i) -> {
            //Column vector
            BigFraction dItem = d.get(i).getFirst();
            BigFraction xBItem = xB.get(i).getFirst();
            if (dItem.signum() <= 0) {
                ratioVector.add(Optional.empty());
            } else {
                ratioVector.add(Optional.of(xBItem.divide(dItem)));
            }
        });
        return ratioVector;
    }

    /**
     * Given map of column indexes of non basic variables -> their current reduced costs, return Optional of entering variable
     * If solution is optimal, return Optional empty
     * @param nonBasicVariablesCurrentReducedCosts
     * @return
     */
    private Optional<Integer> getEnteringVariableIndex(Map<Integer, BigFraction> nonBasicVariablesCurrentReducedCosts) {
        Optional<Integer> enteringVariableIndex = Optional.empty();
        Optional<BigFraction> minReducedCost = Optional.empty();
        for (Map.Entry<Integer, BigFraction> nonBasicVariableCurrentReducedCost: nonBasicVariablesCurrentReducedCosts.entrySet()) {
            if (enteringVariableIndex.isEmpty() && nonBasicVariableCurrentReducedCost.getValue().signum() < 0) {
                enteringVariableIndex = Optional.of(nonBasicVariableCurrentReducedCost.getKey());
                minReducedCost = Optional.of(nonBasicVariableCurrentReducedCost.getValue());
            } else if (minReducedCost.isPresent() && nonBasicVariableCurrentReducedCost.getValue().compareTo(minReducedCost.get()) < 0) {
                enteringVariableIndex = Optional.of(nonBasicVariableCurrentReducedCost.getKey());
                minReducedCost = Optional.of(nonBasicVariableCurrentReducedCost.getValue());
            }
        }

        return enteringVariableIndex;
    }

    /**
     * Given original simplex table and current basis variables, return map of column indexes of non basic variables -> their current reduced costs
     * @param originalSimplexTable
     * @param currentBasis
     * @return
     */
    private Map<Integer, NonBasicVariableCurrentReducedCostCalculationDto> computeNonBasicVariablesCurrentReducedCosts(SimplexTable originalSimplexTable, List<String> currentBasis, List<List<BigFraction>> yT) {
        List<Integer> nonBasicVariablesColumnIndexes = getNonBasicVariablesColumnIndexes(originalSimplexTable, currentBasis);

        //Non basic variable column index -> its current iteration reduced cost
        Map<Integer, NonBasicVariableCurrentReducedCostCalculationDto> nonBasicVariablesCurrentReducedCosts = new HashMap<>(nonBasicVariablesColumnIndexes.size());
        nonBasicVariablesColumnIndexes.forEach(nonBasicVariableIndex -> {
            BigFraction cJ = originalSimplexTable.objectiveFunctionRow.get(nonBasicVariableIndex);
            BigFraction nonBasicVariableReducedCost = linearAlgebraService.multiplyMatricesOrExc(yT, originalSimplexTable.getDataColumnInMatrixForm(nonBasicVariableIndex)).getFirst().getFirst();
            NonBasicVariableCurrentReducedCostCalculationDto nonBasicVariableCurrentReducedCostCalculationDto = new NonBasicVariableCurrentReducedCostCalculationDto();
            nonBasicVariableCurrentReducedCostCalculationDto.setCJ(cJ);
            nonBasicVariableCurrentReducedCostCalculationDto.setAJ(originalSimplexTable.getDataColumnInMatrixForm(nonBasicVariableIndex));
            nonBasicVariableCurrentReducedCostCalculationDto.setNonBasicVariableReducedCost(nonBasicVariableReducedCost);
            nonBasicVariableCurrentReducedCostCalculationDto.setResult(cJ.subtract(nonBasicVariableReducedCost));
            nonBasicVariableCurrentReducedCostCalculationDto.setVariableName(originalSimplexTable.variables.get(nonBasicVariableIndex));
            nonBasicVariablesCurrentReducedCosts.put(nonBasicVariableIndex, nonBasicVariableCurrentReducedCostCalculationDto);
        });

        return nonBasicVariablesCurrentReducedCosts;
    }

    /**
     * Given original simplex table and current basis variables names, return column indexes of all non basic variables
     * @param originalSimplexTable
     * @param currentBasisVariables
     * @return
     */
    private List<Integer> getNonBasicVariablesColumnIndexes(SimplexTable originalSimplexTable, List<String> currentBasisVariables) {
        Set<Integer> basicVariableColumnIndexes = currentBasisVariables.stream().map(baseVariableName -> originalSimplexTable.getVariableColumnIndex(baseVariableName).orElseThrow(() -> new IllegalStateException("Base variable " + baseVariableName + " not found among original simplex table variables!"))).collect(Collectors.toSet());
        return IntStream.range(0, originalSimplexTable.variables.size()).filter(i -> !basicVariableColumnIndexes.contains(i)).boxed().collect(Collectors.toList());
    }

    /**
     * Given original simplex table and current basis variable names, return their values in the original simplex table objective row
     * Return c_b^T given current base
     * @param originalSimplexTable
     * @param currentBasisVariables
     * @return
     */
    private List<List<BigFraction>> getReducedCostsFromSimplexTable(SimplexTable originalSimplexTable, List<String> currentBasisVariables) {
        List<List<BigFraction>> result = new ArrayList<>(currentBasisVariables.size());
        List<Integer> columnIndexes = currentBasisVariables.stream().map(baseVariableName -> originalSimplexTable.getVariableColumnIndex(baseVariableName).orElseThrow(() -> new IllegalStateException("Base variable " + baseVariableName + " not found among original simplex table variables!"))).toList();
        result.add(columnIndexes.stream().map((basisVariableColumnIndex) -> originalSimplexTable.objectiveFunctionRow.get(basisVariableColumnIndex)).toList());
        return result;
    }

    /**
     * Given original simplex table and current basis variables, return matrix basis of the variable values in the original
     * simplex table.
     * Return B given basis variable names
     * @param originalSimplexTable
     * @param currentBasisVariables
     * @return
     */
    private List<List<BigFraction>> getBasisMatrix(SimplexTable originalSimplexTable, List<String> currentBasisVariables) {
        List<List<BigFraction>> result = new ArrayList<>(currentBasisVariables.size());

        //Column indexes of the basis variables
        List<Integer> columnIndexes = currentBasisVariables.stream().map(baseVariableName -> originalSimplexTable.getVariableColumnIndex(baseVariableName).orElseThrow(() -> new IllegalStateException("Base variable " + baseVariableName + " not found among original simplex table variables!"))).toList();

        for (int rowIndex = 0; rowIndex < originalSimplexTable.baseVariables.size(); rowIndex ++) {
            List<BigFraction> resultRow = new ArrayList<>(columnIndexes.size());
            for (int columnIndex: columnIndexes) {
                resultRow.add(originalSimplexTable.data.get(rowIndex).get(columnIndex));
            }
            result.add(resultRow);
        }
        return result;
    }

}
