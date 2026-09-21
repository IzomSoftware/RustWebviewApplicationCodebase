# Rust Webview Application Codebase

## This application codebase is made in Rust with a simple purpose:
- Simplifying everything an app would need to provide a **webview based UI**

## The project is focusing on:
- being minimal
- being the jack of all trades
- being modular

## Why not just use cargo mobile so that I can code more easily?
- The main reason that I made this project was to abandon cargo-mobile2. the reason for that is because it generates the android/IOS code over and over again instead of letting you change the code. the problem with this appraoch is the fact that Android is a pain in ass to work with. It requires OS calls. Making those calls should be done cleanly via Kotlin, Thus, I made this project so I'll be able to make changes to my app's Kotlin code, without worrying about it generating again.

## Why there's no IOS code?
- Becausew I don't use an IOS device!