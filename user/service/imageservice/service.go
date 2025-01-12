package imageservice

import (
	"getway/param/imageparam"
	"getway/pkg/richerror"
)

type ImageRepository interface {

}

type OptimizerService interface {
	SignUrlToken(param imageparam.SignUrlRequest) (string, error)
}

type ImageService struct {
	optimizer OptimizerService
}

func New(os OptimizerService) ImageService {
	return ImageService{optimizer: os}
}

func (s ImageService) SignUrl(req imageparam.SignUrlRequest) (imageparam.SignUrlResponse, error) {
	const op = "imageservice.SignUrl"
	
	token, err := s.optimizer.SignUrlToken(req)
	if err != nil {
		return imageparam.SignUrlResponse{}, richerror.New(op).
		WithErr(err).
		WithKind(richerror.KindUnexpected)
	}
	
	return imageparam.SignUrlResponse {
		Token: token,
	}, nil
}