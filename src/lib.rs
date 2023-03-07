use rand::Rng;

//create an const array of the top 10 best universities of the US
pub const MOVIES: [&str; 10] = [
    "Princeton University",
    "Harvard University",
    "Columbia University",
    "Massachusetts Institute of Technology (MIT)",
    "Yale University",
    "Stanford University",
    "University of Chicago",
    "California Institute of Technology (Caltech)",
    "University of Pennsylvania",
    "Duke University",
];

//create a function that returns a random university in the list above
pub fn random_movie() -> &'static str {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..MOVIES.len());
    MOVIES[random_index]
}
