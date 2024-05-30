use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
};

use chrono::{Local, NaiveDateTime, ParseError};
use clap::Parser;

use exif::{In, Tag};
use sha2::{Digest, Sha256};

use walkdir::WalkDir;

const DATE_FORMAT_YMD_HMS: &str = "%Y-%m-%d %H:%M:%S";
const DATE_FORMAT_YMD: &str = "%Y-%m-%d";

fn main() {
    let args = Args::parse();

    let begin_time = chrono::Utc::now().timestamp_millis();

    let eligible_images = get_eigible_images(&args.src);

    if eligible_images.is_empty() {
        println!("没有找到支持扩展名的文件");
    }

    let (all_count, success_count, error_count) = classify(eligible_images, &args.target);

    let end_time = chrono::Utc::now().timestamp_millis();

    println!(
        "完成! 总共检索了 [{}] 个文件, 成功归类 [{}] 个文件，失败 [{}] 个文件, 总共用时 [{} ms]",
        all_count,
        success_count,
        error_count,
        end_time - begin_time
    );
}

// 归类函数. 返回结果(一共扫描的文件数,成功归类的文件数,失败的文件数)
fn classify(eligible_images: Vec<FileMetadata>, target_path: &str) -> (u32, u32, u32) {
    let mut sha256_mapping: HashMap<&str, &FileMetadata> = HashMap::new();
    let mut name_mapping: HashMap<&str, &FileMetadata> = HashMap::new();

    let success_count: u32 = eligible_images
        .iter()
        .map(|x| do_classify(x, &mut sha256_mapping, &mut name_mapping, target_path))
        .sum();

    (
        eligible_images.len() as u32,
        success_count,
        eligible_images.len() as u32 - success_count,
    )
}

// 真正执行归类的逻辑
fn do_classify<'a>(
    file_metadata: &'a FileMetadata,
    sha256_mapping: &mut HashMap<&'a str, &'a FileMetadata>,
    name_mapping: &mut HashMap<&'a str, &'a FileMetadata>,
    target_path: &str,
) -> u32 {
    // 如果name之前处理过一样的。直接跳过这次
    if name_mapping.contains_key(file_metadata.name.as_str()) {
        let target_file = name_mapping.get(file_metadata.name.as_str()).unwrap();

        let same_name_file =
            gen_file_target_path(target_path, &target_file.classify_path, &target_file.name);

        println!(
            "警告：发现同名文件,跳过不处理. 当前文件位置: [{}], 同名文件位置: [{}]",
            &file_metadata.path, same_name_file
        );
        return 0;
    }

    // 检查sha256是否有相同的
    if sha256_mapping.contains_key(file_metadata.sha256.as_str()) {
        let target_file = sha256_mapping.get(file_metadata.sha256.as_str()).unwrap();

        let same_sha256_file =
            gen_file_target_path(target_path, &target_file.classify_path, &target_file.name);

        println!(
            "警告：发现 sha256 相同的文件,跳过不处理. 当前文件位置: [{}], sha256相同文件位置: [{}]",
            &file_metadata.path, same_sha256_file
        );
        return 0;
    }

    let binding = gen_file_target_path(
        target_path,
        &file_metadata.classify_path,
        &file_metadata.name,
    );
    let target_file = Path::new(&binding);

    // 拷贝文件
    match std::fs::rename(&file_metadata.path, target_file) {
        Ok(_) => {
            sha256_mapping.insert(file_metadata.sha256.as_str(), file_metadata);
            name_mapping.insert(file_metadata.name.as_str(), file_metadata);
            return 1;
        }
        Err(e) => {
            eprintln!("发生错误：{}", e);
            return 0;
        }
    }
}

// 得到所有符合条件的文件
fn get_eigible_images(src_path: &str) -> Vec<FileMetadata> {
    let mut candidate: Vec<FileMetadata> = Vec::new();

    let paths = WalkDir::new(src_path);

    let support_image_extension = get_support_image_extension();

    for entry in paths.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(os_str) = path.extension() {
                if let Some(extension) = os_str.to_str() {
                    if support_image_extension.contains(&extension.to_uppercase()) {
                        let file_metadata = get_file_metadata(path).unwrap();
                        candidate.push(file_metadata);
                    }
                }
            } else {
                println!("警告: [{}] >>>>>> 无法获取扩展名.", path.to_string_lossy());
            }
        }
    }

    candidate
}

