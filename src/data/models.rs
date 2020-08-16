use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Experience<'a> {
    pub title: &'a str,
    pub org: &'a str,
    pub loc: &'a str,
    pub desc: &'a str,
    pub href: Option<&'a str>,
    pub start: &'a str,
    pub end: Option<&'a str>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Education<'a> {
    pub school: &'a str,
    pub degree: &'a str,
    pub desc: &'a str,
    pub href: Option<&'a str>,
    pub start: &'a str,
    pub end: Option<&'a str>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Skill<'a> {
    pub name: &'a str,
    pub details: &'a str,
    pub href: Option<&'a str>,
    pub img: Option<&'a str>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Project<'a> {
    pub name: &'a str,
    pub desc: &'a str,
    pub start: &'a str,
    pub img: Option<&'a str>,
    pub href: Option<&'a str>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Contact<'a> {
    pub link: &'a str,
    pub label: &'a str,
    pub icon: Option<&'a str>,
}
