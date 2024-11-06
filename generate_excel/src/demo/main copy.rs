use calamine::{open_workbook, Error, Reader, Xlsx};
use std::io::Cursor;
use xlsxwriter::*;

fn main() -> Result<(), Error> {
    // 定义列名（假设这些列名已经存在于原始Excel文件的第一个sheet中）
    let columns = [
        "运单号",
        "原寄地",
        "目的地",
        "计费重量",
        "寄件时间",
        "重量单位",
        "产品代码",
        "时效标签",
        "更新时间",
    ];

    // 创建一个新的Excel文件和worksheet
    let new_workbook = Workbook::new("output1.xlsx");
    let worksheet = new_workbook.add_worksheet(None)?;

    // 写入列名到新的worksheet中
    for (index, column) in columns.iter().enumerate() {
        worksheet.write_string(0, index, column);
    }

    // 生成并写入10000条测试数据
    for row_index in 1..=10000 {
        let test_data = [
            &row_index.to_string(), // 索引值
            "P575ZDA",              // 原寄地
            "851SG",                // 目的地
            "3.2",                  // 计费重量
            "2024/6/7 15:08:40",    // 寄件时间
            "kg",                   // 重量单位
            "SE0165",               // 产品代码
            "T682",                 // 时效标签
            "2024/9/5 20:13:00",    // 更新时间
        ];
        for (col_index, data) in test_data.iter().enumerate() {
            worksheet.write_string(row_index, col_index, data);
        }
    }

    // 可选：自动调整列宽以适应内容
    // for col_index in 0..columns.len() {
    //     worksheet.auto_filter(0, col_index, 10000, col_index); // 添加自动筛选
    //     worksheet.set_column(col_index, col_index, 20); // 设置列宽为20（可根据需要调整）
    // }

    // 关闭新的workbook，这会自动保存文件
    new_workbook.close();

    Ok(())
}
