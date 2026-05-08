package org.revengi.abhi.yarax;

public class Rules {
    private long nativePtr;

    Rules(long nativePtr) {
        this.nativePtr = nativePtr;
    }

    /**
     * Serializes the compiled rules to a byte array.
     * The serialized rules can be stored and later deserialized.
     *
     * @return byte array containing serialized rules
     * @throws IllegalStateException if rules are closed
     * @throws RuntimeException if serialization fails
     */
    public byte[] serialize() {
        if (nativePtr == 0) {
            throw new IllegalStateException("Rules are closed");
        }
        byte[] result = serialize(nativePtr);
        if (result == null) {
            throw new RuntimeException("Failed to serialize rules");
        }
        return result;
    }

    /**
     * Deserializes rules from a byte array.
     *
     * @param bytes byte array containing serialized rules
     * @return Rules object
     * @throws IllegalArgumentException if bytes is null
     * @throws RuntimeException if deserialization fails
     */
    public static Rules deserialize(byte[] bytes) {
        if (bytes == null) {
            throw new IllegalArgumentException("Bytes cannot be null");
        }
        long rulesPtr = deserializeFromBytes(bytes);
        if (rulesPtr == 0) {
            throw new RuntimeException("Failed to deserialize rules");
        }
        return new Rules(rulesPtr);
    }

    /**
     * Closes the rules and releases native resources.
     * The rules cannot be used after calling close().
     */
    public void close() {
        if (nativePtr != 0) {
            destroy(nativePtr);
            nativePtr = 0;
        }
    }

    /**
     * Checks if the rules are still valid (not closed).
     *
     * @return true if rules are open, false if closed
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
    private static native byte[] serialize(long ptr);
    private static native long deserializeFromBytes(byte[] bytes);
    private static native void destroy(long ptr);

    static {
        System.loadLibrary("yarax_android");
    }
}
