��          T      �       �      �      �   `  �   �   K  C    p   W  �   �     �     �    �  4     I  :  \   �                                        # Exercise 1: My First Macro ## What are Macros? * [ ] Read this file to understand the theory being tested, and what
      task you will be asked to complete.
* [ ] Try and complete the `main.rs` file.
* [ ] Test to see if your macro creates the same code we have, using
      `cargo run -- test 01_my_first_macro`.
* [ ] Run your code, using `cargo run --bin 01_my_first_macro`, to see what it does. Rust's macros are a way of using code to generate code before compilation.
Because the generation happens before the compiler does anything, you are given
much more flexibility in what you can write. This allows you to break many of the syntax rules Rust imposes on you. For
example, Rust does not allow "variadic" functions: functions with variable
numbers of arguments. This makes a `println` function impossible -- it would
have to take any number of arguments (`println("hello")` and `println("{}",
123)`, for example). Welcome to this introduction to Rust's Macro system.
To complete each exercise (including this one), you should: Project-Id-Version: MacroKata
PO-Revision-Date: 
Language-Team: 
MIME-Version: 1.0
Content-Type: text/plain; charset=UTF-8
Content-Transfer-Encoding: 8bit
Plural-Forms: nplurals=1; plural=0;
X-Generator: Poedit 2.3
Last-Translator: 
Language: zh
 练习一: 第一个宏 ## 什么是宏 * [ ] 阅读每个练习的README学习其中的理论，并完成每个README最后留下的任务
* [ ] 在`main.rs`中完成README中的任务
* [ ] 运行`cargo run -- test 01_my_first_macro`来测试宏展开的结果
* [ ] 运行你的代码`cargo run --bin 01_my_first_macro` Rust宏是在编译前生成代码的一种方式。 因为代码生成发生在编译之前，所以宏可以让你获得相当的灵活性。宏可以让你打破Rust强加于你的一些语法规则。例如，Rust并不允许可变个数参数数量的函数，这就使`println`这样需要不定个参数的函数无法实现，例如`println("hello")` and `println("{}",123)` 欢迎来到这个对Rust宏系统的讲解项目。
完成项目中的练习，你需要： 
