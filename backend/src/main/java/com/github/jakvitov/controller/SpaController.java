package com.github.jakvitov.controller;

import io.micronaut.http.MediaType;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Get;
import io.micronaut.http.server.types.files.StreamedFile;
import io.swagger.v3.oas.annotations.Hidden;

import java.io.InputStream;

@Controller
public class SpaController {

    @Get(uri = "/{path:^(?!.*\\.(css|js|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot|json|map)).*$}", produces = MediaType.TEXT_HTML)
    @Hidden
    public StreamedFile index(String path) {
        InputStream stream = getClass()
                .getResourceAsStream("/public/index.html");
        return new StreamedFile(stream, MediaType.TEXT_HTML_TYPE);
    }
}
