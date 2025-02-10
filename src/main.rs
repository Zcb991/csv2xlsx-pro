use rust_xlsxwriter::{Workbook, XlsxError};
use std::fs::File;
use std::io::{BufReader, BufRead};
use csv::ReaderBuilder;

use chrono::Local;

use std::error::Error;

use indicatif::{ProgressBar, ProgressStyle};  // 新增进度条库
use std::env;

const ROWS_PER_FILE: usize = 1_000_000;
// const ROWS_PER_FILE: usize = 1_00_000;

// 1200行，每个文件100w行，约456秒(453秒)
// 1200w, 每个文件100w行，190s，（恒定内存模式下，提升了约2.3倍速度）

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = std::time::SystemTime::now();
    println!("开始时间：{}", Local::now().format("%Y-%m-%d %H:%M:%S"));

    println!("临时文件目录: {:?}", env::temp_dir());

    // 获取当前 exe 文件所在目录
    let current_dir = env::current_exe()?.parent().unwrap().to_path_buf();

    let file_path = "a.csv";

    // 定义输入和输出文件名
    let input_file = current_dir.join("a.csv");
    let output_prefix = current_dir.join("a-part{n}.xlsx");

    // 检查输入文件是否存在
    if !input_file.exists() {
        eprintln!("源文件不存在：{}", input_file.display());
        return Err("源文件不存在".into());
    }

    let source_path = input_file
        .to_str()
        .ok_or("源文件路径包含非 UTF-8 字符")?;
    println!("源文件路径：{}", source_path);

    let output_file = output_prefix
        .to_str()
        .ok_or("目标文件路径包含非 UTF-8 字符")?;
    println!("目标文件路径：{}", output_file);

    // 先统计总行数
    let total_rows = count_csv_rows(file_path)?;
    println!("文件总行数（不含表头）：{}", total_rows);
    let total_files = (total_rows + ROWS_PER_FILE - 1) / ROWS_PER_FILE;
    println!("预计生成文件数：{}", total_files);
    // println!("文件总行数：{}，预计生成文件数：{}", total_rows, total_files);

    // 初始化进度条
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    pb.inc(0);  // 初始进度条

    let file = File::open(file_path)?;
    // 8MB的缓冲区
    let buf_reader = BufReader::with_capacity(8*1024*1024, file);

    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(buf_reader);
    // let mut reader = ReaderBuilder::new().has_headers(true).from_reader(BufReader::new(file));

    // let total_rows = reader.lines().count().saturating_sub(1); 
    // println!("Total rows: {}", total_rows);
    let headers: Vec<String> = reader.headers()?.iter().map(|s| s.to_string()).collect();

    let mut file_index = 1;
    let mut current_chunk = Vec::with_capacity(ROWS_PER_FILE);

    // let mut cnt = 0;

    for result in reader.records() {
        let record = result?;
        let row_data: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        current_chunk.push(row_data);

        if current_chunk.len() == ROWS_PER_FILE {
            // cnt += 1;
            // println!("生成任务批次: {}", cnt);
            let file_name = format!("a-part{}.xlsx", file_index);
            write_to_excel(&file_name, &headers, &current_chunk)?;
            current_chunk.clear();
            file_index += 1;

            pb.inc(1);  // 每完成一个文件更新进度条
        }
    }

    if !current_chunk.is_empty() {
        // cnt += 1;
        // println!("生成任务批次: {}", cnt);
        let file_name = format!("a-part{}.xlsx", file_index);
        write_to_excel(&file_name, &headers, &current_chunk)?;

        pb.inc(1);  // 最后一个文件更新进度条
    }

    pb.finish_with_message("所有文件生成完成");

    // println!("All files have been generated successfully.");
    println!("结束时间：{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!("所有文件生成完成，总用时：{}秒", start_time.elapsed()?.as_secs());
    Ok(())
}

fn write_to_excel(
    file_name: &str,
    headers: &[String],
    data: &[Vec<String>],
) -> Result<(), XlsxError> {
    // println!("Time: {}, Saving file: {}", Local::now().format("%Y-%m-%d %H:%M:%S"), file_name);
    let mut workbook = Workbook::new();

    // let worksheet = workbook.add_worksheet();

    let worksheet = workbook.add_worksheet_with_constant_memory();

    worksheet.write_row(0, 0, headers)?;

    for (row_index, row) in data.iter().enumerate() {
        worksheet.write_row((row_index + 1) as u32, 0, row)?;
        // worksheet.write_multi_row(row, col, data)?;
    }
    workbook.save(file_name)?;
    // println!("Time: {}, Saved file: {}", Local::now().format("%Y-%m-%d %H:%M:%S"), file_name);
    Ok(())
}

fn count_csv_rows(file_path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(file_path)?;
    // let reader = BufReader::new(file);
    let reader = BufReader::with_capacity(8*1024*1024, file);
    let total_rows = reader.lines().count().saturating_sub(1);  // 使用 saturating_sub 以避免下溢
    Ok(total_rows)
}