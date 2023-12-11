//
//  ContentView.swift
//  HelloRust
//
//  Created by Dimitrios Brukakis on 10.12.23.
//

import SwiftUI

struct ContentView: View {

    @State var html: String = ""

    var body: some View {
        VStack {
            WebView(html: html)
                .background(Color.white)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .task {
            testMarkdown()
        }
    }

    var bodyx: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Hello, world!")
            Button(action: { helloFromRust() }, label: {
                Text("Hello from Rust!")
            })
            Button(action: { testMarkdown() }, label: {
                Text("Test Markdown!")
            })
        }
        .padding()
    }

    func helloFromRust() {
        hello_from_rust()
    }

    func testMarkdown() {
        let markdown = Markdown()
        html = markdown.testMarkdown()
    }
}

#Preview {
    ContentView()
}
