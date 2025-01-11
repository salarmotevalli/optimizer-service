package adapteroptimizer

import (
	"bytes"
	"encoding/json"
	"fmt"
	"getway/param/imageparam"
	"net/http"
)

type OptimizerConfig struct {
	OptimizerUrl string
}

type Optimizer struct {
	config OptimizerConfig
}

func New(c OptimizerConfig) Optimizer {
	return Optimizer {config: c}
}


func (o *Optimizer) SignUrl(param imageparam.SignUrlRequest) (string, error) {
	signUrl := fmt.Sprintf("%s/%s", o.config.OptimizerUrl, "sign-url") 
	
	body, err := json.Marshal(param)
	if err != nil {
		return "", err
	}

	res, rErr := http.Post(signUrl, "Content-Type: application/json", bytes.NewReader(body))

	var resBody []byte 
	res.Body.Read(resBody)

	return string(resBody), rErr
}