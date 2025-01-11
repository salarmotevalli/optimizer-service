package imagehandler

import (
	"getway/param/imageparam"
	"getway/pkg/httpmsg"
	"getway/service/imageservice"
	"net/http"

	"github.com/labstack/echo/v4"
)

type ImageHandler struct {
	ImageSvc imageservice.ImageService
}

func (h ImageHandler) SignUrl(c echo.Context) error {
	var request imageparam.SignUrlRequest
	if err := c.Bind(&request); err != nil {
		msg, code := httpmsg.Error(err)
		return c.JSON(code, echo.Map{
			"message": msg})
	}

	result, err := h.ImageSvc.SignUrl(request)
	if err != nil {
		msg, code := httpmsg.Error(err)
		return c.JSON(code, echo.Map{
			"message": msg})
	}

	return c.JSON(http.StatusAccepted, result)
}
