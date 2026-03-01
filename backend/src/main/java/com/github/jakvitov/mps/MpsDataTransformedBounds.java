package com.github.jakvitov.mps;

import org.hipparchus.fraction.BigFraction;

import java.util.LinkedHashMap;
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
    public Objsense objsense;

    public MpsDataTransformedBounds(MpsData originalMpsData) {
        this.name = originalMpsData.name;
        this.rows = originalMpsData.rows;

        this.columns = originalMpsData.columns;

        if (originalMpsData.rhs.size() > 1) {
            throw new MpsParsingException(MpsSections.RHS, "Multiple RHS definitions are not currently supported.");
        }
        this.rhs = originalMpsData.rhs.entrySet().iterator().next().getValue();
        this.objsense = originalMpsData.objsense;

        transformBoundsIntoNewRows(originalMpsData);
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


}
