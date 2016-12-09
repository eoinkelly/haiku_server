extern crate rand;
use rand::Rng;

static FALLBACK_HAIKU: &'static str = "Could not find\nA proper haiku\nSo sorry friend\n";

static HAIKUS: [&'static str; 53] =
    ["In a dream, it came\nThe pickle farmer's great hat\nIt emcompassed all.\n",
     "Roberto resigns.\nHis seat vacant, Rabid war\nbegins for the throne.\n",
     "Rob's heir is most clear\nHis will, in absentia:\nMy reign will begin\n",
     "all competitors\nswiftly dealt to; syllabic\nerrors found with glee.\n",
     "Foul knaves assault me\nA queen the throne demands now\nOnly one can be\n",
     "Claim unrescinded\nSmack talking haiku battle\nI'll end false regents\n",
     "Haiku is easy\nsyllable-based poetry\nFive seven then five\n",
     "Tower of Babel:\nPhonegap plus framework plus grunt\nEnds in avalanche.\n",
     "Today is enjoy.\nTomorrow will be better\nThen I fly away\n",
     "excuse of illness\ntomorrow excuse of death\nthen there's no excuse\n",
     "poor wee roberto\ndo you know how syllables work?\nhaiku master no more\n",
     "Sad melancholy.\nA new face met, and then lost.\nWashed clean, a fresh slate.\n",
     "Poetry is fun\nLet us save the magic here\nBeautiful artwork\n",
     "their software libre\ncoders quiver with caffeine\nquietly tapping keys\n",
     "There is nought more sad\nThan haiku in disarray\nSo arrange yourselves\n",
     "long-awaited rain\nsad timing on post-chalk day\nhaiku, moustache, gone\n",
     "I like how the word\n'telecommunications'\nfills out this haiku\n",
     "darkest chocolate\nfrozen vanilla ice cream\naffogato dream\n",
     "look behind you, rob\nhitchcock-like, typing usurped\nyou thought you were safe\n",
     "beware it's a cat\nhis bristly whiskers attack\nwithout a respite\n",
     "Great job team Rabid!\nBringing haiku to Railscamp.\nHaters gonna hate.\n",
     "outside, afternoon:\ndecagonal frisbee time.\nbromance abounded\n",
     "lachlan's pain right now:\nstyling ephemeral things.\nlife, CSS, hurt.\n",
     "matt loves his ice cream.\ncoffeescript affogato:\na tasty dessert.\n",
     "Recursive Haiku.\nYou know recursive Haiku?\nGo back to line one.\n",
     "A simple request\nPlease don't kill me so quickly\nI'm not a werewolf\n",
     "Welcome! Home again!\nRabid does what we do best;\nNerf wars are a go.\n",
     "I want a smoothie;\nI need it in my body;\nGive it to me now.\n",
     "Oh man, computers!\nI was like, how do they work?\nThis stuff is silly.\n",
     "Walking down the street,\nCan't contain my haiku glee!\nWalk into a bush\n",
     "eat cake every day\nit's not very healthy but\nlife is very long\n",
     "Didn't know that cats snore\nMerrin would have a field day\nWith my flat of cats\n",
     "Breccan in cream pants?\nA first; the omen is clear,\nApocalypse now.\n",
     "Cats cats, cats cats cats.\nCats! Cats, cats cats? Cats cats cats;\nCats, cats, \ncats \
      cats cats!\n",
     "To join our great team,\nYou must show skill at haiku\nand perhaps coding.\n",
     "She is a poet,\nI think that she may know it.\nPlease alert the press\n",
     "forlorn ex-apple\nlonely core in coffee cup\nas autumn sun shines\n",
     "when will lunch time be?\ni crave tasty sustenance\nand beer in the sun\n",
     "should haiku index\nshowcase new leaves at the top\nlike noble oak trees\n",
     "This isn't a test,\nThis is actually life.\nGet used to it fast.\n",
     "railscamp afternoon,\nleaf shades dance on dusty glass;\nhangover study\n",
     "Mat with just one 't'.\nA game: Dogsteroids. Pew pew!\nGoodbye, sweet doggies.\n",
     "The Haiku Master,\nsmiles as new Haiku appear.\nBest app at Railscamp?\n",
     "I suspect Breccan.\nFellow villagers listen,\nfor haikus don't lie.\n",
     "I believe in love;\nDo you believe in magic?\nScience reigns supreme.\n",
     "Hsoj, hsoj, come\nHsoj, hsoj, hsoj, here\nHsoj, hsoj, now\n",
     "A pull request sent.\nListening for a response.\nThe cavern echoes.\n",
     "my hands are very cold\nintermittently i place\nthem under my butt\n",
     "Warmth radiating.\nIn this finger numbing world,\nit is the small things.\n",
     "mmm coffee coffee,\nmmm coffee coffee coffee,\nI need more coffee.\n",
     "A bubble appeared.\nLong-lived but ephemeral. \nTraversed the lunch room.\n",
     "Merrin and Matt\nDead to us but still they live\non in our tiny dreams.\n",
     "The bubble had dreams.\nTo ascend into heaven\nBut settled for less.\n"];

#[derive(Debug)]
#[derive(RustcEncodable)]
pub struct Haiku<'a> {
    pub content: &'a str,
}

impl<'a> Haiku<'a> {
    pub fn choose_random() -> Self {
        match rand::thread_rng().choose(&HAIKUS) {
            Some(haiku) => Haiku { content: haiku },
            None => Haiku { content: &FALLBACK_HAIKU },
        }
    }
}
