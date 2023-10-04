// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use serde::Serialize;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn row_to_bit(row: &[bool]) -> u8 {
    let mut result: u8 = 0;
    for i in 0..8 {
        result <<= 1;
        if row[i] {
            result |= 1;
        }
    }
    result
}

#[derive(Serialize)]
pub struct Data {
    pub text: String,
    pub num: u64,
}

impl Data {
    fn default() -> Data {
        Data {
            text: String::from(""),
            num: 0,
        }
    }
}

#[tauri::command]
fn data_to_bit(data: Vec<Vec<bool>>) -> Data {
    if data.len() != 8 {
        return Data::default();
    }
    for row in &data {
        if row.len() != 8 {
            return Data::default();
        }
    }
    let mut result: Vec<u8> = Vec::new();
    for row in data {
        result.push(row_to_bit(&row));
    }
    let mut text: String = String::from("0x");
    for i in 0..result.len() {
        text.push_str(&format!("{:02X}", result[i]));
    }
    let mut num: u64 = 0;
    for i in 0..result.len() {
        num <<= 8;
        num |= result[i] as u64;
    }
    Data { text, num }
}

fn text_to_row(text: &str) -> Result<Vec<bool>, &'static str> {
    if text.len() != 2 {
        return Err("Invalid length");
    }
    let mut result = vec![false; 8];
    let mut num: u8 = 0;
    for i in 0..2 {
        let c = text.chars().nth(i).unwrap();
        let mut n: u8 = 0;
        if c >= '0' && c <= '9' {
            n = c as u8 - '0' as u8;
        } else if c >= 'A' && c <= 'F' {
            n = c as u8 - 'A' as u8 + 10;
        } else if c >= 'a' && c <= 'f' {
            n = c as u8 - 'a' as u8 + 10;
        } else {
            return Err("Invalid character");
        }
        num <<= 4;
        num |= n;
    }
    for i in 0..8 {
        result[7 - i] = (num & 1) == 1;
        num >>= 1;
    }
    Ok(result)
}

fn num_to_data(num: u64) -> Result<Vec<Vec<bool>>, &'static str> {
    let mut num = num;
    let mut result: Vec<Vec<bool>> = Vec::new();
    for _ in 0..8 {
        let mut row: Vec<bool> = Vec::new();
        for _ in 0..8 {
            row.push((num & 1) == 1);
            num >>= 1;
        }
        result.push(row);
    }
    Ok(result)
}

#[tauri::command]
fn text_to_data(text: &str) -> Result<Vec<Vec<bool>>, &'static str> {
    let text = text
        .trim()
        .replace(" ", "")
        .replace("\n", "")
        .replace("\r", "")
        .replace("0x", "")
        .replace("0X", "");
    match text.len() {
        0 => Ok(vec![vec![false; 8]; 8]),
        16 => {
            let mut result: Vec<Vec<bool>> = Vec::new();
            for i in 0..8 {
                result.push(text_to_row(&text[2 * i..2 * i + 2])?);
            }
            Ok(result)
        }
        _ => {
            let num: u64 = match text.parse() {
                Ok(n) => n,
                Err(_) => return Err("Invalid number"),
            };
            num_to_data(num)
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, data_to_bit, text_to_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
