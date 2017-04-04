Intro

Who we are
gonna talk about Rust
on ne va pas tout voir

-- jbp speaker
cargo new --bin
on a créé le projet avant et téléchargé les dépendances, le premier build peut être long

main()
println! "hello devoxx".to_string()
macro

function &str pour mettre le println!

passer le message en param par valeur => erreur de compil
passage en ref => borrowing/ownership

-- obmg speaker

coder les structs
struct Speaker
derive de debug
println!
impl new
Speaker::new
println de mon objet

struct Talk
vec<Speaker>

-- jbp speaker

Talk borrow Speaker
lifetime 't sur Talk

HashMap de Speakers
mut
HashMap de Talks

-- obmg speaker
routeur avec get sur /
talks.get("rust".to_string())
boom !
il faut retourner du json
derive(Serialize)
get().unwrap().to_json().unwrap().to_string()
problème du lifetime de la map de Talk
on met dans un Box sur la heap
en fait il faut Arc
move closure pour qu'elle prenne ownership du Arc
I like to move it move it !


-- jb speaker

extraction d'une méthode hello_devoxx() appelée par la closure
remplacement de Arc par & dans la signature et l'appel pour simplifier la signature et code moins lié
match sur le json pour montrer la gestion des erreurs

Conclusion
ownership & borrowing
protège de bcp d'erreurs à la compilation
bas et haut niveau
