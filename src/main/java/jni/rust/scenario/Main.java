package jni.rust.scenario;

import io.questdb.jar.jni.JarJniLoader;

public class Main {
    static {
        JarJniLoader.loadLib(
            Main.class,
            "/jni/rust/scenario/libs",
            "jni_example"
        );
    }

    public static void main(String[] args) {
        try (var obj = new TestObj()) {
            obj.invoke();
        }
    }
}
