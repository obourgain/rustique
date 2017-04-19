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
cd devoxx
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
concept: variable et immutabilité et inférence de type
speaker: olivier
text: une binding s'écrit comme ça, une binding mutable s'écrit comme ça.
mot clé let et inférence de type
code:
```rust
fn main() {
 let age = 42;
 println!("mon age c'est {}", age);
 age = 29;
 //error
 //let mut and it's works
}
```
---------------------------------
concept: ownership
speaker: jbp
text: concept fondamental, ownership, pourquoi, pour permettre au compilo de faire la gestion automatique des ressources sans GC
code:
```rust
fn main() {
   let devoxx_editions = vec!(2016, 2017);
   print_release_date(devoxx_editions);
   println!("Devoxx appears in {:?}", devoxx_editions); // ajouter ce print après 
}
fn print_release_date(date: Vec<i32>) {
   println!("Devoxx appears in {:?}", date)
}
//error on passe la main au suivant pour le concept de borrowing
```
---------------------------------
concept: borrowing
speaker: olivier
text: concept fondamental, borrowing, pourquoi, pour permettre de réutiliser les variables....
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
---------------------------------
concept: struct impl pour struct
speaker: jbp
text: c'est la manière de structurer des champs (comme des classes en java, mais sans comportement) impl pour ajouter du comportement sur la struct.
code:
```rust
fn main() {
   let mut foo = Foo::new(42);
   let bar = Bar { foo: foo };
   foo.get_value();
   foo.set_value(32);
}

struct Foo {
    value: i32,
}

impl Foo {
    fn new(value: i32) -> Foo { // syntaxe avec return
        Foo{value: value}
    }
    
    fn get_value(&self) -> i32 {
        self.value
    }
    
    fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}

struct Bar {
    foo: Foo,
}
```
---------------------------------
concept: trait, impl et derive
speaker: olivier
text: Donne du comportement et permet d’abstraire les types, comme les interfaces java, mais peut être implémenté a posteriori et même sur des types fournis par des librairies.
L’attribut derive permet de générer directement du comportement sans avoir à écrire l’implémentation, par exemple pour Debug.
code:

```rust
use std::fmt::*;
impl Debug for Foo {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Foo {{ val: {} }}", self.value)
    }
}
```

puis 
```rust
#[derive(Debug)]
```

---------------------------------
concept: lifetime
speaker: jbp
text: ça donne des informations (contraintes) au compilo pour savoir quand libérer la mémoire, déclaré comme un type générique,
le nom est arbitraire, il faut une apostrophe
code:
```rust
fn main() {
   let foo = Foo { value: 42 };
   let bar = Bar { foo: &foo };
   println!("{:?}", bar)
}

#[derive(Debug)]
struct Foo {
   value: i32,
}

#[derive(Debug)]
struct Bar<'b> {
   foo: &'b Foo,
}
```
---------------------------------
concept: Drop
speaker: olivier
text: le compilateur sait quand libérer les ressources et on peut lui demander de faire autre chose que libérer la mémoire

```rust
impl Drop for Foo {
   fn drop (&mut self) {
       println!(" drop of {:?}", self);
   }
}

impl <'b>Drop for Bar<'b> {
   fn drop (&mut self) {
       println!("when dropping me, Foo is still alive says {:?}", self);
   }
}

fn main() {
    do_stuff();
    println!("end of main()");
}

fn do_stuff() {
   let foo = Foo { value: 42 };
   let bar = Bar { foo: &foo };
   println!("hello from alive {:?}", bar);
   println!("end of do_stuff()");
}
```

---------------------------------
concept: Gestion d’erreurs
speaker: jbp
text: pas d’exception, gestion des erreurs par un type qui contient soit le résultat soit une erreur, et on utilise le pattern matching dessus
```rust
use std::fs::File;

fn main() {
   let my_file = File::open("/tmp/foo");

   match my_file {
       Ok(file) => file,
       Err(err) => panic!(" error {}", err)
   };
}
```

---------------------------------
concept: Prog fonctionnelle
speaker: olivier
text: closure et api iterator semblable aux streams de java 8
code: 
```rust
fn main() {
   let add = |x: i32, y:i32 | x + y;
   println!("{}", add(5, 5));

   for x in 0..5 {
       println!("{}", x)
   }

   let dates_devoxx = vec!(2015, 2016);
   let new_dates: Vec<i32> = dates_devoxx.iter()
                                         .map(|date| date + 1)
                                         .collect();

   println!("{:?}", new_dates);
}
```

---------------------------------
concept: Stack & heap
speaker: jbp
text: pour le moment tout était alloué sur la stack, mais on doit parfois allouer sur la heap, quand le cycle de vie des données n’est pas le même que celui de la fonction ou trop gros pour la taille de la stack. Mettre sur la heap est explicite via Box ou d’autres types
code: 
```rust
fn main() {
  let boxed = Box::new(42);
}
```

---------------------------------
concept: String
speaker: olivier
text: Rust a 2 types de strings, la première est String, une chaine de caractère, mais comme on a le ownership en Rust, on peut garantir qu'on est le seul à avoir une ref dessus, et donc c'est mutable si le binding ou le borrow est mut.
Si on borrow une String, on a un &String, qui est immutable. Mais toujours grace au borrow checker, on peut savoir à la compilation s'il y a des borrows ou pas, donc on peut optimiser.
Le 2eme type de string est str, ou plutôt &str, la string slice, qui représente une vue sur une String immutable.
code: 
```rust
```


Conclusion
===

rust c’est stylé
encore jeune mais très prometeur
