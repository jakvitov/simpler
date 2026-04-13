package com.github.jakvitov.utils;

import org.hipparchus.fraction.BigFraction;

import java.util.ArrayList;
import java.util.List;

public class MemoryUtils {

    /**
     * Return deep copy of inputMatrix
     * @param inputMatrix
     * @return
     */
    public static List<List<BigFraction>> copyMatrix(List<List<BigFraction>> inputMatrix) {
        List<List<BigFraction>> result = new ArrayList<>(inputMatrix.size());
        inputMatrix.forEach(row -> result.add(new ArrayList<>(row)));
        return result;
    }

}
