package imagevalidator

import (
	"user/param/imageparam"
	"user/pkg/errmsg"
	"user/pkg/richerror"

	validation "github.com/go-ozzo/ozzo-validation/v4"
)

func (v Validator) ValidateSignUrlRequest(req imageparam.SignUrlRequest) (map[string]string, error) {
	const op = "uservalidator.Login"

	err := validation.ValidateStruct(&req,
		validation.Field(&req.ImageFormat, validation.Required, validation.Length(2, 5)),
		// todo: read max acceptable byte from config 
		validation.Field(&req.ImageSize, validation.Required, validation.Max(7000)),
		validation.Field(&req.ImageName, validation.Required, validation.Length(5, 100)),
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
