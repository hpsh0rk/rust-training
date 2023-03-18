use anyhow::Result;
use queryer::query;

#[tokio::main]
async fn main() -> Result<()> {
    let sql = format!(
        "SELECT pid,name,cpu_percent,memory FROM ps where memory>20000000 ORDER BY memory DESC",
    );
    let df1 = query(sql).await?;
    println!("{:?}", df1);

    Ok(())
}
