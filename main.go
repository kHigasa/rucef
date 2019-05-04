package main

import (
    "database/sql"
    "fmt"
    _ "github.com/lib/pq"
)

func main() {
    // connect to db
    db, err := sql.Open("postgres", "host=127.0.0.1 port=5432 user=khigasa password=zdcgbjmlp878 dbname=rucef sslmode=disable")
    if err != nil {
        fmt.Println("failure on Open func")
    }

    err = db.Ping()
    if err != nil {
        fmt.Println("failure on Ping func")
    }

    defer db.Close()
}

