package main

import (
	adapteroptimizer "user/adapter/adapter-optimizer"
	"user/config"
	"user/delivery/httpserver"
	"user/delivery/httpserver/authorizationhandler"
	"user/delivery/httpserver/imagehandler"
	"user/delivery/httpserver/userhandler"
	"user/repository/mysql"
	"user/repository/mysql/mysqluser"
	"user/service/authorizationservice"
	"user/service/authservice"
	"user/service/imageservice"
	"user/service/userservice"
	"user/validator/imagevalidator"
)

func main() {
	cnf := config.Load("./config.yml")

	authSvc, AuthorizationSvc, userSvc, imageSvc:= services(cnf)
	uh, ah, ih := handlers(userSvc, authSvc, AuthorizationSvc, imageSvc)

	hserver := httpserver.New(cnf, uh, ah, ih)
		
	hserver.Serve()
}

func handlers(us userservice.UserService, as authservice.AuthService, aths authorizationservice.AuthorizationService, is imageservice.ImageService) (userhandler.UserHandler, authorizationhandler.AuthorizationHandler, imagehandler.ImageHandler) {
	uh := userhandler.New(us, as)

	iv := imagevalidator.Validator{}
	ih := imagehandler.New(is, iv)
	ah := authorizationhandler.New(aths)
	return uh, ah, ih
}

func services(cnf config.Config) (authservice.AuthService, authorizationservice.AuthorizationService, userservice.UserService, imageservice.ImageService) {
	mysql := mysql.New()
	userRepo := mysqluser.New(mysql)
	
	authSvc := authservice.New(cnf.AuthConfig)
	userSvc := userservice.New(userRepo, authSvc)

	authorizationSvc := authorizationservice.New(cnf.AuthorizationConfig)

	optimizer := adapteroptimizer.New(cnf.OptimizerConfig)
	
	imageSvc := imageservice.New(optimizer)
	

	return authSvc, authorizationSvc, userSvc, imageSvc
}
