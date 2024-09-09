package com.cesarvaliente.rustandroid

import android.os.Bundle
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import com.cesarvaliente.rustandroid.databinding.ActivityMainBinding


class MainActivity : AppCompatActivity(), JNICallback {

    // Used to load the 'rust' library on application startup.
    init {
        System.loadLibrary("rust")
    }

    private lateinit var binding : ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        invokeCallbackFromAppInit(this)
        binding.button.setOnClickListener {
            invokeCallbackViaButton(this)
        }
    }

    /**
     * A native method that is implemented by the 'rust' native library,
     * which is packaged with this application.
     */
    private external fun invokeCallbackFromAppInit(callback: JNICallback?)
    private external fun invokeCallbackViaButton(callback: JNICallback?)

    override fun callbackFromInit(string: String) {
        binding.textview.append("I have something to tell you:\n\n $string")
        binding.textview.append("\n\n ... and now try the following button...")
    }
    override fun callbackFromButton(string: String) {
        val builder = AlertDialog.Builder(this);
        builder.setTitle("Hey from Rust!")
        builder.setMessage(string)
        builder.setPositiveButton("close") { dialog, _ ->
            dialog.cancel()
        }
        builder.create().show()
    }
}

interface JNICallback {
    fun callbackFromInit(string: String)
    fun callbackFromButton(string: String)
}
