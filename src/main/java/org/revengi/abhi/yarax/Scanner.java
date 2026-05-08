package org.revengi.abhi.yarax;

public class Scanner implements AutoCloseable {
    private long nativePtr;

    /**
     * Creates a new scanner with the given compiled rules.
     *
     * @param rules compiled YARA rules
     * @throws IllegalArgumentException if rules is null
     * @throws RuntimeException if scanner creation fails
     */
    public Scanner(Rules rules) {
        if (rules == null) {
            throw new IllegalArgumentException("Rules cannot be null");
        }
        nativePtr = create(rules.getNativePtr());
        if (nativePtr == 0) {
            throw new RuntimeException("Failed to create scanner");
        }
    }

    /**
     * Sets the timeout for scan operations.
     * Once the timeout is reached, scanning will stop and return a timeout error.
     *
     * @param seconds timeout in seconds (0 for no timeout)
     * @throws IllegalStateException if scanner is closed
     */
    public String setTimeout(int seconds) {
        if (nativePtr == 0) {
            throw new IllegalStateException("Scanner is closed");
        }
        if (seconds < 0) {
            throw new IllegalArgumentException("Timeout must be non-negative");
        }
        return setTimeout(nativePtr, seconds);
    }

    /**
     * Sets the maximum number of matches per pattern.
     * This can prevent performance issues with patterns that match too many times.
     *
     * @param maxMatches maximum matches per pattern (0 for unlimited)
     * @throws IllegalStateException if scanner is closed
     */
    public String setMaxMatchesPerPattern(int maxMatches) {
        if (nativePtr == 0) {
            throw new IllegalStateException("Scanner is closed");
        }
        if (maxMatches < 0) {
            throw new IllegalArgumentException("Max matches must be non-negative");
        }
        return setMaxMatchesPerPattern(nativePtr, maxMatches);
    }

    /**
     * Sets a global variable value for scanning.
     * This can override the default value defined during compilation.
     *
     * @param name variable name
     * @param value variable value (as string)
     * @return null if successful, error string if failed
     * @throws IllegalStateException if scanner is closed
     * @throws IllegalArgumentException if name or value is null
     */
    public String setGlobal(String name, String value) {
        if (nativePtr == 0) {
            throw new IllegalStateException("Scanner is closed");
        }
        if (name == null) {
            throw new IllegalArgumentException("Variable name cannot be null");
        }
        if (value == null) {
            throw new IllegalArgumentException("Variable value cannot be null");
        }
        return setGlobal(nativePtr, name, value);
    }

    /**
     * Configures whether the scanner uses memory-mapped files.
     * Memory-mapped files can be faster for large files but may be less safe.
     *
     * @param enabled true to enable memory mapping, false to disable
     * @throws IllegalStateException if scanner is closed
     */
    public String useMmap(boolean enabled) {
        if (nativePtr == 0) {
            throw new IllegalStateException("Scanner is closed");
        }
        return useMmap(nativePtr, enabled);
    }

    /**
     * Scans the given byte array for YARA rule matches.
     *
     * @param data data to scan
     * @return JSON string containing scan results
     * @throws IllegalStateException if scanner is closed
     * @throws IllegalArgumentException if data is null
     */
    public String scanBytes(byte[] data) {
        if (nativePtr == 0) {
            throw new IllegalStateException("Scanner is closed");
        }
        if (data == null) {
            throw new IllegalArgumentException("Data cannot be null");
        }
        return scanBytes(nativePtr, data);
    }

    /**
     * Scans the file at the given path for YARA rule matches.
     *
     * @param path file path to scan
     * @return JSON string containing scan results
     * @throws IllegalStateException if scanner is closed
     * @throws IllegalArgumentException if path is null
     */
    public String scanFile(String path) {
        if (nativePtr == 0) {
            throw new IllegalStateException("Scanner is closed");
        }
        if (path == null) {
            throw new IllegalArgumentException("Path cannot be null");
        }
        return scanFile(nativePtr, path);
    }

    /**
     * Closes the scanner and releases native resources.
     * The scanner cannot be used after calling close().
     */
    public void close() {
        if (nativePtr != 0) {
            destroy(nativePtr);
            nativePtr = 0;
        }
    }

    /**
     * Checks if the scanner is still valid (not closed).
     *
     * @return true if scanner is open, false if closed
     */
    public boolean isOpen() {
        return nativePtr != 0;
    }

    /**
     * Gets the native pointer for internal use.
     *
     * @return native pointer
     */
    long getNativePtr() {
        return nativePtr;
    }

    // Native methods
    private static native long create(long rulesPtr);
    private static native String setTimeout(long ptr, int seconds);
    private static native String setMaxMatchesPerPattern(long ptr, int maxMatches);
    private static native String setGlobal(long ptr, String name, String value);
    private static native String useMmap(long ptr, boolean enabled);
    private static native String scanBytes(long ptr, byte[] data);
    private static native String scanFile(long ptr, String path);
    private static native void destroy(long ptr);

    static {
        System.loadLibrary("yarax_android");
    }
}
