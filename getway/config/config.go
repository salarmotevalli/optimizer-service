package config

import "getway/service/authservice"

type HttpServer struct {
	Port int
}

type Config struct {
	HttpServer HttpServer
	AuthConfig authservice.Config
}
