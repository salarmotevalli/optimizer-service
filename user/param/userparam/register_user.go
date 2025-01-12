package userparam

type RegisterUserRequest struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

type RegisterUserResponse struct {
	User UserInfo `json:"user"`
}
