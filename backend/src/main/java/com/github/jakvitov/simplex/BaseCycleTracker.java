package com.github.jakvitov.simplex;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;

/**
 * Tracker, that keeps track of visited bases and provides interface to decide, if the limit was reached
 */
public class BaseCycleTracker {

    private HashMap<List<String>, Integer> baseVisits;
    private final long limit;

    public BaseCycleTracker(long limit) {
        this.limit = limit;
        baseVisits = new HashMap<>();
    }

    public void visited(List<String> base) {
        List<String> copy = new ArrayList<>(base);
        copy.sort(Comparator.naturalOrder());
        baseVisits.put(copy, baseVisits.getOrDefault(copy, 0) + 1);
    }

    public boolean limitReached(List<String> base) {
        List<String> copy = new ArrayList<>(base);
        copy.sort(Comparator.naturalOrder());
        return baseVisits.getOrDefault(copy, 0) > this.limit;
    }


}
