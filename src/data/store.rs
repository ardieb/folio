use crate::data::models::*;

pub static PROJECTS: [Project; 6] = [
    Project {
        name: "Lightstrip",
        desc: r"
            A dynamic led lightshow that combines arduino, rasberry pi, aws lambda, alexa skills,
            and fourier transforms. The application pipeline starts with the alexa skill. A voice
            command is processed and interpreted to play a particular song or display a particular
            animation. The command updates a shadow document that triggers a call to aws lambda.
            The lambda function sends the document to a server running on a rasberry pi. The pi then
            sends the led animation sequence to the arduino mico controller, which controls a ws2011
            led light strip. The pi is able to convert audio byte sequences to animations by analyzing
            the dominant harmonics in the audio data and producing corresponding color patterns.
        ",
        start: "Summer, 2018",
        img: None,
        href: Some("https://github.com/ardieb/ledlights"),
    },
    Project {
        name: "OAlgebra",
        desc: r"
            A linear algebra library for the OCaml functional programming language. This project
            comes complete with an interpreted language and AST which understands linear algebra
            operations described in strings. The library closely mirrors the design of the MATLAB
            programming language or that of Wolfram Alpha.
        ",
        start: "Spring, 2019",
        img: None,
        href: Some("https://github.com/ardieb/oAlgebra"),
    },
    Project {
        name: "CryptoAI",
        desc: r"
            A machine learning project built using pytorch and the coinbase API. This project
            seeks to analyze both realtime and history trends in the price of various crypto
            currencies, using LSTM neural network.
        ",
        start: "Summer, 2020",
        img: None,
        href: Some("https://github.com/ardieb/cryptoAI"),
    },
    Project {
        name: "ISSTracker",
        desc: r"
            A simple python app that uses NASA's APIs to track the geo coordinates of the ISS.
            The position of the ISS is displayed overlayed on the world map using DASH callbacks.
        ",
        start: "Summer, 2019",
        img: None,
        href: Some("https://github.com/ardieb/ISS-Tracker"),
    },
    Project {
        name: "rusty_tracer",
        desc: r"
            A ray tracing application using the rust programming language. Uses coordinate
            transformations to allow for NAABBs in addition to spherical objects, surfaces, and
            triangles. All rendering is done on the cpu.
        ",
        start: "Summer, 2020",
        img: Some("https://raw.githubusercontent.com/ardieb/RustyTracer/master/result.png"),
        href: Some("https://github.com/ardieb/RustyTracer"),
    },
    Project {
        name: "Trump Tweet Classifier",
        desc: r"
            Analyzes the tweeting behavior of the president. Is it him? Is it an aid? Find out
            with this machine learning tool, written with python. The ai uses nltk in addition to
            the vader sentiment score to alayze different aspects of the president's tweets.
        ",
        start: "Fall, 2019",
        img: None,
        href: Some("https://github.com/ardieb/TrumpTweetClassifier"),
    }
];

pub static EXPERIENCE: [Experience; 4] = [
    Experience {
        title: "",
        org: "",
        loc: "",
        desc: "",
        href: None,
        start: "",
        end: None
    },
    Experience {
        title: "",
        org: "",
        loc: "",
        desc: "",
        href: None,
        start: "",
        end: None
    },
    Experience {
        title: "",
        org: "",
        loc: "",
        desc: "",
        href: None,
        start: "",
        end: None
    },
    Experience {
        title: "",
        org: "",
        loc: "",
        desc: "",
        href: None,
        start: "",
        end: None
    }
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
";
