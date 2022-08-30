/**
 * コマンドラインツール
 * 報告内容を考えられるような何か
 * メモリがわからんからRust導入
 */
use std::io::{self, Write,Error};
use std::fs::{self,File};
use std::{env,path};
//TODO
//一捻り加える

//事実、解釈、行動
struct Report{
    title:String,
    fact:String,
    aspect:String,
    action:String,
}

fn main() {
    let mut title_buf = String::new();
    title_buf = syutoku("報告タイトル");
    //同名ファイルがあるか検索
    let create_file_name = format!("{}{}",title_buf,".txt");
    let search_result = search(&create_file_name);
    if search_result{
        println!("既に同名のファイルが存在します\nタイトルを書き換えてください");
        title_buf = syutoku("報告タイトル");
    }
    //レポートとしてタイトル以外を入力してもらう
    let report = Report{
        title: title_buf,
        fact: syutoku("事実"),
        aspect: syutoku("解釈"),
        action: syutoku("行動"),
    };
    println!("テキストファイルを作成します");

    //テキストファイルを作成する処理
    create_text(report);

    println!("作成しました\n処理を終了します");

}

fn search(create_file_name:&str)-> bool{
    //ファイル名が同名のものがあったらtrue、なければfalseを返す
    let mut flg:bool = false;
    //カレントディレクトリから検索する
    let current_path = path::PathBuf::from(".");
    let files_dir = current_path.read_dir().expect("存在しません");
    for file in files_dir{
        let file_path = file.unwrap().path();
        if file_path.is_dir(){
            continue;
        }
        let file_name = file_path.file_name().unwrap().to_string_lossy();
        if create_file_name == &file_name{
            flg = true;
            return flg;
        }
    }
    return flg;
}

//テキストファイルの作成
fn create_text(report: Report)->io::Result<()>{
    {//書き込みが有効なブロック
        //ここで、スライスを作成しないと、formatの作成でtitleのライフタイムが終わってしまう
        let filename = format!("{}{}",&report.title,".txt");
        let fp = File::create(filename).unwrap();
        let mut writer = io::BufWriter::new(fp);
        let line = format!("タイトル：{}\n 事実：{}\n 解釈：{}\n 行動：{}\n"
                                    ,report.title,report.fact,report.aspect,report.action);
        let buf = line.as_bytes();
        writer.write(buf).unwrap();
    }
    Ok(())
}

//標準入力へ入力したい内容を表示して取得する処理
fn syutoku(print_name:&str)-> String{
    println!("{}を入力",print_name);
    input()
}

//標準入力から取得する処理
fn input() -> String{
    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence).expect("文字入力エラー");
    sentence.trim().to_string()
}
