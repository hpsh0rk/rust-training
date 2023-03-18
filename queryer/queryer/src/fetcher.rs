use anyhow::{anyhow, Result};
use async_trait::async_trait;
use tokio::fs;
use sysinfo::{ProcessExt, System, SystemExt};

#[async_trait]
pub trait Fetch {
    type Error;
    async fn fetch(&self) -> Result<String, Self::Error>;
}

pub async fn retrieve_data(source: impl AsRef<str>) -> Result<String> {
    let name = source.as_ref();
    if &name[..2] == "ps" {
        PsFetcher().fetch().await
    } else {
        match &name[..4] {
            "http" => UrlFetcher(name).fetch().await,
            "file" => FileFetcher(name).fetch().await,
            _ => Err(anyhow!("We only support http/https/file at the moment")),
        }
    }
}

struct UrlFetcher<'a>(&'a str);
struct FileFetcher<'a>(&'a str);
struct PsFetcher();

#[async_trait]
impl<'a> Fetch for UrlFetcher<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        Ok(reqwest::get(self.0).await?.text().await?)
    }
}

#[async_trait]
impl<'a> Fetch for FileFetcher<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        Ok(fs::read_to_string(&self.0[7..]).await?)
    }
}

#[async_trait]
impl Fetch for PsFetcher {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        let system = System::new_all();

        let mut result = vec!["pid,name,cpu_percent,memory\n".to_string()];

        // Iterate over all currently running processes
        for (pid, process) in system.processes() {
            let name = process.name().to_string();
            let cpu_percent = process.cpu_usage() as f32;
            let memory = process.memory();

            let data = format!("{},{},{},{}\n", pid.to_string(), name, cpu_percent, memory);
            result.push(data)
        }
        Ok(result.into_iter().collect())
    }
}
