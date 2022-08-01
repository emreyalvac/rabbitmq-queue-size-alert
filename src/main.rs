use std::io;
use std::time::Duration;
use job_scheduler::{Job, JobScheduler};
use lapin::{Connection, ConnectionProperties};
use lapin::options::QueueDeclareOptions;
use lapin::types::FieldTable;
use serde::{Serialize, Deserialize};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    rabbit: Vec<Rabbit>,
    slack_webhook_url: String,
    cron: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Rabbit {
    name: String,
    url: String,
    ui_url: String,
    alerts: Vec<Alert>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Alert {
    queue_name: String,
    threshold: i32,
}

async fn parse_config_file() -> io::Result<Config> {
    let mut file = File::open("alerts.toml").await?;
    let mut file_buffer = String::new();

    file.read_to_string(&mut file_buffer).await?;

    let decoded: Config = toml::from_str(&file_buffer).unwrap();

    Ok(decoded)
}

async fn send_queue_size_to_slack(hook_url: &String, queue_name: String, queue_size: i32, threshold: i32) -> () {
    let echo_json = reqwest::Client::new()
        .post(hook_url)
        .json(&serde_json::json!({
            "blocks": [
                {
                    "type": "section",
                    "text": {
                        "type": "mrkdwn",
                        "text": format!("{} : {}, Threshold: {}", queue_name, queue_size, threshold)
                    },
                },
            ]
        }))
        .send()
        .await;
    ()
}

async fn run_job(conf: Config) {
    for conn in conf.rabbit {
        match Connection::connect(
            &conn.url,
            ConnectionProperties::default(),
        )
            .await {
            Ok(rabbit_conn) => {
                let channel = rabbit_conn.create_channel().await.unwrap();

                for alert in conn.alerts {
                    let mut queue_declare_options = QueueDeclareOptions::default();
                    queue_declare_options.passive = true;
                    queue_declare_options.durable = true;
                    queue_declare_options.auto_delete = false;
                    match channel.queue_declare(alert.queue_name.as_str(), queue_declare_options, FieldTable::default()).await {
                        Ok(queue) => {
                            let queue_name = queue.name();
                            let message_count = queue.message_count() as i32;
                            let threshold = alert.threshold;

                            if message_count >= threshold {
                                send_queue_size_to_slack(&conf.slack_webhook_url, queue_name.to_string(), message_count, threshold).await;
                            }
                        }
                        Err(_) => {
                            println!("Queue not found");
                        }
                    }
                }
            }
            Err(_) => {
                println!("Connection error");
            }
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let conf = parse_config_file().await.unwrap();
    let mut sched = JobScheduler::new();

    sched.add(Job::new(conf.cron.parse().unwrap(), move || {
        tokio::spawn(run_job(conf.clone()));
    }));

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }


    Ok(())
}
