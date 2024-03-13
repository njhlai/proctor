use std::error::Error;
use std::io::{self, Write};

use colored::Colorize;
use reqwest::blocking::Client;
use serde::Deserialize;
use strum::Display;

use crate::modules::fetcher::{GraphQLResponse, Method, Request, Response};
use crate::modules::lang::Lang;

use super::metadata::MetaData;
use super::QuestionDetails;

const QUESTION_LIST_QUERY: &str = r#"
query questionList($skip: Int) {
  obj: questionList(
    categorySlug: ""
    limit: 1
    skip: $skip
    filters: {}
  ) {
    questions: data {
      questionFrontendId
      content
      metaData
      codeSnippets {
        lang
        langSlug
        code
      }
      exampleTestcases
    }
  }
}
"#;

#[derive(Display)]
enum LeetcodeURL {
    #[strum(to_string = "https://leetcode.com/graphql")]
    GraphQL,
}

type QuestionDataQuery = Request<String, GraphQLResponse<QuestionList>>;

impl QuestionDataQuery {
    fn new(id: usize) -> Self {
        Request::from(
            LeetcodeURL::GraphQL.to_string(),
            Method::POST,
            QUESTION_LIST_QUERY,
            format!("{{\"skip\": \"{id}\"}}"),
        )
    }
}

#[derive(Debug, Deserialize)]
struct QuestionList {
    questions: Vec<QuestionData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuestionData {
    question_frontend_id: String,
    content: String,
    meta_data: String,
    code_snippets: Vec<CodeSnippetJson>,
    example_testcases: String,
}

#[derive(Debug, Deserialize)]
struct CodeSnippetJson {
    lang: String,
    code: String,
}

pub fn query(id: &str, lang: &Lang) -> Result<QuestionDetails, Box<dyn Error>> {
    let client = Client::new();

    print!("Querying question data for problem {}... ", id.cyan().bold());
    io::stdout().flush()?;

    let question = &QuestionDataQuery::new(id.parse::<usize>()?.saturating_sub(1))
        .response(&client)?
        .data
        .questions[0];
    assert!(format!("{:0>4}", question.question_frontend_id) == id);
    println!("{}!", "OK".green().bold());

    Ok((
        question.content.clone(),
        question
            .code_snippets
            .as_slice()
            .iter()
            .find(|q| q.lang == lang.get_name())
            .map(|q| q.code.clone()),
        MetaData::from(&question.meta_data, lang)?,
        question.example_testcases.clone(),
    ))
}
