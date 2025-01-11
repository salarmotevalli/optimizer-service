package imageservice

import (
	"getway/param/imageparam"
	"getway/pkg/richerror"
)

type ImageRepository interface {

}

type OptimizerService interface {
	SignUrl(param imageparam.SignUrlRequest) (string, error)
}

type ImageService struct {
	repo ImageRepository
	optimizer OptimizerService
}

func NewImageService(repo ImageRepository) ImageService {
	return ImageService{repo: repo}
}

func (s *ImageService) SignUrl(req imageparam.SignUrlRequest) (imageparam.SignUrlResponse, error) {
	const op = "imageservice.SignUrl"
	
	url, err := s.optimizer.SignUrl(req)
	if err != nil {
		return imageparam.SignUrlResponse{}, richerror.New(op).
		WithErr(err).
		WithKind(richerror.KindUnexpected)
	}
	
	return imageparam.SignUrlResponse {
		Url: url,
	}, nil
}