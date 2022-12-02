use crate::set_up_graph::set_up_graph;
use crate::set_up_graph::Node;
use crate::set_up_questions::set_up_questions;
use crate::set_up_questions::Question;
// This trait is required to use `try_next()` on the mongo cursor
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};


use mongodb::bson::{Document};
use std::cmp::max;
use std::collections::HashMap;
use std::io::{self, Write};
use tokio::task;

mod mongo_class;
mod set_up_graph;
mod set_up_questions;

fn input(prompt: &str) -> String {
    let mut s: String = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

#[derive(Debug, Clone)]
struct Leitner {
    boxes: HashMap<usize, Vec<Question>>,
    graph: HashMap<usize, Node>,
    questions: Vec<Question>,
}

impl Leitner {
    fn add_box_number(&mut self) {
        for i in 0..self.questions.len() {
            let box_num = ((i + 1) as f32).log2().floor() as usize;
            // println!("Box number: {} {}", i, box_num);
            // self.boxes.insert(box_num, self.questions[i].clone());
            // add item to a list in the key value pair of the hashmap
            let mut cloned_question = self.questions[i].clone();
            cloned_question.box_number = box_num;
            self.boxes
                .entry(box_num)
                .or_insert(vec![])
                .push(cloned_question);
        }
        // print!("Boxes: {:?}", self.boxes);
    }

    fn print_stats(&self) {
        for i in 0..self.boxes.len() {
            println!(
                "Box number: {}. This box has at most {} questions",
                i,
                2usize.pow(i as u32)
            );
            for question in self.boxes.get(&i).unwrap() {
                println!(
                    "Question: {} Answer: {}",
                    question.question, question.answer
                );
            }
            println!()
        }
    }

    fn get_question(&self, topic: i32) -> Question {
        for i in 0..self.questions.len() {
            for question in &self.boxes[&i] {
                if topic == -1 || question.topics.contains(&(topic as i64)) {
                    return question.clone();
                }
            }
        }
        panic!("No questions found");
    }

    fn move_question(&mut self, question: &Question, mut box_number: usize) {
        box_number = max(box_number, 0);
        let mut cloned_question = question.clone();
        cloned_question.box_number = box_number;
        self.boxes
            .entry(box_number)
            .or_insert(vec![])
            .push(cloned_question);
        // remove question from old box
        for i in 0..self.boxes[&question.box_number].len() {
            if self.boxes[&question.box_number][i] == question.clone() {
                self.boxes.get_mut(&question.box_number).unwrap().remove(i);
                break;
            }
        }
    }

    fn answer_question(&mut self, question: Question, answer: String) -> bool {
        if question.answer == answer {
            Self::move_question(self, &question, question.box_number + 1);
            return true;
        } else {
            let target = if question.box_number > 0 { 0 } else { 1 };
            Self::move_question(self, &question, target);
            return false;
        }
    }

    pub fn __init__(&mut self) {
        self.questions.sort_by_key(|q| q.topics.len());
        // update box numbers
        Self::add_box_number(self);
        Self::print_stats(&self);
    }
}


#[tokio::main]
async fn main() {
    //     let graph = set_up_graph();
    //     let questions: Vec<Question> = set_up_questions();
    //     let mut leitner = Leitner {
    //         boxes: HashMap::new(),
    //         graph: graph,
    //         questions: questions,
    //     };
    //     leitner.__init__();
    //     // leitner is immutable, so we need to clone it
    //     // loop {
    //     let question = leitner.get_question(-1);
    //     let answer = input(&question.question);
    //     if leitner.answer_question(question, answer.to_string()) {
    //         println!("Correct!");
    //     } else {
    //         println!("Incorrect!");
    //     }
    let db = mongo_class::set_up_database().await;
    print!("Database: {:?}", db);
    let collection = db.collection::<Document>("accounts");
    let mut cursor = collection.find(None, None).await.unwrap();

    // Iterate over the results of the cursor.
    // while let Ok(Some(document)) = cursor.try_next().await{
    //     println!("Found document {:?}", document);
    // }
}
