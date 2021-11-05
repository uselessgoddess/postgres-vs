use tokio::time::Instant;
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;
    connection.await?;

    let instant = Instant::now();
    println!("start creation");

    client.query(r#"
        DO $$
            BEGIN
                FOR i IN 1..1000000 LOOP
                    INSERT INTO links
                        values (i, i, i);
                END LOOP;
            END
        $$;
    "#, &[]).await?;

    println!("creation elapsed: {:?}", instant.elapsed());
    Ok(())
}