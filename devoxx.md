concept:introduction
speaker: olivier
text: Rust language, ce qui est cool (sans GC, abstraction, générique....) la version 1 sortie en mai 2015, ou en production dans firefox, servo
code: None
---------------------------------
concept: main/function/compilation
speaker: jbp
text: cargo new déclaration d'une fonction, création du main, cargo run, println! macro
code:
```bash
cargo new devoxx --bin
cd cargo
cargo run
```

```rust
fn main() {
  hello_devoxx();
}
fn hello_devoxx() {
  println!("hello devoxx");
}
```
---------------------------------
concept: variable et immutabilité
speaker: olivier
text: une variable s'écrit comme ça, une variable mutable s'écrit comme ça.
code:
```rust
fn main() {
  let olivier_age = 42;
  println!("mon age c'est {}", olivier_age);
  olivier_age = 29;
  //error
  //let mut and it's works
}
```
---------------------------------
concept: ownership
speaker: jbp
text: concept fondamentale, ownership, pourquoi, pour permettre au compilo d'ajouter des allocate et des free
code:
```rust
fn main() {
    let devoxx_release_date = vec!(2016, 2017, 42);
    print_release_date(devoxx_release_date);
    println!("Devoxx appears in {:?}", devoxx_release_date)
}
fn print_release_date(date: Vec<i32>) {
    println!("Devoxx appears in {:?}", date)
}
//error on passe la main au suivant pour le concept de borrowing
```
---------------------------------
concept: borrowing
speaker: olivier
text: concept fondamentale, borrowing, pourquoi, pour permettre de réutiliser les variables....
code:
```rust
fn main() {
    let devoxx_release_date = vec!(2016, 2017, 42);
    print_release_date(&devoxx_release_date);
    println!("Devoxx appears also in {:?}", devoxx_release_date)
}
fn print_release_date(date: &Vec<i32>) {
    println!("Devoxx appears in {:?}", date)
}
//works
```