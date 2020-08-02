use crate::data::models::*;

pub static PROJECTS: [Project; 0] = [

];

pub static EXPERIENCE: [Experience; 0] = [

];

pub static EDUCATION: [Education; 0] = [

];

pub static SKILLS: [Skill; 0] = [

];

pub static CONTACTS: [Contact; 5] = [
    Contact {
        link: "https://github.com/ardieb",
        label: "Github",
        icon: Some("fa fa-github"),
    },
    Contact {
        link: "https://www.facebook.com/arbtur",
        label: "Facebook",
        icon: Some("fa fa-facebook"),
    },
    Contact {
        link: "https://www.instagram.com/artyburke/",
        label: "Instagram",
        icon: Some("fa fa-instagram"),
    },
    Contact {
        link: "https://www.linkedin.com/in/arthur-burke-27128616a/",
        label: "LinkedIn",
        icon: Some("fa fa-linkedin"),
    },
    Contact {
        link: "mailto:me@artbur.com",
        label: "Email",
        icon: Some("fa fa-envelope"),
    },
];

pub static BLURB: &str = r"
Hi, I'm Arthur. I'm a senior at Cornell University. Mostly I program, but I've also a fascination
with physics. 'Physics is the science of approximation.' That line from an introductory mechanics
class has stayed with me. It says that our understanding of the world is not revealed but
coerced. Just a thing I like to ponder. I am a huge fan of podcasts. My favorites are Vox's Worldly,
Vox's The Weeds, Vox's Future Perfect, Mike Duncan's History of Revolutions, First Monday's,
and Revisionist History. My favourite food to cook is lemon scallop paella. Sourdough is inferior in
most applications to yeast flour. I like hosting dinner parties. I've spent a month backpacking in Denali
";
