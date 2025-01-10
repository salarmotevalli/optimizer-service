package userhandler

import (
	"getway/param/userparam"
	"getway/pkg/httpmsg"
	"getway/service/authservice"
	"getway/service/userservice"
	"net/http"

	"github.com/labstack/echo/v4"
)

type UserHandler struct {
	UserSvc userservice.UserService
	AuthSvc authservice.AuthService
}

func New(us userservice.UserService, as authservice.AuthService) UserHandler {
	return UserHandler{
		UserSvc: us,
		AuthSvc: as,
	}
}

func (h UserHandler) Login(c echo.Context) error {
	var request userparam.LoginRequest
	if err := c.Bind(&request); err != nil {
		msg, code := httpmsg.Error(err)
		return c.JSON(code, echo.Map{
			"message": msg})
	}

	result, err := h.UserSvc.Login(request)
	if err != nil {
		msg, code := httpmsg.Error(err)
		return c.JSON(code, echo.Map{
			"message": msg})
	}

	return c.JSON(http.StatusAccepted, result)
}

func (h UserHandler) Register(c echo.Context) error {
	var request userparam.RegisterUserRequest
	if err := c.Bind(&request); err != nil {
		msg, code := httpmsg.Error(err)
		return c.JSON(code, echo.Map{
			"message": msg})
	}

	result, err := h.UserSvc.Register(request)
	if err != nil {
		msg, code := httpmsg.Error(err)
		return c.JSON(code, echo.Map{
			"message": msg})
	}

	return c.JSON(http.StatusCreated, result)
}
