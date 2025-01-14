package authorizationhandler

import (
	"net/http"
	"user/config"
	"user/param/authorizationparam"
	"user/pkg/httpmsg"
	"user/service/authorizationservice"
	"user/service/authservice"

	"github.com/labstack/echo/v4"
)

type AuthorizationHandler struct {
	service authorizationservice.AuthorizationService
}

func New(as authorizationservice.AuthorizationService) AuthorizationHandler {
	return AuthorizationHandler{service: as}
}

func (h AuthorizationHandler) AuthorizeFileUploadAction(c echo.Context) error {
	authUser := c.Get(config.AuthMiddlewareContextKey).(authservice.Claims)
	
	res, err := h.service.AuthorizeFileUploadAction(c.Request().Context(),
	 authorizationparam.AuthorizeActionRequest{UserID: authUser.UserID})
	 if err != nil {
		msg, code := httpmsg.Error(err)
		return c.JSON(code, echo.Map{
			"message": msg})
	}

	return c.JSON(http.StatusOK, res)
}
