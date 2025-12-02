package jni.rust.scenario;

public class TestObj implements AutoCloseable {
    private long impl;

    private static native long create();
    private static native void invoke(long impl);
    private static native void destroy(long impl);

    public TestObj() {
        impl = create();
    }

    public void invoke() {
        invoke(impl);
    }

    @Override
    public void close() {
        if (impl != 0) {
            destroy(impl);
            impl = 0;
        }
    }
}
