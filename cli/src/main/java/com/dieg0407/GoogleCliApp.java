package com.dieg0407;

import picocli.CommandLine;

import java.util.concurrent.Callable;

import static java.lang.System.err;
import static java.lang.System.exit;

@CommandLine.Command
public class GoogleCliApp implements Callable<Integer> {
    @Override
    public Integer call() throws Exception {
        err.println("This is the google cli app!");
        return 0;
    }

    public static void main(String[] args) {
        final var exitCode = new CommandLine(new GoogleCliApp()).execute(args);
        exit(exitCode);
    }
}
