package imagehandler

import (
	"getway/param/imageparam"
	"getway/pkg/httpmsg"
	"getway/service/imageservice"
	"getway/validator/imagevalidator"
	"net/http"

	"github.com/labstack/echo/v4"
)

type ImageHandler struct {
	imageSvc imageservice.ImageService
	imageValidator imagevalidator.Validator
}

func New(s imageservice.ImageService, iv imagevalidator.Validator) ImageHandler {
	return ImageHandler{imageSvc: s, imageValidator: iv}
}

func (h ImageHandler) SignUrl(c echo.Context) error {
	var req imageparam.SignUrlRequest

	if err := c.Bind(&req); err != nil {
		msg, code := httpmsg.Error(err)
		return c.JSON(code, echo.Map{
			"message": msg})
	}

	if fieldErrors, err := h.imageValidator.ValidateSignUrlRequest(req); err != nil {
		msg, code := httpmsg.Error(err)
		return c.JSON(code, echo.Map{
			"message": msg,
			"errors":  fieldErrors,
		})
	}

	result, err := h.imageSvc.SignUrl(req)
	if err != nil {
		msg, code := httpmsg.Error(err)
		return c.JSON(code, echo.Map{
			"message": msg})
	}

	return c.JSON(http.StatusAccepted, result)
}
