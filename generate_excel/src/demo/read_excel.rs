use calamine::{
    open_workbook, Data, DataType, Error as CalamineError, ExcelDateTime, Reader,
    ToCellDeserializer, Xlsx,
};
use std::io::{BufReader, Cursor};
use xlsxwriter::*;
// use chrono::{NaiveDate, NaiveDateTime, Local};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();

    // 路径到你的原始Excel文件
    let original_file_path = "output_file3.xlsx";

    // 打开原始Excel文件
    let mut workbook: Xlsx<_> = open_workbook(original_file_path)?;
    let mut sheet_reader = workbook.worksheet_range("运单主表").unwrap(); // 读取第一个sheet，假设它叫"Sheet1"
    println!("运单主表 has {} rows", sheet_reader.rows().count());
    // 获取原始数据的行数，以便我们知道从哪里开始插入新数据
    // let original_rows_count = sheet_reader.rows().count();

    // 复制原始数据到新worksheet
    for (row_id, row) in sheet_reader.rows().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            // let cell_value = match cell{
            //     DataType::String(s) => s.to_string(),
            //     DataType::Float(f) => f.to_string(),
            //     DataType::Int(i) => i.to_string(),
            //     _ => "".to_string(),
            // };
            // 判断是时间日期
            match cell {
                Data::DateTime(time) => {
                    println!("1:{}", time);
                }
                _ => {
                    println!("2:{}", v);
                    worksheet.write_string(
                        row_id.try_into().unwrap(),
                        col_idx.try_into().unwrap(),
                        &v,
                        None,
                    )?;
                }
            }

            //  if let Some(text) = cell.get_string() {
            //     worksheet.write_string(
            //         row_id.try_into().unwrap(),
            //         col_idx.try_into().unwrap(),
            //         &text,
            //         None,
            //     )?;
            // }
            // if let Some(num) = cell.get_float() {
            //     worksheet.write_number(
            //         row_id.try_into().unwrap(),
            //         col_idx.try_into().unwrap(),
            //         num,
            //         None,
            //     )?;
            // }
        }
    }

    let elapsed = now.elapsed();
    // 打印耗时
    println!("耗时: {:?}", elapsed);
    Ok(())
}
