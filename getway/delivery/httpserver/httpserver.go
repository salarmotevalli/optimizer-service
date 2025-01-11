package httpserver

import (
	"errors"
	"getway/config"
	"getway/delivery/httpserver/imagehandler"
	internalMiddleware "getway/delivery/httpserver/middleware"
	"getway/delivery/httpserver/userhandler"
	"log/slog"
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

type Server struct {
	config       config.Config
	userHandler  userhandler.UserHandler
	imageHandler imagehandler.ImageHandler
}

func New(cnf config.Config,
	uh userhandler.UserHandler) Server {
	return Server{
		config:       cnf,
		userHandler:  uh,
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

	if err := e.Start(":8080"); err != nil && !errors.Is(err, http.ErrServerClosed) {
		slog.Error("failed to start server", "error", err)
	}
}
