# Plinth Core
This crate is part of my Plinth stack (See example at [Plinth-Hello-World](https://github.com/gusjengis/Plinth-Hello-World).)

This is a fork of [Foxicution](https://github.com/Foxicution)'s (wgpu-template)(https://github.com/Foxicution/wgpu-template).

What I've added is a couple traits called PlinthApp and PlinthRendere with a bunch of functions that the template app now calls at the appropriate times.

This allows you to create your own struct that implements PlinthApp and write code that gets executed during init or when events fire or during rendering without having to look at the complex setup code.

Just a nice abstraction that allows me to reuse the same setup without having to pollute each project with the messy winit + wgpu setup code.
