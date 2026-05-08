package org.revengi.abhi.yarax;

public class Compiler {
    private long nativePtr;

    public Compiler() {
        nativePtr = create();
        if (nativePtr == 0) {
            throw new RuntimeException("Failed to create compiler");
        }
    }

    /**
     * Adds YARA source code to the compiler.
     *
     * @param source YARA rule source code
     * @return null if successful, error string if failed
     * @throws IllegalStateException if compiler is closed
     */
    public String addSource(String source) {
        if (nativePtr == 0) {
            throw new IllegalStateException("Compiler is closed");
        }
        if (source == null) {
            throw new IllegalArgumentException("Source cannot be null");
        }
        return addSource(nativePtr, source);
    }

    /**
     * Creates a new namespace for subsequent rules.
     *
     * @param namespace the namespace name
     * @throws IllegalStateException if compiler is closed
     * @throws IllegalArgumentException if namespace is null
     */
    public void newNamespace(String namespace) {
        if (nativePtr == 0) {
            throw new IllegalStateException("Compiler is closed");
        }
        if (namespace == null) {
            throw new IllegalArgumentException("Namespace cannot be null");
        }
        newNamespace(nativePtr, namespace);
    }

    /**
     * Defines a global variable that can be used in YARA rules.
     *
     * @param name variable name
     * @param value variable value (as string)
     * @return null if successful, error string if failed
     * @throws IllegalStateException if compiler is closed
     * @throws IllegalArgumentException if name or value is null
     */
    public String defineGlobal(String name, String value) {
        if (nativePtr == 0) {
            throw new IllegalStateException("Compiler is closed");
        }
        if (name == null) {
            throw new IllegalArgumentException("Variable name cannot be null");
        }
        if (value == null) {
            throw new IllegalArgumentException("Variable value cannot be null");
        }
        return defineGlobal(nativePtr, name, value);
    }

    /**
     * Builds the compiled YARA rules from all added source code.
     * This consumes the compiler - it cannot be used after calling build().
     *
     * @return compiled Rules object
     * @throws IllegalStateException if compiler is closed
     * @throws RuntimeException if build fails
     */
    public Rules build() {
        if (nativePtr == 0) {
            throw new IllegalStateException("Compiler is closed");
        }
        long rulesPtr = build(nativePtr);
        nativePtr = 0;
        if (rulesPtr == 0) {
            throw new RuntimeException("Failed to build rules");
        }
        return new Rules(rulesPtr);
    }

    /**
     * Closes the compiler and releases native resources.
     * The compiler cannot be used after calling close().
     */
    public void close() {
        if (nativePtr != 0) {
            destroy(nativePtr);
            nativePtr = 0;
        }
    }

    /**
     * Checks if the compiler is still valid (not closed).
     *
     * @return true if compiler is open, false if closed
     */
    public boolean isOpen() {
        return nativePtr != 0;
    }

    // Native methods
    private static native long create();
    private static native String addSource(long ptr, String source);
    private static native long newNamespace(long ptr, String namespace);
    private static native String defineGlobal(long ptr, String name, String value);
    private static native long build(long ptr);
    private static native void destroy(long ptr);

    static {
        System.loadLibrary("yarax_android");
    }
}
