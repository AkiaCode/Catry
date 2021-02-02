use std::{fs, process::Command, thread, time};
use catry::{Token, find_branch, find_line};

fn main() {
    let file = std::env::args().nth(1).expect("파일을 입력하세요.");
    if !file.contains(".catry") {
        return eprintln!("파일 확장자가 잘 못 되었습니다 (가능한 파일: .catry)");
    }
    let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file");
    let mut num = 1;
    let mut tokens: Vec<Token> = vec![];
    for i in contents.lines() {
        if i.is_empty() { num += 1; continue; } 
        if i.contains(":") {
            let keyword = i.split(":").into_iter().nth(0).unwrap().trim();
            let word = i.split(":").into_iter().last().unwrap().trim_start();
            match keyword {
                "BRANCH" | "가지" => {
                    tokens.push(Token::new(String::from("BRANCH"), vec![String::from(word)], num, i.chars().count()));
                },
                "GOTO" | "가라" => {
                    if tokens.iter().any(|e| e.keyword == "BRANCH" || e.keyword == "가지") {
                        tokens.push(Token::new(String::from("GOTO"), vec![String::from(word)], num, i.chars().count()));
                    } else {
                        return eprintln!("TokenParserError: 'BRANCH' 또는 '가지' 키워드 없이 '{}'(을)를 사용할 수 없습니다\n\tLine: {}:{}", keyword, num, i.chars().count());
                    }
                },
                "PLAYER" | "플레이어" => {
                    if tokens.iter().any(|e| e.keyword == "BRANCH" || e.keyword == "가지") {
                        tokens.push(Token::new(String::from("PLAYER"),vec![String::from(word)], num, i.chars().count()))
                    } else {
                        return eprintln!("TokenParserError: 'BRANCH' 또는 '가지' 키워드 없이 '{}'(을)를 사용할 수 없습니다\n\tLine: {}:{}", keyword, num, i.chars().count());
                    }
                },
                "CHOICE" | "선택" => {
                    if tokens.iter().any(|e| e.keyword == "BRANCH" || e.keyword == "가지") {
                        let mut choice: Vec<String> = Vec::new();
                        for i in word.split("|").collect::<Vec<&str>>() { choice.push(i.trim().to_string()) }
                        tokens.push(Token::new(String::from("CHOICE"), choice, num, i.chars().count()));
                    } else {
                        return eprintln!("TokenParserError: 'BRANCH' 또는 '가지' 키워드 없이 '{}'(을)를 사용할 수 없습니다\n\tLine: {}:{}", keyword, num, i.chars().count());
                    }
                },
                _ => {
                    if tokens.iter().any(|e| e.keyword == "BRANCH" || e.keyword == "가지") {
                        tokens.push(Token::new(String::from(keyword), vec![String::from(word)], num, i.chars().count()));
                    } else {
                        return eprintln!("TokenParserError: 'BRANCH' 또는 '가지' 키워드 없이 '{}'(을)를 사용할 수 없습니다\n\tLine: {}:{}", i.trim(), num, i.chars().count());
                    }
                }
            }
        } else {
            return eprintln!("TokenParserError: '{}'(은)는 사용할 수 없습니다\n\tLine: {}:{}", i.trim(), num, i.chars().count());
        }
        num += 1;
    }
    //BRANCH: 1 is Start
    if !tokens.iter().any(|e| (e.keyword == "BRANCH" || e.keyword == "가지") && e.value.first().unwrap() == "1") {
        return eprintln!("RuntimeError: 'BRANCH(또는 가지): 1'(을)를 찾을 수 없습니다");
    }

    Command::new("cmd")
        .args(&["/c", "cls"])
        .spawn()
        .expect("cls command failed to start")
        .wait()
        .expect("failed to wait");
    print!(r#"
Welcome to Catry Story Book (CSB)!

Copyright (c) 2021 Catry Story Book Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
        
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
        
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"#);

    thread::sleep(time::Duration::from_millis(2000));
    println!("Now, It's time to start!");
    thread::sleep(time::Duration::from_millis(2000));
    Command::new("cmd")
        .args(&["/c", "cls"])
        .spawn()
        .expect("cls command failed to start")
        .wait()
        .expect("failed to wait");
    // Find BRANCH(또는 가지): 1
    let mut d = find_branch(find_line(tokens.iter(), 1), tokens.iter(), 1);
    // 단순 기록용
    let mut choice_number = 0;
    let mut player = 0;
loop {
    let a = d.clone().into_iter().enumerate();
    for (i, pop) in a {
        match pop.keyword.as_ref() {
            "CHOICE" | "선택" => {
                let mut choice: Vec<String> = Vec::new();
                println!("\nChoice\n=============");
                choice.push("정님".to_string());
                if !pop.value[1].is_empty() {
                    for (mut i, ch) in pop.value.to_vec().into_iter().enumerate() {
                        i += 1;
                        choice.push(ch.to_string());
                        print!("{}. {}", i, ch.to_string() + "\n");
                    }
                } else {
                    print!("1. {}", &pop.value[0].to_string());
                }
                let mut line: String = String::new();
                std::io::stdin().read_line(&mut line).unwrap();

                let mut number: usize = match line.trim().parse() {
                    Ok(number) => number,
                    Err(_) => {
                            eprintln!("위험해요...\n1~{}까지만 있지만...\n1번으로 이동시킬게욧...!\n다음에는 이러지 말아요..!", choice.len()-1);
                            thread::sleep(time::Duration::from_millis(1500));
                            Command::new("cmd")
                                .args(&["/c", "cls"])
                                .spawn()
                                .expect("cls command failed to start")
                                .wait()
                                .expect("failed to wait");
                        1
                    }
                };
                
                if number > choice.len()-1 || choice[number].is_empty() || number == 0 {
                    eprintln!("위험해요...\n1~{}까지만 있지만...\n1번으로 이동시킬게욧...!\n다음에는 이러지 말아요..!", choice.len()-1);
                    thread::sleep(time::Duration::from_millis(1500));
                    Command::new("cmd")
                        .args(&["/c", "cls"])
                        .spawn()
                        .expect("cls command failed to start")
                        .wait()
                        .expect("failed to wait");
                    number = 1;
                }
                choice_number = d[i+2*number].value[0].parse::<usize>().unwrap_or(1);
                player = number;
                //println!("You> {}", choice[number]);
                thread::sleep(time::Duration::from_millis(2000));
                Command::new("cmd")
                    .args(&["/c", "cls"])
                    .spawn()
                    .expect("cls command failed to start")
                    .wait()
                    .expect("failed to wait");
            }, 
            a => {
                if pop.keyword == "PLAYER" || pop.keyword == "가라" && player.to_string() == pop.value[0] {
                    d.clear();
                    if d.is_empty() {
                        let f = find_branch(find_line(tokens.iter(), choice_number), tokens.iter(), choice_number);
                        for i in f {
                            d.push(i);
                        }
                    }
                }
                if a == "BRANCH" || a == "GOTO" || a == "PLAYER" || a == "플레이어" || a == "가라" || a == "가지" {
                    continue; 
                } else { 
                    println!("{}: {}", pop.keyword, pop.value[0]);
                    thread::sleep(time::Duration::from_millis(1000));
                }
            }
        }
    }
}
}