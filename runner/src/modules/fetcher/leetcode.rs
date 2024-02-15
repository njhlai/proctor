use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

use colored::Colorize;
use reqwest::blocking::Client;
use serde::Deserialize;
use strum::Display;

use crate::modules::lang::Lang;

use super::query::{Constructible, Query, QueryResponse, Response};

const QUESTION_DATA_QUERY: &str = r"
query getQuestionDetail($titleSlug: String!) {
  obj: question(titleSlug: $titleSlug) {
    questionFrontendId
    codeSnippets {
      lang
      langSlug
      code
    }
  }
}
";
const EMPTY_QUERY: &str = "";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuestionData {
    question_frontend_id: String,
    code_snippets: Vec<CodeSnippetJson>,
}

#[derive(Debug, Deserialize)]
struct CodeSnippetJson {
    lang: String,
    code: String,
}

#[derive(Debug, Deserialize)]
struct ProblemSet {
    stat_status_pairs: Vec<StatStatusPair>,
}

#[derive(Debug, Deserialize)]
struct StatStatusPair {
    stat: Stat,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct Stat {
    frontend_question_id: usize,
    question__title_slug: String,
}

#[derive(Display)]
enum LeetcodeURL {
    #[strum(to_string = "https://leetcode.com/graphql")]
    GraphQL,
    #[strum(to_string = "https://leetcode.com/api/problems/all")]
    APIProblemsAll,
}

type QuestionDataQuery = Query<String, QueryResponse<QuestionData>>;

impl QuestionDataQuery {
    fn new(title: String) -> Self {
        Query::from(LeetcodeURL::GraphQL.to_string(), QUESTION_DATA_QUERY, title)
    }
}

impl Constructible for QuestionDataQuery {
    fn json(&self) -> HashMap<&str, String> {
        let mut json = HashMap::new();

        json.insert("query", String::from(self.query));
        json.insert("variables", r#"{"titleSlug": "$titleSlug"}"#.replace("$titleSlug", &self.variable));

        json
    }
}

type ProblemSetQuery = Query<(), ProblemSet>;

impl ProblemSetQuery {
    fn new() -> Self {
        Query::from(LeetcodeURL::APIProblemsAll.to_string(), EMPTY_QUERY, ())
    }
}

impl Constructible for ProblemSetQuery {
    fn json(&self) -> HashMap<&str, String> {
        HashMap::new()
    }
}

pub fn query(id: &str, lang: &Lang) -> Result<String, Box<dyn Error>> {
    let usize_id = id.parse::<usize>()?;

    let client = Client::new();

    print!("Querying problems... ");
    io::stdout().flush()?;

    let problems = ProblemSetQuery::new().response(&client)?;
    let title = problems
        .stat_status_pairs
        .iter()
        .find(|s| s.stat.frontend_question_id == usize_id)
        .unwrap()
        .stat
        .question__title_slug
        .clone();
    println!("{}!", "OK".green().bold());

    print!("Querying question data for problem {}... ", id.cyan().bold());
    io::stdout().flush()?;

    let question = QuestionDataQuery::new(title).response(&client)?.data;
    assert!(format!("{:0>4}", question.question_frontend_id) == id);
    println!("{}!", "OK".green().bold());

    Ok(question
        .code_snippets
        .into_iter()
        .find(|q| q.lang == lang.get_name())
        .unwrap()
        .code
        .clone())
}
