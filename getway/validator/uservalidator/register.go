package uservalidator

import (
	"getway/param/userparam"
	"getway/pkg/errmsg"
	"getway/pkg/richerror"

	validation "github.com/go-ozzo/ozzo-validation/v4"
)

func (v Validator) Register(req userparam.RegisterUserRequest) (map[string]string, error) {
	const op = "uservalidator.Login"

	err := validation.ValidateStruct(&req,
		validation.Field(&req.Username, validation.Required, validation.Length(3, 50)),
		validation.Field(&req.Password, validation.Required, validation.Length(8, 100)),
	)

	if err != nil {
		fieldErrors := make(map[string]string)

		errV, ok := err.(validation.Errors)
		if ok {
			for key, value := range errV {
				if value != nil {
					fieldErrors[key] = value.Error()
				}
			}
		}

		return fieldErrors, richerror.New(op).WithMessage(errmsg.ErrorMsgInvalidInput).
			WithKind(richerror.KindInvalid).
			WithMeta(map[string]interface{}{"req": req}).WithErr(err)
	}

	return nil, nil
}
