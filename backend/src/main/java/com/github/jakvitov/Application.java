package com.github.jakvitov;

import io.micronaut.runtime.Micronaut;
import lombok.extern.slf4j.Slf4j;

import java.awt.*;
import java.net.URI;

@Slf4j
public class Application {

    private final static String APPLICATION_URL = "http://localhost:8080/";

    public static void main(String[] args) {
        Micronaut.run(Application.class, args);
        startBrowserWindowOrShowRef();
    }

    private static void startBrowserWindowOrShowRef() {
        try {
            if (Desktop.isDesktopSupported()) {
                Desktop desktop = Desktop.getDesktop();
                desktop.browse(new URI(APPLICATION_URL));
            }
        } catch (Exception ignored) {}
        log.info("Simpler successfully started up and is available at: " + APPLICATION_URL);
    }
}
