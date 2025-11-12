module media_tools

go 1.25.3

replace (
	pixly/pkg/utils => ../../pkg/utils
	pixly/pkg/utils/metadata => ../../pkg/utils/metadata
	pixly/utils => ../utils
)

require (
	pixly/pkg/utils v0.0.0
	pixly/utils v0.0.0
)

require (
	github.com/h2non/filetype v1.1.3 // indirect
	go.uber.org/multierr v1.10.0 // indirect
	go.uber.org/zap v1.27.0 // indirect
	pixly/pkg/utils/metadata v0.0.0 // indirect
)
