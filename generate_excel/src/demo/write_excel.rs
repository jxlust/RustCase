use calamine::{
    open_workbook, Data, DataType, Error as CalamineError, ExcelDateTime, Reader,
    ToCellDeserializer, Xlsx,
};
use std::io::{BufReader, Cursor};
use std::time::Instant;

pub fn write_demo() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();

    // 新Excel文件的路径
    let new_file_path = "new_data.xlsx";

    // 创建一个新的Excel workbook和worksheet
    let new_workbook = Workbook::new(new_file_path)?;
    let mut worksheet = new_workbook.add_worksheet(Some("Sheet1"))?;
    let mut worksheet2 = new_workbook.add_worksheet(Some("Sheet2"))?;
    let mut worksheet3 = new_workbook.add_worksheet(Some("Sheet3"))?;
    let mut worksheet4 = new_workbook.add_worksheet(Some("Sheet4"))?;

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
