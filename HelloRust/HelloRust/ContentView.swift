//
//  ContentView.swift
//  HelloRust
//
//  Created by Dimitrios Brukakis on 10.12.23.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Hello, world!")
            Button(action: {
                helloFromRust()
            }, label: {
                Text("Hello from Rust!")
            })
        }
        .padding()
    }

    func helloFromRust() {
        hello_from_rust()
    }
}

#Preview {
    ContentView()
}
