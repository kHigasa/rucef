package main

import (
    "database/sql"
    "io"
    "log"
    "net/http"
    "os"
    "strconv"
    "time"

    "github.com/PuerkitoBio/goquery"
    _ "github.com/lib/pq"
)

const HOME_STORAGE_DIR = "specimen_storage"
const MALCODE_DIR = "malcode"
const MALCODE_BASE_URL = "http://malc0de.com/database/"

type Specimen struct {
    id int
    hash string
    src string
    ip string
    url string
    country string
    timestamp timestamp
}

func getFromMalcode() {
    log.Println("####################################")
    log.Println("Brought to you by malc0de")
    log.Println("####################################")
    log.Println()

    for i := 1; i <= 3; i++ {
        page_query := "?&page=" + strconv.Itoa(i)
        res, err := http.Get(MALCODE_BASE_URL + page_query)
        if err != nil {
            log.Fatal(err)
        }
        defer res.Body.Close()
        if res.StatusCode != 200 {
            log.Fatalf("status code error: %d %s", res.StatusCode, res.Status)
        }

        doc, err := goquery.NewDocumentFromReader(res.Body)
        if err != nil {
            log.Fatal(err)
        }

        selector := "font > center > table.prettytable > tbody > tr.class1"
        doc.Find(selector).Each(func(i int, s *goquery.Selection) {
            host := goquery.NewDocumentFromNode(s.Find("td").Get(1)).Text()
            url := "http://" + host
            log.Printf("Download malware from %s\n", url)

            // Even if the site has already shutted down, we continue getting malware from other sites.
            res, err := http.Get(url)
            if err != nil {
                log.Printf("[NG] %s\n", err)
                goto Fin
            }
            defer res.Body.Close()
            log.Printf("[%s]\n", res.Status)

            if res.StatusCode != 404 {
                filehash := goquery.NewDocumentFromNode(s.Find("td").Get(6)).Find("a").Text()
                out, err := os.Create(filehash)
                if err != nil {
                    panic(err)
                }
                defer out.Close()
                log.Printf("[OK] Filehash is %s\n", filehash)

                io.Copy(out, res.Body)
                filepath := "../" + HOME_STORAGE_DIR + "/" + MALCODE_DIR + "/" + filehash
                if err := os.Rename(filehash, filepath); err != nil {
                    panic(err)
                }
                log.Printf("[OK] Filepath is %s\n", filepath)
            }
            Fin:
            log.Println()
        })
    }
}

// func getFromMarshare() {}
// func getFromVxVault() {}

func main() {
    // set logfile
    layout := "2006-01-02_15:04:05"
    logtime := time.Now().Format(layout)
    logfileName := logtime + ".log"
    logfile, err := os.OpenFile(logfileName, os.O_WRONLY|os.O_CREATE, 0666)
    if err != nil {
        log.Fatal(err)
    }
    defer logfile.Close()
    log.SetOutput(io.MultiWriter(logfile, os.Stdout))
    log.SetFlags(log.Ldate | log.Ltime)

    // connect to db
    db, err := sql.Open("postgres", "host=127.0.0.1 port=5432 user=khigasa password=zdcgbjmlp878 dbname=rucef sslmode=disable")
    if err != nil {
        log.Fatal(err)
    }
    defer db.Close()

    // get malware
    getFromMalcode()
    // getFromMalshare()
    // getFromVxVault()

    // move logfile
    logfilePath := "./logs/" + logfileName
    if err := os.Rename(logfileName, logfilePath); err != nil {
        panic(err)
    }
    log.Printf("[OK] Finish writing logs to %s", logfilePath)
}

