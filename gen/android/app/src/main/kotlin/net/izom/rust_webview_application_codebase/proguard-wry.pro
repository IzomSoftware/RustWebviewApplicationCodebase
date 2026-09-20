# THIS FILE IS AUTO-GENERATED. DO NOT MODIFY!!
#
# ProGuard/R8 rules for the wry 0.55.x Android glue.
#
# The glue classes are reached by name from:
#   1. JNI (Rust -> Kotlin): getId, startActivity, getAppClass, setWebView,
#      loadUrlMainThread, evalScript, currentUrl, postMessage, constructors, ...
#   2. The WebView JS bridge (@JavascriptInterface Ipc.postMessage)
#   3. The native library via Java_<pkg>_<Class>_<method> symbol lookups
# Renaming or removing anything in this package breaks the app at runtime,
# so keep every member of every glue class.

-keep class net.izom.rust_webview_application_codebase.** { *; }

# JNI-called members must keep their exact names even in more aggressive setups
-keepclasseswithmembernames class net.izom.rust_webview_application_codebase.** {
    native <methods>;
}