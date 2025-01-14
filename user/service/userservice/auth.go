package userservice

import (
	"crypto/md5"
	"encoding/hex"
	"errors"
	"user/entity"
	"user/param/userparam"
	"fmt"
	"log"
)

type UserRepo interface {
	GetUserByUsername(string) (entity.User, bool, error)
	CreateUser(entity.User) (entity.User, error)
}

type AuthService interface {
	CreateAccessToken(entity.User) (string, error)
	CreateRefreshToken(entity.User) (string, error)
}

type UserService struct {
	repo UserRepo
	auth AuthService
}

func New(r UserRepo, as AuthService) UserService {
	return UserService{
		repo: r,
		auth: as,
	}
}

func (s *UserService) Register(req userparam.RegisterUserRequest) (userparam.RegisterUserResponse, error) {
	// check is there username in db
	_, exist, err := s.repo.GetUserByUsername(req.Username)
	if err != nil {
		log.Println(err)
		return userparam.RegisterUserResponse{}, errors.New("unexpected")
	}

	if exist {
		return userparam.RegisterUserResponse{}, errors.New("user already exists")
	}

	// hash the password
	hashedPassword := getMD5Hash(req.Password)

	// create new user in db
	user := entity.User{
		UserName:       req.Username,
		HashedPassword: hashedPassword,
	}

	user, err = s.repo.CreateUser(user)

	return userparam.RegisterUserResponse{
		User: userparam.UserInfo{Username: user.UserName},
	}, err
}

func (s *UserService) Login(req userparam.LoginRequest) (userparam.LoginResponse, error) {
	// check is there username and password in db
	user, exist, err := s.repo.GetUserByUsername(req.Username)
	if err != nil {
		return userparam.LoginResponse{}, errors.New("unexpected")
	}

	if !exist {
		return userparam.LoginResponse{}, errors.New("user not found")
	}

	if user.HashedPassword != getMD5Hash(req.Password) {
		return userparam.LoginResponse{}, errors.New("password is incorrect")
	}

	accessToken, err := s.auth.CreateAccessToken(user)
	if err != nil {
		return userparam.LoginResponse{}, fmt.Errorf("unexpected error: %w", err)
	}

	refreshToken, err := s.auth.CreateRefreshToken(user)
	if err != nil {
		return userparam.LoginResponse{}, fmt.Errorf("unexpected error: %w", err)
	}

	return userparam.LoginResponse{
		User:         userparam.UserInfo{Username: user.UserName},
		AccessToken:  accessToken,
		RefreshToken: refreshToken,
	}, err
}

func getMD5Hash(text string) string {
	hash := md5.Sum([]byte(text))
	return hex.EncodeToString(hash[:])
}
