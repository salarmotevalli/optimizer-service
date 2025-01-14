package authorizationservice

import (
	"context"
	"fmt"
	"user/param/authorizationparam"
	"user/pkg/richerror"
)

type Repo interface {
	GetTotalFileUploadCount(string) (uint16, error)
	GetSuccessfulFileUploadCount(string) (uint16, error)
	IncreamentTotalFileUploadCount(string) error
}

type Config struct{
	Prefix string
	MaxSuccessfulRequestPerDay 	uint16 
  	MaxTotalRequestPerDay 		uint16
}

type AuthorizationService struct {
	repo Repo
	config Config
}

func New(c Config) AuthorizationService {
	return AuthorizationService {config: c}
}

func (s *AuthorizationService)totalKey(id uint) string {
	return fmt.Sprintf("%s:total:%d", s.config.Prefix, id)
}

func (s *AuthorizationService)successKey(id uint) string {
	return fmt.Sprintf("%s:success:%d", s.config.Prefix, id)
}

func (s *AuthorizationService) AuthorizeFileUploadAction(ctx context.Context, 
	req authorizationparam.AuthorizeActionRequest) (authorizationparam.AuthorizeActionResponse, error) {

	err := s.authorizeUploadAction(req.UserID)
	if err != nil {
		return authorizationparam.AuthorizeActionResponse{}, err
	}

	_ = s.IncreamentTotalCount(req.UserID)

	return authorizationparam.AuthorizeActionResponse{}, nil
}

func (s *AuthorizationService)authorizeUploadAction(userId uint) error {
	const op = "authorizationservice.authorizeUploadAction"
	// check count total and success
	
	res, sErr := s.repo.GetSuccessfulFileUploadCount(s.successKey(userId))
	if sErr != nil {
		return richerror.New(op).WithKind(richerror.KindUnexpected).WithErr(sErr)
	}
	
	if res >= s.config.MaxSuccessfulRequestPerDay{
		return richerror.New(op).WithKind(richerror.KindForbidden)
	}

	res, tErr := s.repo.GetTotalFileUploadCount(s.successKey(userId))
	if tErr != nil {
		return richerror.New(op).WithKind(richerror.KindUnexpected).WithErr(tErr)
	}
	
	if res >= s.config.MaxSuccessfulRequestPerDay{
		return richerror.New(op).WithKind(richerror.KindForbidden)
	}

	return nil
}

func (s *AuthorizationService)IncreamentTotalCount(id uint) error {
	return s.repo.IncreamentTotalFileUploadCount(s.totalKey(id))
}

