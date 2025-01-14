package adapteroptimizer

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"user/param/imageparam"
	"user/pkg/richerror"
	"io"
	"net/http"
)

type OptimizerConfig struct {
	Url string `koanf:"url"`
}

type Optimizer struct {
	Config OptimizerConfig
}

func New(c OptimizerConfig) Optimizer {
	return Optimizer {Config: c}
}

func (o Optimizer) SignUrlToken(param imageparam.SignUrlRequest) (string, error) {
	const op = "adapteroptimizer.SignUrl"
	
	signUrl := fmt.Sprintf("%s/%s", o.Config.Url, "opt/sign-url-token") 
	
	body, err := json.Marshal(param)
	if err != nil {
		return "", err
	}

	res, pErr := http.Post(signUrl, "application/json", bytes.NewReader(body))
	if pErr != nil {
		return "", richerror.New(op).WithErr(pErr).WithKind(richerror.KindUnexpected)
	}

	resBody, ioErr := io.ReadAll(res.Body)
	if ioErr != nil {
		return "", richerror.New(op).WithErr(ioErr).WithKind(richerror.KindUnexpected)
	}


    var dat map[string]interface{}

	umErr := json.Unmarshal(resBody, &dat)
	if umErr != nil {
		return "", richerror.New(op).WithErr(umErr).WithKind(richerror.KindUnexpected)
	}

	strConverted, cErr := dat["token"].(string)

	if !cErr {
		return "", richerror.New(op).WithErr(errors.New("couldn't convert token to string")).WithKind(richerror.KindUnexpected)
	}

	return strConverted, nil
}