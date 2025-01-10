package authservice

import (
	"getway/entity"
	"getway/pkg/richerror"
	"strings"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

type Config struct {
	SignKey               string
	AccessExpirationTime  time.Duration
	RefreshExpirationTime time.Duration
	AccessSubject         string
	RefreshSubject        string
}

type AuthService struct {
	config Config
}

func New(config Config) AuthService {
	return AuthService{config: config}
}

func (s AuthService) CreateAccessToken(user entity.User) (string, error) {
	return s.createToken(user.ID, s.config.AccessSubject, s.config.AccessExpirationTime)
}

func (s AuthService) CreateRefreshToken(user entity.User) (string, error) {
	return s.createToken(user.ID, s.config.RefreshSubject, s.config.RefreshExpirationTime)
}

func (s AuthService) VerifyToken(bearerToken string) (*Claims, error) {
	const op = "authservice.VerifyToken"

	tokenStr := strings.Replace(bearerToken, "Bearer ", "", 1)

	token, err := jwt.ParseWithClaims(tokenStr, &Claims{}, func(token *jwt.Token) (interface{}, error) {
		return []byte(s.config.SignKey), nil
	}, jwt.WithLeeway(5*time.Second))

	if err != nil {
		return nil, richerror.New(op).WithErr(err).
			WithKind(richerror.KindInvalidToken)
	}

	if claims, ok := token.Claims.(*Claims); ok {
		return claims, nil
	} else {
		return nil, richerror.New(op).WithKind(richerror.KindInvalidToken).
			WithMessage("unknown claims type, cannot proceed")
	}
}

func (s AuthService) createToken(userID uint, subject string, expireDuration time.Duration) (string, error) {
	claims := Claims{
		RegisteredClaims: jwt.RegisteredClaims{
			Subject:   subject,
			ExpiresAt: jwt.NewNumericDate(time.Now().Add(expireDuration)),
		},
		UserID: userID,
	}

	accessToken := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	tokenString, err := accessToken.SignedString([]byte(s.config.SignKey))

	return tokenString, err
}
