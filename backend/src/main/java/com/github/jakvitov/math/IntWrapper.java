package com.github.jakvitov.math;

public class IntWrapper {
    public int value;

    public static IntWrapper of(int value) {
        IntWrapper wrapper = new IntWrapper();
        wrapper.value = value;
        return wrapper;
    }

}
