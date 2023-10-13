// Q1.1,1.2
// fn make_document(text: &str) -> Vec<String> {
//     let mut paragraphs = vec![];
//     let mut current_paragraph = String::new();

//     for line in text.lines() {
//         if line.trim().is_empty() {
//             if !current_paragraph.is_empty() {
//                 paragraphs.push(current_paragraph.clone());
//                 current_paragraph.clear();
//             }
//         } else {
//             current_paragraph.push_str(line);
//             current_paragraph.push('\n');
//         }
//     }

//     if !current_paragraph.is_empty() {
//         paragraphs.push(current_paragraph);
//     }

//     paragraphs
// }

// fn rank_documents(docs: &Vec<Vec<String>>) -> Vec<Vec<String>> {
//     let mut ranked_docs = docs.clone();
//     ranked_docs.sort_by_key(|doc| doc.len());
//     ranked_docs.reverse();
//     ranked_docs
// }

// fn main() {
//     let fox = "The quick brown fox jumps over the lazy dog.";
//     let para3 = "a\n\nb\n\nc";
//     let bustle = "\
// The bustle in a house\n\
// The morning after death\n\
// Is solemnest of industries\n\
// Enacted upon earth,—\n\
// \n\
// The sweeping up the heart,\n\
// And putting love away\n\
// We shall not want to use again\n\
// Until eternity.\n\
// ";

//     let doc1 = make_document(fox);
//     let doc2 = make_document(bustle);
//     let doc3 = make_document(para3);

//     let docs = vec![doc1.clone(), doc3.clone(), doc2.clone()];
//     let rnk_docs = rank_documents(&docs);

//     assert_eq!(rnk_docs, [doc3, doc2, doc1]);

//     for (i, doc) in rnk_docs.iter().enumerate() {
//         println!("Document {} ({} paragraphs):", i + 1, doc.len());
//         for paragraph in doc {
//             println!("{}", paragraph);
//         }
//         println!();
//     }
// }





// Q2.1
// use std::fs;
// use std::io::{BufReader, BufRead};
// use std::path::Path;

// struct Document {
//     file_name: String,
//     paragraph_count: usize,
// }

// fn count_paragraphs(file_path: &str) -> usize {
//     let file = fs::File::open(file_path).expect("Unable to open file");
//     let reader = BufReader::new(file);
//     let mut paragraph_count = 0;
//     let mut in_paragraph = false;

//     for line in reader.lines() {
//         if let Ok(line) = line {
//             if line.trim().is_empty() {
//                 in_paragraph = false;
//             } else if !in_paragraph {
//                 paragraph_count += 1;
//                 in_paragraph = true;
//             }
//         }
//     }
//     paragraph_count
// }


// fn generate_html_table(documents: Vec<Document>) -> String {
//     let mut table_content = String::from("<table border='1'><tr><th>File Name</th><th>Paragraph Count</th></tr>");
//     for doc in documents {
//         table_content.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", doc.file_name, doc.paragraph_count));
//     }
//     table_content.push_str("</table>");
//     table_content
// }

// fn main() {
//     let file_list = vec!["file1.txt", "file2.txt", "file3.txt"]; // Replace with your list of file names
//     let mut documents: Vec<Document> = Vec::new();

//     for file_name in file_list {
//         let file_path = Path::new(file_name);
//         if file_path.exists() {
//             let paragraph_count = count_paragraphs(file_name);
//             documents.push(Document {
//                 file_name: file_name.to_string(),
//                 paragraph_count,
//             });
//         } else {
//             println!("File not found: {}", file_name);
//         }
//     }

//     documents.sort_by(|a, b| b.paragraph_count.cmp(&a.paragraph_count));

//     let html_table = generate_html_table(documents);
//     fs::write("output.html", html_table).expect("Unable to write file");

//     println!("HTML table generated successfully.");
// }


// Q2.2)
use std::fs;
use std::io::{BufReader, BufRead};
use std::path::Path;

struct Document {
    file_name: String,
    word_count: usize,
}

fn count_words(file_path: &str) -> usize {
    let file = fs::File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut word_count = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            word_count += line.split_whitespace().count();
        }
    }
    word_count
}

fn generate_html_table(documents: Vec<Document>) -> String {
    let mut table_content = String::from("<table border='1'><tr><th>File Name</th><th>Word Count</th></tr>");
    for doc in documents {
        table_content.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", doc.file_name, doc.word_count));
    }
    table_content.push_str("</table>");
    table_content
}

fn main() {
    let file_list = vec!["file1.txt", "file2.txt", "file3.txt"]; // Replace with your list of file names
    let mut documents: Vec<Document> = Vec::new();

    for file_name in file_list {
        let file_path = Path::new(file_name);
        if file_path.exists() {
            let word_count = count_words(file_name);
            documents.push(Document {
                file_name: file_name.to_string(),
                word_count,
            });
        } else {
            println!("File not found: {}", file_name);
        }
    }

    documents.sort_by(|a, b| b.word_count.cmp(&a.word_count));

    let html_table = generate_html_table(documents);
    fs::write("output1.html", html_table).expect("Unable to write file");

    println!("HTML table generated successfully.");
}
