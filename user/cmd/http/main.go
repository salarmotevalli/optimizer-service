package main

import (
	adapteroptimizer "getway/adapter/adapter-optimizer"
	"getway/config"
	"getway/delivery/httpserver"
	"getway/delivery/httpserver/imagehandler"
	"getway/delivery/httpserver/userhandler"
	"getway/repository/mysql"
	"getway/repository/mysql/mysqluser"
	"getway/service/authservice"
	"getway/service/imageservice"
	"getway/service/userservice"
	"getway/validator/imagevalidator"
)

func main() {
	cnf := config.Load("./config.yml")

	authSvc, userSvc, imageSvc:= services(cnf)
	uh, ih := handlers(userSvc, authSvc, imageSvc)

	hserver := httpserver.New(cnf, uh, ih)
		
	hserver.Serve()
}

func handlers(us userservice.UserService, as authservice.AuthService, is imageservice.ImageService) (userhandler.UserHandler, imagehandler.ImageHandler) {
	uh := userhandler.New(us, as)

	iv := imagevalidator.Validator{}
	ih := imagehandler.New(is, iv)
	return uh, ih
}

func services(cnf config.Config) (authservice.AuthService, userservice.UserService, imageservice.ImageService) {
	mysql := mysql.New()
	userRepo := mysqluser.New(mysql)
	
	authSvc := authservice.New(cnf.AuthConfig)
	userSvc := userservice.New(userRepo, authSvc)
	

	optimizer := adapteroptimizer.New(cnf.OptimizerConfig)
	
	imageSvc := imageservice.New(optimizer)
	

	return authSvc, userSvc, imageSvc
}