// 获取文件信息
fn get_file_metadata(path: &Path) -> Option<FileMetadata> {
    let sha256 = compute_sha256(&path.display().to_string()).unwrap();

    // 计算拍摄时间
    let file = File::open(path).unwrap();
    let mut bufreader = BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).unwrap();

    if let Some(data) = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
        let shooting_time = data.display_value().to_string();

        if let Ok(classify_path) =
            conversion_data_format(&shooting_time, DATE_FORMAT_YMD_HMS, DATE_FORMAT_YMD)
        {
            let metadata = FileMetadata::new(
                path.display().to_string(),
                path.file_name().unwrap().to_str().unwrap().to_string(),
                sha256,
                classify_path,
            );
            return Some(metadata);
        }
    };
    None
}

// 计算文件的sha256
fn compute_sha256(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;

    let mut vec = Vec::new();
    file.read_to_end(&mut vec)?;

    let mut hasher = Sha256::new();
    hasher.update(vec);
    let result = hasher.finalize();

    let hex_strs: Vec<String> = result[..]
        .iter()
        .map(|&byte| format!("{:02X}", byte))
        .collect();

    Ok(hex_strs.join("").to_lowercase())
}

// 定义参数
#[derive(Parser, Debug)]
#[command(version,about,long_about=None)]
struct Args {
    /// 需要检查的目录
    #[arg(short, long)]
    src: String,
    /// 归类后的目录
    #[arg(short, long)]
    target: String,
}

#[derive(Debug)]
struct FileMetadata {
    // 文件路径
    path: String,
    // 文件名
    name: String,
    // sha256
    sha256: String,
    // 拍摄时间
    classify_path: String,
}

impl FileMetadata {
    fn new(path: String, name: String, sha256: String, classify_path: String) -> Self {
        FileMetadata {
            path,
            name,
            sha256,
            classify_path,
        }
    }
}

// 得到目前所有支持的照片格式
fn get_support_image_extension() -> HashSet<String> {
    vec!["TIFF", "RAW", "HEIF", "JPEG", "WEBP", "PNG", "JPG"]
        .into_iter()
        .map(|x| x.to_string())
        .collect()
}

// 得到文件的目标文件
fn gen_file_target_path(target_path: &str, shooting_time: &str, file_name: &str) -> String {
    let path = Path::new(target_path).join(shooting_time);
    // 创建目录
    if !path.exists() {
        let _ = std::fs::create_dir_all(&path);
    }
    path.join(file_name).as_path().to_string_lossy().to_string()
}

// 时间格式转换
fn conversion_data_format(
    date: &str,
    src_date_format: &str,
    target_date_format: &str,
) -> Result<String, ParseError> {
    match NaiveDateTime::parse_from_str(date, src_date_format) {
        Ok(native_date_time) => {
            let date_time = native_date_time
                .and_local_timezone(Local)
                .unwrap()
                .format(target_date_format)
                .to_string();
            return Ok(date_time);
        }
        Err(err) => return Err(err),
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Local, NaiveDateTime};
    use exif::{In, Tag};

    use super::*;

    #[test]
    fn test_get_file_metadata() {
        let path = Path::new("/Users/zhangdi/Downloads/wzzs.sql");
        let file_metadata = get_file_metadata(&path).unwrap();
        println!("{file_metadata:?}");
    }

    #[test]
    fn test_compute_sha256() {
        let sha256 =
            compute_sha256("/Users/zhangdi/Downloads/引擎-用户参数遍历行为检测.png").unwrap();

        assert_eq!(
            "ac23c56076e6af360d593e2b0b8287a1dfbb348dbfcf4d088ca158167d3ab1d9",
            sha256.to_lowercase()
        );
    }

    #[test]
    fn test_image_metadata() {
        let file =
            File::open("/Volumes/Samsung_T5/照片/myself/2014年9月29日/IMG_1355.jpeg").unwrap();
        let mut bufreader = BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader).unwrap();

        let cc = exif
            .get_field(Tag::DateTimeOriginal, In::PRIMARY)
            .unwrap()
            .display_value()
            .to_string();

        println!("========={}", cc);

        let format = "%Y-%m-%d %H:%M:%S";
        let local_datetime = NaiveDateTime::parse_from_str(&cc, &format)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();

        let cc = local_datetime.format("%Y%m%d%H%M%S").to_string();

        println!("===={}", cc);

        println!("------{}", local_datetime.timestamp());

        for f in exif.fields() {
            println!(
                "{} -- {} -- {}",
                f.tag,
                f.ifd_num,
                f.display_value().with_unit(&exif)
            );
        }
    }
}
