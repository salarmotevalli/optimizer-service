package httpserver

import (
	"errors"
	"log/slog"
	"net/http"
	"user/config"
	"user/delivery/httpserver/authorizationhandler"
	"user/delivery/httpserver/imagehandler"
	internalMiddleware "user/delivery/httpserver/middleware"
	"user/delivery/httpserver/userhandler"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

type Server struct {
	config       config.Config
	userHandler  userhandler.UserHandler
	authorizationHandler authorizationhandler.AuthorizationHandler
	imageHandler imagehandler.ImageHandler
}

func New(cnf config.Config,
	uh userhandler.UserHandler,
	ah authorizationhandler.AuthorizationHandler,
	ih imagehandler.ImageHandler) Server {
	return Server{
		config:       cnf,
		userHandler:  uh,
		authorizationHandler:  ah,
		imageHandler:  ih,
	}
}

func (s Server) Serve() {
	e := echo.New()

	// Middleware
	e.Use(middleware.Logger())
	e.Use(middleware.Recover())

	e.GET("health-check", s.healthCheck)

	// auth
	userGroup := e.Group("/users")
	userGroup.POST("/register", s.userHandler.Register)
	userGroup.POST("/login", s.userHandler.Login)

	imageGroup := e.Group("/images", internalMiddleware.Auth(s.userHandler.AuthSvc, s.config.AuthConfig))
	imageGroup.GET("/optimize/sign-url", s.imageHandler.SignUrl)
	imageGroup.POST("/authorize/file-upload", s.authorizationHandler.AuthorizeFileUploadAction)

	if err := e.Start(":8080"); err != nil && !errors.Is(err, http.ErrServerClosed) {
		slog.Error("failed to start server", "error", err)
	}
}
