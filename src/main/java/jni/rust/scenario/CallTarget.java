package jni.rust.scenario;

public class CallTarget {
    public static void raiseExc() {
        throw new RuntimeException("or maybe not..");
    }

    public static void log() {
        System.err.println("logging from Java, as called from Rust");
    }
}
