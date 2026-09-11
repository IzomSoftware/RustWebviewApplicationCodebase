# THIS FILE IS AUTO-GENERATED. DO NOT MODIFY!!

# Copyright 2020-2023 Tauri Programme within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

-keep class net.izom.rust_webview_application_codebase.* {
  native <methods>;
}

-keep class net.izom.rust_webview_application_codebase.WryActivity {
  public <init>(...);

  void setWebView(net.izom.rust_webview_application_codebase.RustWebView);
  java.lang.Class getAppClass(...);
  java.lang.String getVersion();
}

-keep class net.izom.rust_webview_application_codebase.Ipc {
  public <init>(...);

  @android.webkit.JavascriptInterface public <methods>;
}

-keep class net.izom.rust_webview_application_codebase.RustWebView {
  public <init>(...);

  void loadUrlMainThread(...);
  void loadHTMLMainThread(...);
  void setAutoPlay(...);
  void setUserAgent(...);
  void evalScript(...);
}

-keep class net.izom.rust_webview_application_codebase.RustWebChromeClient,net.izom.rust_webview_application_codebase.RustWebViewClient {
  public <init>(...);
}