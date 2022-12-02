// use std::collections::HashMap;
#[derive(Debug, Default, PartialEq)]
pub struct Question {
    pub question: String,
    pub answer: String,
    pub topics: Vec<i64>,
    pub box_number: usize,
}

impl Clone for Question {
    fn clone(&self) -> Self {
        Question {
            question: self.question.clone(),
            answer: self.answer.clone(),
            topics: self.topics.clone(),
            box_number: self.box_number,
        }
    }
}

impl Question {
    pub fn update_box_number(&mut self, box_number: usize) {
        self.box_number = box_number;
    }
}

pub fn set_up_questions() -> Vec<Question> {
    let questions: Vec<Question> = vec![
        Question {
            question: "1 + 1".to_string(),
            answer: "2".to_string(),
            topics: vec![0],
            box_number: Default::default(),
        },
        Question {
            question: "1 - 1".to_string(),
            answer: "0".to_string(),
            topics: vec![0],
            box_number: Default::default(),
        },
        Question {
            question: "2 * 2".to_string(),
            answer: "4".to_string(),
            topics: vec![0, 1],
            box_number: Default::default(),
        },
        Question {
            question: "2 / 2".to_string(),
            answer: "1".to_string(),
            topics: vec![0, 2],
            box_number: Default::default(),
        },
        Question {
            question: "2 ^ 2".to_string(),
            answer: "4".to_string(),
            topics: vec![1, 3],
            box_number: Default::default(),
        },
        Question {
            question: "sqrt(4)".to_string(),
            answer: "2".to_string(),
            topics: vec![1, 4],
            box_number: Default::default(),
        },
        Question {
            question: "1/2".to_string(),
            answer: "0.5".to_string(),
            topics: vec![2, 5],
            box_number: Default::default(),
        },
        Question {
            question: "50%".to_string(),
            answer: "0.5".to_string(),
            topics: vec![2, 6],
            box_number: Default::default(),
        },
    ];
    questions
}
