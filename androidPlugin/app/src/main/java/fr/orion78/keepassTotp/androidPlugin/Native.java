package fr.orion78.keepassTotp.androidPlugin;

public class Native {
    static {
        System.loadLibrary("godot_rs_keepass");
    }

    public static native String hello_world(String input);
}
