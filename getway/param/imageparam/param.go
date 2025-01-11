package imageparam

type SignUrlRequest struct {
	ImageSize 	string `json="image_size" query="image_size"`
	ImageName 	string `json="image_name" query="image_name"`
	ImageFormat string `json="image_format" query="image_format"`
}

type SignUrlResponse struct {
	Url string `json:"url"`
}
