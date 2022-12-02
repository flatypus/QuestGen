import math
import random
import collections
import time
import uuid

graph = {
    0: {"head": [], "child": [1, 2], "name": "addition and subtraction"},
    1: {"head": [0], "child": [3, 4], "name": "multiplication"},
    2: {"head": [0], "child": [5, 6], "name": "division"},
    3: {"head": [1], "child": [], "name": "exponents"},
    4: {"head": [1], "child": [], "name": "square roots"},
    5: {"head": [2], "child": [], "name": "fractions"},
    6: {"head": [2], "child": [], "name": "percentages"},
}

questions = [
    {"question": "1 + 1", "answer": "2", "topics": {0}, "uuid": uuid.uuid4()},
    {"question": "1 - 1", "answer": "0", "topics": {0}, "uuid": uuid.uuid4()},
    {"question": "2 * 2", "answer": "4",
        "topics": {0, 1}, "uuid": uuid.uuid4()},
    {"question": "2 / 2", "answer": "1",
        "topics": {0, 2}, "uuid": uuid.uuid4()},
    {"question": "2 ^ 2", "answer": "4",
        "topics": {1, 3}, "uuid": uuid.uuid4()},
    {"question": "sqrt(4)", "answer": "2", "topics": {
        1, 4}, "uuid": uuid.uuid4()},
    {"question": "1/2", "answer": "0.5",
        "topics": {2, 5}, "uuid": uuid.uuid4()},
    {"question": "50%", "answer": "0.5",
        "topics": {2, 6}, "uuid": uuid.uuid4()},
]

# Leitner system


class Leitner:
    def __init__(self, questions):
        # self.questions = questions
        self.boxes = collections.defaultdict(list)
        questions.sort(key=lambda x: len(x["topics"]), reverse=True)
        # add questions to box such that the nth box has at most 2^n questions
        for i in range(len(questions)):
            boxnum = math.floor(math.log(i + 1, 2))
            self.boxes[boxnum].append(questions[i])
            # add box number
            self.boxes[boxnum][-1]["box"] = boxnum

    def print_stats(self):
        for i in range(len(self.boxes.keys())):
            print("box", i, ":", len(self.boxes[i]))

    def get_question(self, topic=-1):
        for i in range(len(questions)):
            for question in self.boxes[i]:
                if topic == -1 or topic in question["topics"]:
                    return question

    def shift_question(self, question, box):
        box = max(0, box)
        if not self.boxes:
            self.boxes[box] = []
        self.boxes[box].append(self.boxes[question["box"]].pop(
            self.boxes[question["box"]].index(question)))
        # update box
        self.boxes[box][-1]["box"] = box

    def answer_question(self, question, answer):
        if (question["answer"] == answer):
            self.shift_question(question, question["box"] + 1)
            return True
        else:
            self.shift_question(question, question["box"] - 1)
            return False


def main():
    Test = Leitner(questions)
    while True:
        question = Test.get_question(-1)
        print(question["question"])
        answer = input()
        if Test.answer_question(question, answer):
            print("Correct!")
        else:
            print("Incorrect!")
        # Test.print_stats()


if __name__ == "__main__":
    main()
