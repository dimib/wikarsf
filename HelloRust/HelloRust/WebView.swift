//
//  WebView.swift
//  HelloRust
//
//  Created by Dimitrios Brukakis on 11.12.23.
//

import Foundation
import SwiftUI
import WebKit

struct WebView: UIViewRepresentable {
    // 1
    let html: String?

    // 2
    func makeUIView(context: Context) -> WKWebView {
        return WKWebView()
    }

    // 3
    func updateUIView(_ webView: WKWebView, context: Context) {
        guard let html else { return }

        webView.loadHTMLString(html, baseURL: nil)
    }
}
