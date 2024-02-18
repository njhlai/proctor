use std::error::Error;
use std::io::{self, Write};

use colored::Colorize;
use reqwest::blocking::Client;
use serde::Deserialize;
use strum::Display;

use crate::modules::fetcher::{Empty, Method, Query, QueryResponse, Response};
use crate::modules::lang::Lang;

const QUESTION_DATA_QUERY: &str = r"
query getQuestionDetail($titleSlug: String!) {
  obj: question(titleSlug: $titleSlug) {
    questionFrontendId
    content
    codeSnippets {
      lang
      langSlug
      code
    }
  }
}
";
const EMPTY_QUERY: &str = "";

#[derive(Display)]
enum LeetcodeURL {
    #[strum(to_string = "https://leetcode.com/graphql")]
    GraphQL,
    #[strum(to_string = "https://leetcode.com/api/problems/all")]
    APIProblemsAll,
}

type QuestionDataQuery = Query<String, QueryResponse<QuestionData>>;

impl QuestionDataQuery {
    fn new(title: &str) -> Self {
        Query::from(
            LeetcodeURL::GraphQL.to_string(),
            Method::POST,
            QUESTION_DATA_QUERY,
            format!("{{\"titleSlug\": \"{title}\"}}"),
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuestionData {
    question_frontend_id: String,
    content: String,
    code_snippets: Vec<CodeSnippetJson>,
}

#[derive(Debug, Deserialize)]
struct CodeSnippetJson {
    lang: String,
    code: String,
}

type ProblemSetQuery = Query<Empty, ProblemSet>;

impl ProblemSetQuery {
    fn new() -> Self {
        Query::from(LeetcodeURL::APIProblemsAll.to_string(), Method::GET, EMPTY_QUERY, Empty)
    }
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

pub fn query(id: &str, lang: &Lang) -> Result<(String, Option<String>), Box<dyn Error>> {
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
        .as_str();
    println!("{}!", "OK".green().bold());

    print!("Querying question data for problem {}... ", id.cyan().bold());
    io::stdout().flush()?;

    let question = QuestionDataQuery::new(title).response(&client)?.data;
    assert!(format!("{:0>4}", question.question_frontend_id) == id);
    println!("{}!", "OK".green().bold());

    Ok((
        question.content,
        question
            .code_snippets
            .into_iter()
            .find(|q| q.lang == lang.get_name())
            .map(|q| q.code.clone()),
    ))
}
