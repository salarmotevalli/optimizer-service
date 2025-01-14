package middleware

import (
	cfg "user/config"
	"user/service/authservice"
	mw "github.com/labstack/echo-jwt/v4"
	"github.com/labstack/echo/v4"
)

func Auth(service authservice.AuthService, config authservice.Config) echo.MiddlewareFunc {
	return mw.WithConfig(mw.Config{
		ContextKey: cfg.AuthMiddlewareContextKey,
		SigningKey: []byte(config.SignKey),
		// TODO - as sign method string to config...
		SigningMethod: "HS256",
		ParseTokenFunc: func(c echo.Context, auth string) (interface{}, error) {
			claims, err := service.VerifyToken(auth)
			if err != nil {
				return nil, err
			}

			return claims, nil
		},
	})
}
