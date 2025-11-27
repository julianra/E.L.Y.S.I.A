package com.example.orbit

import android.content.Context
import android.net.wifi.WifiManager
import android.os.Bundle
import io.flutter.embedding.android.FlutterActivity

class MainActivity : FlutterActivity() {

    private var multicastLock: WifiManager.MulticastLock? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Acquire multicast lock required for mDNS discovery
        val wifi = applicationContext.getSystemService(Context.WIFI_SERVICE) as WifiManager
        multicastLock = wifi.createMulticastLock("elysia_mdns_lock")
        multicastLock?.setReferenceCounted(true)
        multicastLock?.acquire()
    }

    override fun onDestroy() {
        super.onDestroy()
        multicastLock?.release()
    }
}
