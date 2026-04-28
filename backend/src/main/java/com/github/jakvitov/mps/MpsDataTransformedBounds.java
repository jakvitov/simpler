package com.github.jakvitov.mps;

import org.hipparchus.fraction.BigFraction;

import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

/**
 * MpsData object with cropped RHS to only one and Bounds transformed into rows.
 */
public class MpsDataTransformedBounds {

    public String name;
    public LinkedHashMap<String, RowType> rows;
    //Variable name -> row name -> value
    public LinkedHashMap<String, Map<String, BigFraction>> columns;
    //Rhs name -> Row name -> value
    public Map<String, BigFraction> rhs;

    public MpsDataTransformedBounds(MpsData originalMpsData) {
        this.name = originalMpsData.name;
        this.rows = originalMpsData.rows;

        this.columns = originalMpsData.columns;

        if (originalMpsData.rhs.size() > 1) {
            throw new MpsParsingException(MpsSections.RHS, "Multiple RHS definitions are not currently supported.");
        }
        this.rhs = originalMpsData.rhs.entrySet().iterator().next().getValue();

        transformBoundsIntoNewRows(originalMpsData);
        flipNegativeRows();
    }

    private void transformBoundsIntoNewRows(MpsData mpsData) {
        if (mpsData.bounds.size() > 1) {
            throw new MpsParsingException(MpsSections.BOUNDS, "Multiple bounds encountered. Only one Bound definition is currently supported.", String.join(", ", mpsData.bounds.keySet()));
        } else if (mpsData.bounds.isEmpty()) {
            return;
        }

        LinkedHashMap<String, LinkedHashMap<BoundType, BigFraction>> bound = mpsData.bounds.entrySet().iterator().next().getValue();

        for (Map.Entry<String, LinkedHashMap<BoundType, BigFraction>> variableBounds : bound.entrySet()) {
            String variableName = variableBounds.getKey();
            BigFraction upperbound = variableBounds.getValue().get(BoundType.UP);
            BigFraction lowerbound = variableBounds.getValue().get(BoundType.LO);


            if (upperbound != null) {
                String rowName = variableBounds.getKey() + "-UP";
                rows.put(rowName, RowType.L);
                rhs.put(rowName, upperbound);
                columns.get(variableName).put(rowName, BigFraction.ONE);
            }

            if (lowerbound != null) {
                String rowName = variableBounds.getKey() + "-LO";
                rows.put(rowName, RowType.G);
                rhs.put(rowName, lowerbound);
                columns.get(variableName).put(rowName, BigFraction.ONE);
            }
        }

    }

    /**
     * Flip all negative RHS rows
     */
    public void flipNegativeRows() {
        List<String> rowsToFlip = new ArrayList<>();
        for (Map.Entry<String, RowType> row : rows.entrySet()) {
            if (row.getValue() == RowType.N) {
                continue;
            }
            if(!rhs.containsKey(row.getKey())) {
                throw new MpsParsingException(MpsSections.RHS, "RHS does not contain row " + row.getKey());
            }
            //If row has negative RHS - sign needs to be flipped and rhs negated
            if (rhs.get(row.getKey()).compareTo(BigFraction.ZERO) < 0) {
                rowsToFlip.add(row.getKey());
            }
        }

        rowsToFlip.forEach(rowNameToFlip -> {
            this.rows.put(rowNameToFlip, this.rows.get(rowNameToFlip).flip());
            this.rhs.put(rowNameToFlip, this.rhs.get(rowNameToFlip).negate());
            this.columns.values().forEach(variableValues -> {
                if (variableValues.containsKey(rowNameToFlip)) {
                    variableValues.put(rowNameToFlip, variableValues.get(rowNameToFlip).negate());
                }
            });
        });
    }



}
