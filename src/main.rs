// запуск
// cargo run --release C:\rust\projects\marketplace\05082024\heads\struct_project.txt
/* 
cargo run --release C:\rust\projects\marketplace\05082024\heads\struct_project.txt

*/

/* 
можешь по-русски? скажи - ты понял смысл программы основной? 
я расскажу - программа отслеживает прописанные нами файлы типа 
PathBuf::from(r"C:\rust\projects\marketplace\31072024\insite\heads\postcss.config.js") - 
и у нас есть файл со структурой проекта, короче, вот он @struct_project.txt 
- и если мы изменяем содержимое отслеживаемых файлов, 
то программа тут же вносит изменения в struct_project.txt в области с маркером 
<code name_file.rs>Тут мы колдуем</code name_files.rs> - 
надеюсь, что понятно рассказал, просто перевари это и далее я расскажу, 
в чем проблема
*/
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config, Event, EventKind};
use std::fs;
use std::sync::mpsc::channel;
use std::path::{Path, PathBuf};
use std::env;

fn main() {
    // Получаем аргументы командной строки
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_project_structure_file>", args[0]);
        return;
    }
    let project_structure_file = Path::new(&args[1]);

    // Проверка существования файла структуры проекта
    if !project_structure_file.exists() {
        eprintln!("Project structure file does not exist: {}", project_structure_file.display());
        return;
    }

    // Создаем канал для получения уведомлений
    let (tx, rx) = channel();

    // Создаем наблюдателя за файлами
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

    // Пути к файлам, которые нужно отслеживать
    let files_to_watch: Vec<PathBuf> = vec![
        PathBuf::from(r"C:\rust\projects\marketplace\05082024\heads\src\main.rs"),
        PathBuf::from(r"C:\rust\projects\marketplace\05082024\heads\src\app.rs"),
        PathBuf::from(r"C:\rust\projects\marketplace\05082024\heads\Cargo.toml"),
        PathBuf::from(r"C:\rust\projects\marketplace\05082024\heads\Trunk.toml"),
        PathBuf::from(r"C:\rust\projects\marketplace\05082024\heads\index.html"),
        PathBuf::from(r"C:\rust\projects\marketplace\05082024\heads\package.json"),
        PathBuf::from(r"C:\rust\projects\marketplace\05082024\heads\tailwind.config.js"),
        PathBuf::from(r"C:\rust\projects\marketplace\05082024\heads\terminal.md"),

        
        


    ];

    // Проверка существования файлов и добавление их в наблюдатель
    for file in &files_to_watch {
        if file.exists() {
            watcher.watch(file, RecursiveMode::NonRecursive).unwrap();
        } else {
            eprintln!("Path does not exist or is not a file: {}", file.display());
        }
    }

    // Основной цикл для обработки событий
    loop {
        match rx.recv() {
            Ok(event) => match event {
                Ok(Event { kind: EventKind::Modify(_), paths, .. }) => {
                    for path in paths {
                        println!("{:?} был изменен", path);
                    }
                    update_project_structure(project_structure_file, &files_to_watch);
                }
                _ => (),
            },
            Err(e) => println!("Ошибка наблюдателя: {:?}", e),
        }
    }
}

// Функция для обновления структуры проекта
fn update_project_structure(project_structure_file: &Path, files: &[PathBuf]) {
    let mut structure_content = fs::read_to_string(project_structure_file).unwrap();

    for file_path in files {
        let file_content = fs::read_to_string(file_path).unwrap();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        // Обновление содержимого файла в структуре проекта
        let start_marker = format!("<code {}>", file_name);
        let end_marker = format!("</code {}>", file_name);

        if let Some(start_index) = structure_content.find(&start_marker).map(|index| index + start_marker.len()) {
            if let Some(end_index) = structure_content.find(&end_marker) {
                println!("Updating content for {} between {} and {}", file_name, start_index, end_index); // Debug print
                structure_content.replace_range(start_index..end_index, &format!("\n{}\n", file_content));
            } else {
                println!("End marker not found for {}", file_name); // Debug print
            }
        } else {
            println!("Start marker not found for {}", file_name); // Debug print
        }
    }

    fs::write(project_structure_file, structure_content).unwrap();
}