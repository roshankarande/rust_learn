use anyhow::Ok;
use sqlx::{Row, FromRow};

#[derive(Debug, FromRow)]
struct Message {
    id: i64,
    message: String,
}

async fn update_message(id: i64, message: &str, pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
    sqlx::query("UPDATE messages SET message = ? WHERE id = ?")
        .bind(message)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    tracing_subscriber::fmt::init();
    
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;


    // let messages = sqlx::query("SELECT id, message FROM messages")
    //     .map(|row: sqlx::sqlite::SqliteRow| {
    //         let id: i64 = row.get(0);
    //         let message: String = row.get(1);
    //         (id, message)
    //     })
    //     .fetch_all(&pool)
    //     .await?;

//     for (id, message) in messages {
//         println!("{id}: {message}");
// }
    
    println!("--- fetch all ---");
    let messages = sqlx::query_as::<_, Message>("SELECT id, message FROM messages")
    .fetch_all(&pool)
    .await?;

    for message in messages.into_iter() {
        println!("{message:?}");
    }

    update_message(1, "Updated First Message", &pool).await?;

    println!("--- stream ---");
    use futures::TryStreamExt;
    let mut message_stream = sqlx::query_as::<_, Message>("SELECT id, message FROM messages")
        .fetch(&pool);
    while let Some(message) = message_stream.try_next().await? {
        println!("{message:?}");
    }

    Ok(())
}
