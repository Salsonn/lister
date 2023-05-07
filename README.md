# Overview

Lister is an application that allows for the selection of one or many items within a terminal interface. Essentially, it allows for intuitive, checkbox-like item selection without the need of a GUI and/or mouse.

The purpose of this program serves the ultimate purpose of facilitating parts of "Redeploy", an application that allows combines the functionality of user data backups with machine imaging to produce a flexible product that can back up, restore, or migrate a user's preferred working or personal desktop environment from one machine to another, or from an old installation of Windows to a newer one.

{Provide a link to your YouTube demonstration. It should be a 4-5 minute demo of the software running and a walkthrough of the code. Focus should be on sharing what you learned about the language syntax.}

[Software Demo Video](https://drive.google.com/file/d/1Rtq6T5CNkeh_ycAwvQho_65QSk_mT4DS/view?usp=share_link)

# Development Environment

To develop this code, I am using the RustC compiler along with the Crates manager, and Visual Studio Code for my IDE. My operating system of choice is Windows 11...for now.

To mimic my setup exactly, you need simply to run the following in a terminal prompt (make sure the Microsoft Store is up to date).

```
winget install Microsoft.VisualStudioCode
winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --quiet --add ProductLang En-us --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
winget install rustup
```
The second line is the most complicated. In a nutshell it automatically installs the necessary packages that `rustup` will attempt to fetch. Sadly the implementation built into the `rustup` install is somewhat lacking. Not only does it still require manual user input, it also installs more packages than are necessary for the `rustc` compiler to function properly.

Note that the VisualStudio BuildTools will take a _while_ to download and install. You might want to set it aside while it does its thing.

This application is developed in Rust, a programming lanugage based on C which emphasizes performance and type safety. It utilized the Crossterm library to implement a proper terminal-based user interface.

# Useful Websites

The following websites

- [TutorialsPoint - Rust](https://www.tutorialspoint.com/rust/)
- [Official Rust Documentation](https://doc.rust-lang.org/)
- [Crossterm Documentation](https://docs.rs/crossterm/latest/crossterm)

# Future Work

{Make a list of things that you need to fix, improve, and add in the future.}

- Fix caret location and highlight coloring inconsistency
- Add proper return to `stdout`
- Fix input crashes