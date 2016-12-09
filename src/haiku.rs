extern crate rand;
use rand::Rng;

static FALLBACK_HAIKU: [&'static str; 3] = ["Could not find", "A proper haiku", "So sorry friend"];

static HAIKUS: [[&'static str; 3]; 53] =
    [["In a dream, it came", "The pickle farmer's great hat", "It emcompassed all"],
     ["Roberto resigns", "His seat vacant, Rabid war", "begins for the throne"],
     ["Rob's heir is most clear", "His will, in absentia:", "My reign will begin"],
     ["all competitors", "swiftly dealt to; syllabic", "errors found with glee"],
     ["Foul knaves assault me", "A queen the throne demands now", "Only one can be"],
     ["Claim unrescinded", "Smack talking haiku battle", "I'll end false regents"],
     ["Haiku is easy", "syllable-based poetry", "Five seven then five"],
     ["Tower of Babel:", "Phonegap plus framework plus grunt", "Ends in avalanche"],
     ["Today is enjoy", "Tomorrow will be better", "Then I fly away"],
     ["excuse of illness", "tomorrow excuse of death", "then there's no excuse"],
     ["poor wee roberto", "do you know how syllables work?", "haiku master no more"],
     ["Sad melancholy", "A new face met, and then lost", "Washed clean, a fresh slate"],
     ["Poetry is fun", "Let us save the magic here", "Beautiful artwork"],
     ["their software libre", "coders quiver with caffeine", "quietly tapping keys"],
     ["There is nought more sad", "Than haiku in disarray", "So arrange yourselves"],
     ["long-awaited rain", "sad timing on post-chalk day", "haiku, moustache, gone"],
     ["I like how the word", "'telecommunications'", "fills out this haiku"],
     ["darkest chocolate", "frozen vanilla ice cream", "affogato dream"],
     ["look behind you, rob", "hitchcock-like, typing usurped", "you thought you were safe"],
     ["beware it's a cat", "his bristly whiskers attack", "without a respite"],
     ["Great job team Rabid!", "Bringing haiku to Railscamp", "Haters gonna hate"],
     ["outside, afternoon:", "decagonal frisbee time", "bromance abounded"],
     ["lachlan's pain right now:", "styling ephemeral things", "life, CSS, hurt"],
     ["matt loves his ice cream", "coffeescript affogato:", "a tasty dessert"],
     ["Recursive Haiku", "You know recursive Haiku?", "Go back to line one"],
     ["A simple request", "Please don't kill me so quickly", "I'm not a werewolf"],
     ["Welcome! Home again!", "Rabid does what we do best;", "Nerf wars are a go"],
     ["I want a smoothie;", "I need it in my body;", "Give it to me now"],
     ["Oh man, computers!", "I was like, how do they work?", "This stuff is silly"],
     ["Walking down the street,", "Can't contain my haiku glee!", "Walk into a bush"],
     ["eat cake every day", "it's not very healthy but", "life is very long"],
     ["Didn't know that cats snore", "Merrin would have a field day", "With my flat of cats"],
     ["Breccan in cream pants?", "A first; the omen is clear,", "Apocalypse now"],
     ["Cats cats, cats cats cats",
      "Cats! Cats, cats cats? Cats cats cats;",
      "Cats, cats, cats cats cats!"],
     ["To join our great team,", "You must show skill at haiku", "and perhaps coding"],
     ["She is a poet,", "I think that she may know it", "Please alert the press"],
     ["forlorn ex-apple", "lonely core in coffee cup", "as autumn sun shines"],
     ["when will lunch time be?", "i crave tasty sustenance", "and beer in the sun"],
     ["should haiku index", "showcase new leaves at the top", "like noble oak trees"],
     ["This isn't a test,", "This is actually life", "Get used to it fast"],
     ["railscamp afternoon,", "leaf shades dance on dusty glass;", "hangover study"],
     ["Mat with just one 't'", "A game: Dogsteroids Pew pew!", "Goodbye, sweet doggies"],
     ["The Haiku Master,", "smiles as new Haiku appear", "Best app at Railscamp?"],
     ["I suspect Breccan", "Fellow villagers listen,", "for haikus don't lie"],
     ["I believe in love;", "Do you believe in magic?", "Science reigns supreme"],
     ["Hsoj, hsoj, come", "Hsoj, hsoj, hsoj, here", "Hsoj, hsoj, now"],
     ["A pull request sent", "Listening for a response", "The cavern echoes"],
     ["my hands are very cold", "intermittently i place", "them under my butt"],
     ["Warmth radiating", "In this finger numbing world,", "it is the small things"],
     ["mmm coffee coffee,", "mmm coffee coffee coffee,", "I need more coffee"],
     ["A bubble appeared", "Long-lived but ephemeral ", "Traversed the lunch room"],
     ["Merrin and Matt", "Dead to us but still they live", "on in our tiny dreams"],
     ["The bubble had dreams", "To ascend into heaven", "But settled for less"]];

#[derive(Debug)]
#[derive(RustcEncodable)]
pub struct Haiku<'a> {
    pub lines: &'a [&'a str; 3],
}

impl<'a> Haiku<'a> {
    pub fn choose_random() -> Self {
        let lines = match rand::thread_rng().choose(&HAIKUS) {
            Some(lines) => lines,
            None => &FALLBACK_HAIKU,
        };
        Haiku { lines: lines }
    }
}
