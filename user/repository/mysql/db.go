package mysql

import (
	"database/sql"
	"log"
	"time"

	_ "github.com/go-sql-driver/mysql"
)

type MySQLDB struct {
	db *sql.DB
}

func (m *MySQLDB) Conn() *sql.DB {
	return m.db
}

func New() *MySQLDB {
	db, err := sql.Open("mysql", "goevent:abc123@tcp(0.0.0.0:3306)/goevent?parseTime=true")
	if err != nil {
		log.Fatal("couldn't connect to Mysql")
	}

	db.SetConnMaxLifetime(time.Minute * 3)
	db.SetMaxOpenConns(10)
	db.SetMaxIdleConns(10)

	return &MySQLDB{
		db: db,
	}
}
