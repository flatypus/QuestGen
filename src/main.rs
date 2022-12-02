use crate::set_up_graph::set_up_graph;
use crate::set_up_graph::Node;
use crate::set_up_questions::set_up_questions;
use crate::set_up_questions::Question;
use std::collections::HashMap;

pub mod set_up_graph;
pub mod set_up_questions;
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

    fn print_stats(self) {
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

    pub fn __init__(mut self) {
        self.questions.sort_by_key(|q| q.topics.len());
        // update box numbers
        Self::add_box_number(&mut self);
        Self::print_stats(self);
    }
}

fn main() {
    let graph = set_up_graph();
    let questions: Vec<Question> = set_up_questions();
    let leitner = Leitner {
        boxes: HashMap::new(),
        graph: graph,
        questions: questions,
    };
    leitner.__init__();
}
