package main

import (
	"getway/config"
	"getway/delivery/httpserver"
	"getway/delivery/httpserver/userhandler"
	"getway/repository/mysql"
	"getway/repository/mysql/mysqluser"
	"getway/service/authservice"
	"getway/service/userservice"
)

func main() {
	cnf := getConfig()

	authSvc, userSvc:= services(cnf)
	uh := handlers(userSvc, authSvc)
	hserver := httpserver.New(cnf, uh)

	hserver.Serve()
}

func handlers(us userservice.UserService, as authservice.AuthService) (userhandler.UserHandler) {
	uh := userhandler.New(us, as)
	
	return uh
}

func services(cnf config.Config) (authservice.AuthService, userservice.UserService) {
	mysql := mysql.New()
	userRepo := mysqluser.New(mysql)
	
	authSvc := authservice.New(cnf.AuthConfig)
	userSvc := userservice.New(userRepo, authSvc)
	
	return authSvc, userSvc
}

func getConfig() config.Config {
	return config.Config{
		HttpServer: config.HttpServer{Port: 8080},
		AuthConfig: authservice.Config{
			SignKey:               config.JwtSignKey,
			AccessSubject:         config.AccessTokenSubject,
			RefreshSubject:        config.RefreshTokenSubject,
			AccessExpirationTime:  config.AccessTokenExpireDuration,
			RefreshExpirationTime: config.RefreshTokenExpireDuration,
		},
	}
}
