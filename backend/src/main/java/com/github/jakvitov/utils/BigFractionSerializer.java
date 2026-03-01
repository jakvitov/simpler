package com.github.jakvitov.utils;

import io.micronaut.core.annotation.NonNull;
import io.micronaut.core.type.Argument;
import io.micronaut.serde.Decoder;
import io.micronaut.serde.Deserializer;
import io.micronaut.serde.Encoder;
import io.micronaut.serde.Serializer;
import jakarta.inject.Singleton;
import org.hipparchus.fraction.BigFraction;

import java.io.IOException;

@Singleton
public class BigFractionSerializer implements Serializer<BigFraction>, Deserializer<BigFraction> {

    @Override
    public void serialize(@NonNull Encoder encoder, @NonNull EncoderContext context, @NonNull Argument<? extends BigFraction> type, BigFraction value) throws IOException {
        if (value == null) {
            encoder.encodeNull();
            return;
        }
        // Serialize as object (recommended)
        encoder.encodeObject(type);
        encoder.encodeKey("numerator");
        encoder.encodeString(value.getNumerator().abs().toString());

        encoder.encodeKey("denominator");
        encoder.encodeString(value.getDenominator().abs().toString());

        encoder.encodeKey("sign");
        if (value.signum() < 0) {
            encoder.encodeString("N");
        } else {
            encoder.encodeString("P");
        }
        encoder.finishStructure();
    }


    @Override
    public BigFraction deserialize(@NonNull Decoder decoder, @NonNull DecoderContext context, @NonNull Argument<? super BigFraction> type) throws IOException {
        return null;
    }
}

