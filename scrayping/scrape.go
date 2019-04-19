package main

import (
    "fmt"
    "io/ioutil"
    "log"
    "net/http"
    "os"

    "github.com/PuerkitoBio/goquery"
)

const HOME_STORAGE_DIR = "specimen_storage"
const MALCODE_DIR = "malcode"
const MALCODE_URL = "http://malc0de.com"

func getFromMalcode() {
    fmt.Println("####################################")
    fmt.Println("Brought to you by malc0de")
    fmt.Println("####################################")
    res, err := http.Get(MALCODE_URL)
    if err != nil {
        panic(err)
    }
    defer res.Body.Close()
    if res.StatusCode != 200 {
        log.Fatalf("status code error: %d %s", res.StatusCode, res.Status)
    }

    doc, err := goquery.NewDocumentFromReader(res.Body)
    if err != nil {
        log.Fatal(err)
    }

    doc.Find("table.prettytable > tr").Each(func(i int, s *goquery.Selection) {
        url := s.Find("td")[1].Text()
        fmt.Println("####################################")
        fmt.Printf("Download malware from %s\n", url)
        fmt.Println("####################################")
        res, err := http.Get(url)
        if err != nil {
            panic(err)
        }
        defer res.Body.Close()

        hash, err := s.Find("td")[6].Find("a").Text()
        out, err := os.Create(hash)
        if err != nil {
            panic(err)
        }
        defer out.Close()

        file := "../" + HOME_STORAGE_DIR + "/" + MALCODE_DIR + "/" + out
        ioutil.WriteFile(file, []byte(res.Body), 0666)
    })
}

