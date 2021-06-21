package fr.orion78.keepassTotp.androidPlugin;

import android.app.Activity;
import android.content.Intent;
import android.net.Uri;
import android.util.Log;

import androidx.annotation.NonNull;

import org.godotengine.godot.Dictionary;
import org.godotengine.godot.Godot;
import org.godotengine.godot.plugin.GodotPlugin;
import org.godotengine.godot.plugin.UsedByGodot;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;

public class FileOpenerPlugin extends GodotPlugin {
    private static final int REQUEST_RESULT_GET_FILE = 4269;
    private static final String FILE_FOUND_SIGNAL_NAME = "fileFound";

    private CompletableFuture<Uri> future = new CompletableFuture<>();

    public FileOpenerPlugin(Godot godot) {
        super(godot);
    }

    @NonNull
    @Override
    public String getPluginName() {
        return "Android File Opener Plugin";
    }

    @NonNull
    @UsedByGodot
    public String getTheHelloWorld() {
        return "Hello World from Android Plugin !";
    }

    public Dictionary test() {
        Dictionary d = new Dictionary();
        d.put("JNIEnv", );
    }

    @NonNull
    @UsedByGodot
    public byte[] getKeepassFile() throws IOException {
        Uri uri = openFile();

        return readFile(uri);
    }

    @NonNull
    private byte[] readFile(Uri uri) throws IOException {
        byte[] buf = new byte[1024];
        try (InputStream is = getActivity().getContentResolver().openInputStream(uri)) {
            ByteArrayOutputStream bb = new ByteArrayOutputStream();
            int readBytes;
            while ((readBytes = is.read(buf, 0, 1024)) > 0) {
                bb.write(buf, 0, readBytes);
            }
            return bb.toByteArray();
        }
    }


    private Uri openFile() {
        Intent intent = new Intent(Intent.ACTION_OPEN_DOCUMENT);
        intent.addCategory(Intent.CATEGORY_OPENABLE);
        intent.setType("*/*");

        Log.e("File Opener Plugin", "Starting a file open request with result " + REQUEST_RESULT_GET_FILE);
        getActivity().startActivityForResult(intent, REQUEST_RESULT_GET_FILE);

        try {
            return future.get();
        } catch (ExecutionException | InterruptedException e) {
            Log.e("File Opener Plugin", "Error while getting ", e);
            throw new RuntimeException(e);
        }
    }

    @Override
    public void onMainActivityResult(int requestCode, int resultCode, Intent data) {
        super.onMainActivityResult(requestCode, resultCode, data);
        if (requestCode == REQUEST_RESULT_GET_FILE && resultCode == Activity.RESULT_OK) {
            Log.e("File Opener Plugin", "Got a file ! " + data.getData().toString());

            future.complete(data.getData());
        }
    }
}
