package richerror

type Kind uint16

const (
	KindInvalid Kind = iota + 1
	KindForbidden
	KindNotFound
	KindUnexpected
	KindInvalidToken
)

type Op string

type RichError struct {
	wrappedError error
	message      string
	kind         Kind
	operation    Op
	meta         map[string]interface{}
}

func New(op Op) RichError {
	return RichError{operation: op}
}

func (r RichError) WithOp(op Op) RichError {
	r.operation = op
	return r
}

func (r RichError) WithErr(err error) RichError {
	r.wrappedError = err
	return r
}

func (r RichError) WithMessage(message string) RichError {
	r.message = message
	return r
}

func (r RichError) WithKind(kind Kind) RichError {
	r.kind = kind
	return r
}

func (r RichError) WithMeta(meta map[string]interface{}) RichError {
	if r.meta == nil {
		r.meta = make(map[string]interface{}, len(meta))
	}

	for k, v := range meta {
		r.meta[k] = v
	}

	return r
}

func (r RichError) Error() string {
	if r.message == "" && r.wrappedError != nil {
		return r.wrappedError.Error()
	}

	return r.message
}

func (r RichError) Kind() Kind {
	if r.kind != 0 {
		return r.kind
	}

	re, ok := r.wrappedError.(RichError)
	if !ok {
		return 0
	}

	return re.Kind()
}

func (r RichError) Message() string {
	if r.message != "" {
		return r.message
	}

	re, ok := r.wrappedError.(RichError)
	if ok {
		return re.Message()
	}

	if r.wrappedError != nil {
		return r.wrappedError.Error()
	}

	return ""
}
