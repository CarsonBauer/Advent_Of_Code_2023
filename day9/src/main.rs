use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let file_path = "./report";
    let report_vec: Vec<Vec<i32>> = read_file(file_path);
    let predictions: Vec<i32>  = predict_value(report_vec);
    print_predictions(predictions);
}

fn print_predictions(predictions: Vec<i32>) {
    
    for prediction in predictions {
        println!("{prediction}");
    }

}

fn predict_value(report_vec: Vec<Vec<i32>>) -> Vec<i32> {
    let mut output_vec: Vec<i32> = Vec::new();
    for report in report_vec {
        let mut history:Vec<Vec<i32>> = generate_history(report.clone());
        let hist_len = history.len() - 1;
        history[hist_len].push(0);
        let mut i = hist_len - 1;

        loop {
            let prev_val: i32 = history[i+1][history[i+1].len() - 1];
            let curr_val: i32 = history[i][history[i].len() - 1];
            history[i].push(curr_val + prev_val);
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        output_vec.push(history[0][history[0].len() - 1]);
    }
    return output_vec;
}

fn generate_history(incoming_vec: Vec<i32>) -> Vec<Vec<i32>> {
    let mut output_vec: Vec<Vec<i32>> = Vec::new();
    let mut run: bool = true;
    output_vec.push(incoming_vec);
    let mut out_index = 0;

    while run {
        let mut measure_vec: Vec<i32> = Vec::new();
        let curr_vec = &output_vec[out_index];
        for i in 1..curr_vec.len() {
            measure_vec.push(curr_vec[i] - curr_vec[i-1]);
        }

        output_vec.push(measure_vec.clone());

        if all_zero(&measure_vec) {
            run = false;
        }

        out_index += 1;
    }

    return output_vec;
}

fn all_zero(incoming_vec: &[i32]) -> bool {

    for i in 0..incoming_vec.len() {
        if incoming_vec[i] != 0 {
            return false;
        }
    }

    return true;
}

fn read_file(file_path: &str) -> Vec<Vec<i32>> {
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file."));
    let mut report_vec: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        let mut word_vec: Vec<i32> = Vec::new();
        for word in line.unwrap().split_whitespace() {
            word_vec.push(word.to_string().parse::<i32>().unwrap());
        }
        report_vec.push(word_vec);
    }

    return report_vec;
}

