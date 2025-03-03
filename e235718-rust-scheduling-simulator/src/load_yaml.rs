use std::fs;
use yaml_rust::YamlLoader;

#[derive(Debug)]
pub struct Task {
    pub id: usize,
    pub name: String,
    pub clock: usize,
}

impl Task {
    pub fn task_exec(mut self) {
        println!(
            "id: {}, name: {}, clock: {}",
            self.id, self.name, self.clock
        );
        while self.clock > 0 {
            println!("task runnnig id:{}, clock:{}", self.id, self.clock);
            self.clock -= 1;
        }
        println!("task {} execed!\n", self.name);
    }
}

pub fn load_tasks_from_yaml(path: &str) -> Vec<Task> {
    let file_content = fs::read_to_string(path).expect("can't open file");
    let docs = YamlLoader::load_from_str(&file_content).expect("failed to road to YAML");
    let doc = &docs[0];

    doc.as_vec()
        .expect("YAML's format is not vector")
        .iter()
        .map(|entry| {
            let hash = entry.as_hash().expect("YAML's format is not hash");

            let id = hash
                .get(&yaml_rust::Yaml::String("id".to_string()))
                .and_then(|v| v.as_i64())
                .expect("can't find id") as usize;

            let name = hash
                .get(&yaml_rust::Yaml::String("name".to_string()))
                .and_then(|v| v.as_str())
                .expect("can't find name")
                .to_string();

            let clock = hash
                .get(&yaml_rust::Yaml::String("clock".to_string()))
                .and_then(|v| v.as_i64())
                .expect("can't find clock") as usize;

            Task { id, name, clock }
        })
        .collect()
}

// fn main() {
//     // // ファイル読み込み
//     // let path = "tasks.yaml";
//     // let tasks = load_yaml::load_tasks_from_yaml(path);
//     // for task in tasks {
//     //     task.task_exec();
//     // }
// }
