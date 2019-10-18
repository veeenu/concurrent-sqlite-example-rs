extern crate rand;
use rusqlite::{Result, Connection, ToSql, NO_PARAMS};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use threadpool::ThreadPool;

fn create() -> Result<usize> {
  let conn = Connection::open("test.db")?;
  conn.execute("DROP TABLE IF EXISTS `main`", NO_PARAMS)?;
  conn.execute("CREATE TABLE `main` (
    `id`	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    `thread` INTEGER,
    `threadindex` INTEGER,
    `pwd`	TEXT
  );", NO_PARAMS)
}

fn writer(i: i64) -> Result<()> {
  let conn = Connection::open("test.db")?;
  conn.busy_handler(Some(|_| {
    std::thread::sleep(std::time::Duration::from_millis(16));
    true
  }))?;
  let strings = (1..1000).map(|j| {
    (i, j, thread_rng()
      .sample_iter(&Alphanumeric)
      .take(30)
      .collect::<String>())
  }).collect::<Vec<(i64, i64, String)>>();
  conn.execute("BEGIN TRANSACTION", NO_PARAMS)?;
  let mut stmt = conn.prepare("INSERT INTO `main` (`thread`, `threadindex`, `pwd`) VALUES (?, ?, ?)")?;
  for s in strings {
    stmt.execute(&[&s.0, &s.1, &s.2 as &dyn ToSql])?;
  }

  conn.execute("COMMIT", NO_PARAMS)?;
  println!("Thread {} finished", i);
  Ok(())
}

fn main() {
  create().unwrap();
  let pool = ThreadPool::new(64);

  let now = std::time::Instant::now();
  (1..100).into_iter().map(|i| {
    pool.execute(move || {
      writer(i).unwrap();
    });
    1
  }).sum::<i32>();

  pool.join();
  let end = now.elapsed().as_millis() as f64 / 1000f64;

  println!("Bye, elapsed {}s", end);
}
