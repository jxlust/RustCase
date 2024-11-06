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
    // let original_file_path = "output_file3.xlsx";
    // 新Excel文件的路径
    let new_file_path = "new_data.xlsx";

    // 打开原始Excel文件
    // let mut workbook: Xlsx<_> = open_workbook(original_file_path)?;
    // let mut sheet_reader = workbook.worksheet_range("运单主表").unwrap(); // 读取第一个sheet，假设它叫"Sheet1"
    // println!("Sheet1 has {} rows", sheet_reader.rows().count());
    // 获取原始数据的行数，以便我们知道从哪里开始插入新数据
    // let original_rows_count = sheet_reader.rows().count();

    // 创建一个新的Excel workbook和worksheet
    let new_workbook = Workbook::new(new_file_path)?;
    let mut worksheet = new_workbook.add_worksheet(Some("Sheet1"))?;
    let mut worksheet2 = new_workbook.add_worksheet(Some("Sheet2"))?;
    let mut worksheet3 = new_workbook.add_worksheet(Some("Sheet3"))?;
    let mut worksheet4 = new_workbook.add_worksheet(Some("Sheet4"))?;

    // 复制原始数据到新worksheet
    // for (row_id, row) in sheet_reader.rows().enumerate() {
    //     for (col_idx, cell) in row.iter().enumerate() {
    //         // let cell_value = match cell{
    //         //     DataType::String(s) => s.to_string(),
    //         //     DataType::Float(f) => f.to_string(),
    //         //     DataType::Int(i) => i.to_string(),
    //         //     _ => "".to_string(),
    //         // };
    //         let v = cell.to_string().to_string();
    //         // 判断是时间日期
    //         match cell {
    //             Data::DateTime(time) => {
    //                 // println!("1:{}", time);
    //                  // let date = NaiveDateTime::partial_cmp(&self, other)

    //                 // let date_float = time.as_f64();
    //                 // let excel_epoch = NaiveDate::from_ymd(1899, 12, 30);
    //                 // let days = (date_float.floor() as i32) - 2;
    //                 // //  let mut naive_date = excel_epoch.naive_local() + chrono::Duration::days(days);

    //                 //  // 处理可能出现的小数部分（例如 0.5 代表一天的一半，转换为时间）
    //                 //  let fraction = date_float.fract();
    //                 //  let hours = (fraction * 24.0).floor() as u32;
    //                 //  let minutes = ((fraction * 24.0 - hours as f64) * 60.0).floor() as u32;
    //                 //  let seconds = ((fraction * 24.0 * 60.0 - hours as f64 * 60.0 - minutes as f64) * 60.0).floor() as u32;

    //                 //  println!("Time as string:{} {}:{}:{}",days, hours, minutes, seconds);

    //                 //  naive_date = naive_date + chrono::Duration::hours(hours);
    //                 //  naive_date = naive_date + chrono::Duration::minutes(minutes);
    //                 //  naive_date = naive_date + chrono::Duration::seconds(seconds);

    //                 //  println!("Date as string: {}", naive_date.format("%Y-%m-%d"));
    //             },
    //             _ => {
    //                 // println!("2:{}", v);
    //                  worksheet.write_string(
    //                     row_id.try_into().unwrap(),
    //                     col_idx.try_into().unwrap(),
    //                     &v,
    //                     None,
    //                 )?;
    //             }
    //         }

    //         //  if let Some(text) = cell.get_string() {
    //         //     worksheet.write_string(
    //         //         row_id.try_into().unwrap(),
    //         //         col_idx.try_into().unwrap(),
    //         //         &text,
    //         //         None,
    //         //     )?;
    //         // }
    //         // if let Some(num) = cell.get_float() {
    //         //     worksheet.write_number(
    //         //         row_id.try_into().unwrap(),
    //         //         col_idx.try_into().unwrap(),
    //         //         num,
    //         //         None,
    //         //     )?;
    //         // }
    //     }
    // }

    // 生成并写入1000条新数据
    for row_index in 0..(1000000 + 1000) {
        // 这里是示例数据，你可以根据需要替换
        let new_data = [
            &format!("NewItem{}", row_index), // 假设这是你的新运单号或其他唯一标识符
            "P575ZDA",                        // 原寄地
            "851SG",                          // 目的地
            "3.2",                            // 计费重量
            "2024/6/7 15:08:40",              // 寄件时间
            "kg",                             // 重量单位
            "SE0165",                         // 产品代码
            "T682",                           // 时效标签
            "2024/9/5 20:13:00",              // 更新时间
        ];
        for (col_index, data) in new_data.iter().enumerate() {
            worksheet.write_string(
                row_index.try_into().unwrap(),
                col_index.try_into().unwrap(),
                data,
                None,
            )?;

            worksheet2.write_string(
                row_index.try_into().unwrap(),
                col_index.try_into().unwrap(),
                data,
                None,
            )?;
            worksheet3.write_string(
                row_index.try_into().unwrap(),
                col_index.try_into().unwrap(),
                data,
                None,
            )?;
            worksheet4.write_string(
                row_index.try_into().unwrap(),
                col_index.try_into().unwrap(),
                data,
                None,
            )?;
        }
    }

    // // 关闭workbook，保存新文件
    new_workbook.close();
    let elapsed = now.elapsed();
    // 打印耗时
    println!("耗时: {:?}", elapsed);
    Ok(())
}
