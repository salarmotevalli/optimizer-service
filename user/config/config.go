package config

import (
	adapteroptimizer "user/adapter/adapter-optimizer"
	"user/service/authorizationservice"
	"user/service/authservice"
)

type HttpServer struct {
	Port int
}

type Config struct {
	HttpServer 		HttpServer `koanf:"http_server"`
	AuthConfig 		authservice.Config `koanf:"auth_config"`
	AuthorizationConfig 		authorizationservice.Config `koanf:"authorization_config"`
	OptimizerConfig adapteroptimizer.OptimizerConfig `koanf:"optimizer_config"`
}
