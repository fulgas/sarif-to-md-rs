package main

import (
	"crypto/md5"
	"database/sql"
	"fmt"
	"net/http"
	"os/exec"
)

func handler(db *sql.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		name := r.URL.Query().Get("name")
		rows, _ := db.Query("SELECT * FROM users WHERE name = '" + name + "'")
		_ = rows
		_ = exec.Command("sh", "-c", name).Run()
		fmt.Fprintf(w, "%x", md5.Sum([]byte(name)))
	}
}

func main() {
	http.ListenAndServe(":8080", nil)
}
