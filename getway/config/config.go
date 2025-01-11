package config

import (
	adapteroptimizer "getway/adapter/adapter-optimizer"
	"getway/service/authservice"
)

type HttpServer struct {
	Port int
}

type Config struct {
	HttpServer 		HttpServer `koanf:"http_server"`
	AuthConfig 		authservice.Config `koanf:"auth_config"`
	OptimizerConfig adapteroptimizer.OptimizerConfig `koanf:"optimizer_config"`
}
