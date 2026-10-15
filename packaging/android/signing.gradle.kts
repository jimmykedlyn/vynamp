import com.android.build.api.dsl.ApplicationExtension
// Apply from apps/android/app after the pinned Android application plugin is loaded.
extensions.configure<ApplicationExtension> {
  signingConfigs {
    create("vynampRelease") {
      storeFile = file(requireNotNull(System.getenv("ANDROID_KEYSTORE_PATH")))
      storePassword = requireNotNull(System.getenv("ANDROID_STORE_PASSWORD"))
      keyAlias = requireNotNull(System.getenv("ANDROID_KEY_ALIAS"))
      keyPassword = requireNotNull(System.getenv("ANDROID_KEY_PASSWORD"))
    }
  }
  buildTypes.getByName("release").signingConfig = signingConfigs.getByName("vynampRelease")
}
