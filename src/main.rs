use flate2::read::GzDecoder;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tar::Archive;
use zip::ZipArchive;

// 遍历目录并分类文件
fn traverse_and_classify(dir: &Path) -> io::Result<HashMap<String, Vec<PathBuf>>> {
    let mut classifications: HashMap<String, Vec<PathBuf>> = HashMap::new();

    // 递归遍历目录
    fn recurse_dir(
        dir: &Path,
        classifications: &mut HashMap<String, Vec<PathBuf>>,
    ) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                recurse_dir(&path, classifications)?;
            } else if path.is_file() {
                let extension = path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext.to_lowercase())
                    .unwrap_or_else(|| "".to_string());

                classifications
                    .entry(extension.clone())
                    .or_default()
                    .push(path.clone());

                // 处理压缩文件
                if extension == "gz" {
                    if let Ok(_) = decompress_gz(&path, classifications) {
                        fs::remove_file(path)?; // 删除原压缩文件
                    }
                } else if extension == "zip" {
                    if let Ok(_) = decompress_zip(&path, classifications) {
                        fs::remove_file(path)?; // 删除原压缩文件
                    }
                }
            }
        }
        Ok(())
    }

    recurse_dir(dir, &mut classifications)?;
    Ok(classifications)
}

// 解压 .gz 文件
fn decompress_gz(
    file_path: &Path,
    classifications: &mut HashMap<String, Vec<PathBuf>>,
) -> io::Result<()> {
    let file = fs::File::open(file_path)?;
    let decompressed = GzDecoder::new(file);
    let mut archive = Archive::new(decompressed);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.to_path_buf();
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
            .unwrap_or_else(|| "".to_string());

        let outpath = file_path.with_file_name(path.file_name().unwrap());
        entry.unpack(&outpath)?;
        classifications.entry(extension).or_default().push(outpath);
    }
    Ok(())
}

// 解压 .zip 文件
fn decompress_zip(
    file_path: &Path,
    classifications: &mut HashMap<String, Vec<PathBuf>>,
) -> io::Result<()> {
    let file = fs::File::open(file_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = file_path.with_file_name(file.name());
        let mut outfile = fs::File::create(&outpath)?;
        io::copy(&mut file, &mut outfile)?;

        let extension = outpath
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
            .unwrap_or_else(|| "".to_string());

        classifications.entry(extension).or_default().push(outpath);
    }
    Ok(())
}

// 打印分类结果
fn print_classification_results(classifications: &HashMap<String, Vec<PathBuf>>) {
    for (extension, files) in classifications {
        println!("{}. {} files", extension, files.len());
    }
}

// 抽样文件并复制到新目录
fn sample_files(
    classifications: &HashMap<String, Vec<PathBuf>>,
    sample_ratio: f64,
    output_dir: &Path,
) -> io::Result<()> {
    fs::create_dir_all(output_dir)?;

    let mut rng = rand::thread_rng();
    for (extension, files) in classifications {
        let sample_size = (files.len() as f64 * sample_ratio).ceil() as usize;
        let sample: Vec<&PathBuf> = files.choose_multiple(&mut rng, sample_size).collect();

        for file in &sample {
            let file_name = file.file_name().unwrap();
            let dest = output_dir.join(file_name);
            fs::copy(file, dest)?;
        }
        println!("Sampled {} {} files", sample.len(), extension);
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let input_dir = Path::new("input_directory");
    let classifications = traverse_and_classify(&input_dir)?;

    print_classification_results(&classifications);

    let sample_ratio = 0.1; // 10% 抽样比例
    let output_dir = Path::new("output_directory");
    sample_files(&classifications, sample_ratio, &output_dir)?;

    Ok(())
}
