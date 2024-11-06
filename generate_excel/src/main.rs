use calamine::{
    open_workbook, Data, DataType, Error as CalamineError, ExcelDateTime, Reader,
    ToCellDeserializer, Xlsx,
};
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use std::time::Instant;
use xlsxwriter::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();

    let date = NaiveDate::from_ymd_opt(2024, 1, 23);
    let time = NaiveTime::from_hms_opt(10, 50, 20);
    let date_time = NaiveDateTime::new(date.unwrap(), time.unwrap());
    println!("DateTime: {}", date_time);
    let now_naive = Local::now().naive_local();
    println!("Now: {}", now_naive);
    // 路径到你的原始Excel文件
    // let original_file_path = "output_file3.xlsx";
    let original_file_path = "new_data.xlsx";

    // 打开原始Excel文件
    let mut workbook: Xlsx<_> = open_workbook(original_file_path)?;
    let mut sheet_reader = workbook.worksheet_range("Sheet1").unwrap(); // 读取第一个sheet，假设它叫"Sheet1"
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
                    // ExcelDateTime::as_datetime(time);
                    // let date_str = time.as_datetime().unwrap();
                    // println!("1:{}", date_str);
                    // let format = "%Y/%m/%d %H:%M:%S";
                    // let formatted_datetime = date_str.format(format).to_string();
                    // println!("1:{}", formatted_datetime)
                }
                v => {
                    println!("2:{}", v.to_string());
                }
            }
        }
    }

    let elapsed = now.elapsed();
    // 打印耗时
    println!("耗时: {:?}", elapsed);
    Ok(())
}
