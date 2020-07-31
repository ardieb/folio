use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Experience<'a>
{
    pub title: &'a str,
    pub org: &'a str,
    pub loc: &'a str,
    pub desc: &'a str,
    pub details: Vec<&'a str>,
    pub href: Option<&'a str>,
    pub start: &'a str,
    pub end: Option<&'a str>,
}
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Education<'a>
{
    pub school: &'a str,
    pub degree: &'a str,
    pub desc: &'a str,
    pub details: Vec<&'a str>,
    pub href: Option<&'a str>,
    pub start: &'a str,
    pub end: Option<&'a str>,
}
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Skill<'a>
{
    pub name: &'a str,
    pub details: &'a str,
    pub href: Option<&'a str>,
    pub img: Option<&'a str>,
}
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Project<'a>
{
    pub name: &'a str,
    pub desc: &'a str,
    pub start: &'a str,
    pub img: Option<&'a str>,
    pub href: Option<&'a str>,
}
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Contact<'a>
{
    pub name: &'a str,
    pub email: &'a str,
    pub phone_num: Option<&'a str>,
}
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Social<'a>
{
    pub name: &'a str,
    pub href: &'a str,
    pub ico: Option<&'a str>,
}
