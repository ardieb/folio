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

pub static EXPERIENCE: [Experience; 5] = [
    Experience {
        title: "MathWorks EDG Internship",
        org: "The MathWorks, Inc.",
        loc: "Natick, Ma",
        desc: r"
            As an engineering intern for the engineering development group at The MathWorks, I
            worked alongside the Matlab online team to develop a new api layer for The MathWorks
            network stack. The culumination of my work is a redesign of the entire Mathworks stack
            that uses the reactive socket layer 5 transport protocol at its base. This work involded
            devling into both the C++ and JavaSript MathWorks APIs and provided exposure to api
            design, networking protocol design, tenents of reactive programming, in addition to
            reaching out to downstream teams in negotiating the needs of their application specific
            code.
        ",
        href: Some("https://www.mathworks.com/company/jobs/students/edg.html"),
        start: "May, 2020",
        end: Some("Aug, 2020"),
    },
    Experience {
        title: "Bruin LLC Software Design Internship",
        org: "Bruin LLC",
        loc: "New York, Ny",
        desc: r"
            As an intern at Bruin, I developed a subscription based model for distributing
            company information and order events through a react native application. The prototype
            application I developed showcased use cases for a cross platform application in
            distributing important alerts about order statuses, corporate news events, and todo
            information. Tangential to this project, I worked alongside a subset of the development
            team in developing react components for the company website and developing a machine
            learning algorithm for helping clients and employees locate more relevant information
            through company internal search. These projects ran the gammit of .NET programming with C#,
            react native development with JavaScript, native code development for IOS and Android in Swift and Java,
            SQL database access, and firebase google tools.
        ",
        href: Some("https://www.bruin.com/company/"),
        start: "Jun, 2018",
        end: Some("Sept, 2018"),
    },
    Experience {
        title: "CISER Research Consultant",
        org: "Cornell Institute for Socio-Economic Research",
        loc: "Ithaca, Ny",
        desc: r"
            At CISER, I work with students, phd/graduate researchers, and Cornell professors to
            provide assistance using CISER's servers and preparing statistical research. This work
            centers around administative server management of CISER's servers, qualifying new tools,
            assisting researchers in using CISER tools and in preparing research statistics, and
            educating researchers in statiscal software.
        ",
        href: Some("https://ciser.cornell.edu/"),
        start: "Sept, 2019",
        end: None
    },
    Experience {
        title: "Hover Front End Developer",
        org: r"Skwads LLC.",
        loc: "Online",
        desc: r"
            With friends at Cornell and colleagues from my internship at Bruin, I worked on the
            Hover social media app, a react native application for interacting with fellow gamers
            by sharing and reacting to clips. My contributions to the project centered in front-end
            react native development, predominantly in IOS, and spanned into the realm of machine
            learning with python sklearn for improving the apps recomendation system. This algorithm
            was deployed using AWS SAM and invoked a lambda function for providing computation results
            to the company database.
        ",
        href: Some("https://hover.gg"),
        start: "Sept, 2019",
        end: Some("May, 2020"),
    },
    Experience {
        title: "CisLunar Embedded Developer",
        org: "Cornell University",
        loc: "Ithaca, Ny",
        desc: r"
            The CisLunar satellite mission targets the cube sat platform with fprime development.
            As a team member, I developed software using NASA JPL's fprime c++ framework for the
            cube sat platform. My focus targeted the drivers for the satellite's power system and
            single use separation thruster.
        ",
        href: None,
        start: "Feb, 2019",
        end: Some("Aug, 2019"),
    }
];

pub static EDUCATION: [Education; 1] = [
    Education {
        school: "Cornell University",
        degree: "Bachelors of Arts",
        gpa: "3.93",
        desc: "Physics and Computer Science Dual Major",
        href: Some("www.cornell.edu"),
        start: "Aug, 2017",
        end: Some("May, 2021"),
    }
];

pub static SKILLS: [Skill; 5] = [
    Skill {
        name: "C++ Development",
        details: r"
            Fluent in modern C++ development (^C++14). Significant experience using boost libraries
            and networking with boost asio and boost beast. Experienced with concurrency
            and multi-threaded applications. Experienced with the cmake toolchain.
        ",
        href: None,
        img: None
    },
    Skill {
        name: "Python Development",
        details: r"
            Very experienced with python development. Significant expeirence using ml libraries:
            sklearn, keras, tensorflow, and pytorch. Significant experience using pandas, numpy,
            and sqlite. Experience with AWS lambda using SAM layers. Experience with python for
            scientific applications (predominantly physics simulation).
        ",
        href: None,
        img: None
    },
    Skill {
        name: "React / React Native Development",
        details: r"
            Very experience with both react and react native development. Signficant experience
            using redux for state management and the newer context hook. Significant experience
            writing and using react hooks. Experience deploying iOS applications written in react
            native.
        ",
        href: None,
        img: None
    },
    Skill {
        name: "Embedded Development",
        details: r"
            Experience with arduino and rasberry pi. Experience writing kernel drivers for
            debbian. Experience interoperating python and c for targeting the rasberry pi platform.
        ",
        href: None,
        img: None
    },
    Skill {
        name: "Networking Design",
        details: r"
            Significant experience designing rpc systems. Experience interfacing javascript
            web clients with C++ microservices. Experience using reactive networking protocols and
            asynchronous coding design. Knowledgable in TCP, BGP, UDP, RPC, DVP, Rsocket, and HTTP/2.
        ",
        href: None,
        img: None
    }
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
