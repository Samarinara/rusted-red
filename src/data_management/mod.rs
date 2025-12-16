use rusqlite::{Connection, Result};
use std::{fs, iter::Enumerate, path::Path};

pub static SQLITE_RESERVED_KEYWORDS: &[&str] = &[
    "abort",
    "action",
    "add",
    "after",
    "all",
    "alter",
    "analyze",
    "and",
    "as",
    "asc",
    "attach",
    "autoincrement",
    "before",
    "begin",
    "between",
    "by",
    "cascade",
    "case",
    "cast",
    "check",
    "collate",
    "column",
    "commit",
    "conflict",
    "constraint",
    "create",
    "cross",
    "current",
    "current_date",
    "current_time",
    "current_timestamp",
    "database",
    "deferrable",
    "deferred",
    "delete",
    "desc",
    "detach",
    "distinct",
    "do",
    "drop",
    "each",
    "else",
    "end",
    "escape",
    "except",
    "exclusive",
    "exists",
    "explain",
    "fail",
    "for",
    "foreign",
    "from",
    "full",
    "glob",
    "group",
    "having",
    "if",
    "ignore",
    "immediate",
    "in",
    "index",
    "indexed",
    "initially",
    "inner",
    "insert",
    "instead",
    "intersect",
    "into",
    "is",
    "isnull",
    "join",
    "key",
    "left",
    "like",
    "limit",
    "match",
    "natural",
    "no",
    "not",
    "nothing",
    "notify",
    "null",
    "of",
    "offset",
    "on",
    "or",
    "order",
    "outer",
    "plan",
    "pragma",
    "primary",
    "query",
    "raise",
    "recursive",
    "references",
    "regexp",
    "reindex",
    "release",
    "rename",
    "replace",
    "restrict",
    "right",
    "rollback",
    "row",
    "savepoint",
    "select",
    "set",
    "table",
    "temp",
    "temporary",
    "then",
    "to",
    "transaction",
    "trigger",
    "union",
    "unique",
    "update",
    "using",
    "vacuum",
    "values",
    "view",
    "virtual",
    "when",
    "where",
    "with",
];

pub fn create_database_from_csv(file_path: &str, csv_path: &str) -> Result<()> {
    let conn = Connection::open(file_path)?;

    let filename = Path::new(csv_path).file_stem().unwrap().to_str().unwrap();

    println!("Filename: {}", filename);

    let csv_file = fs::read_to_string(csv_path);
    let mut csv_array = Vec::new();
    for line in csv_file.unwrap().lines() {
        let items = line.split(",");
        let mut line_array = Vec::new();
        for item in items {
            line_array.push(item.to_string());
        }
        csv_array.push(line_array);
    }
    println!("First row: {:?}", csv_array[0]);
    let mut headers = String::new();
    for header in &csv_array[0] {
        headers.push_str(header);
        headers.push_str(" ");
    }
    println!("{}", headers);
    for item in &csv_array[0] {
        println!("Header: {}", item);
    }

    let mut creation_query = String::new();
    creation_query.push_str("CREATE TABLE ");
    creation_query.push_str(filename);
    creation_query.push_str(" (\n");
    for (i, item) in csv_array[0].iter().enumerate() {
        creation_query.push_str("_");
        creation_query.push_str(item);
        creation_query.push_str(" ");
        if item.trim().parse::<i32>().is_ok() {
            creation_query.push_str("INTEGER NOT NULL");
        } else {
            creation_query.push_str("TEXT NOT NULL");
        }
        if i < csv_array[0].len() - 1 {
            creation_query.push_str(",\n");
        }
    }
    creation_query.push_str("\n)");

    println!("Query: {}", creation_query);
    /*
    *
    "CREATE TABLE another_test (
        {0},
        name TEXT NOT NULL,
        age INTEGER NOT NULL,
        email TEXT NOT NULL
    )";
    */

    conn.execute(creation_query.as_str(), ())?;

    for item in csv_array.iter().skip(1) {
        let mut insert_query = String::new();
        insert_query.push_str("INSERT INTO ");
        insert_query.push_str(filename);
        insert_query.push_str(" VALUES (");
        for (i, value) in item.iter().enumerate() {
            insert_query.push_str(value);
            if i < item.len() - 1 {
                insert_query.push_str(", ");
            }
        }
        insert_query.push_str(")");
        conn.execute(insert_query.as_str(), ())?;
    }

    Ok(())
}
pub fn remove_table(file_path: &str, table_name: &str) -> Result<()> {
    let conn = Connection::open(file_path)?;
    let removal_query = format!("DROP TABLE {}", table_name);
    conn.execute(removal_query.as_str(), ())?;
    Ok(())
}

/*
pub fn add_dummy_row(file_path: &str, name) -> Result<()> {
     let conn = Connection::open(file_path)?;
     lent
 }
 */
