package main

import (
    "fmt"
    "io"
    "log"
    "net/http"
    "os"
    "strconv"

    "github.com/PuerkitoBio/goquery"
)

const HOME_STORAGE_DIR = "specimen_storage"
const MALCODE_DIR = "malcode"
const MALCODE_BASE_URL = "http://malc0de.com/database/"

func getFromMalcode() {
    fmt.Println("####################################")
    fmt.Println("Brought to you by malc0de")
    fmt.Println("####################################")
    fmt.Println()
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
            fmt.Printf("Download malware from %s\n", url)

            res, err := http.Get(url)
            if err != nil {
                fmt.Printf("[NG] %s\n", err)
                goto Fin
            }
            defer res.Body.Close()
            fmt.Printf("[%s]\n", res.Status)

            if res.StatusCode != 404 {
                filehash := goquery.NewDocumentFromNode(s.Find("td").Get(6)).Find("a").Text()
                out, err := os.Create(filehash)
                if err != nil {
                    panic(err)
                }
                defer out.Close()
                fmt.Printf("[OK] Filehash is %s\n", filehash)

                io.Copy(out, res.Body)
                filepath := "../" + HOME_STORAGE_DIR + "/" + MALCODE_DIR + "/" + filehash
                if err := os.Rename(filehash, filepath); err != nil {
                    panic(err)
                }
                fmt.Printf("[OK] Filepath is %s\n", filepath)
            }
            Fin:
            fmt.Println()
        })
    }
}

func main() {
    getFromMalcode()
}

