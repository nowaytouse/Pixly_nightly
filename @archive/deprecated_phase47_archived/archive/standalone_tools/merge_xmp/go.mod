module merge_xmp

go 1.25.3

replace pixly/utils => ../utils

replace pixly/pkg/validation => ../../pkg/validation

require (
	github.com/karrick/godirwalk v1.17.0
	pixly/pkg/validation v0.0.0-00010101000000-000000000000
	pixly/utils v0.0.0-00010101000000-000000000000
)

require (
	github.com/h2non/filetype v1.1.3 // indirect
	go.uber.org/multierr v1.10.0 // indirect
	go.uber.org/zap v1.27.0 // indirect
	pixly/pkg/utils v0.0.0 // indirect
	pixly/pkg/utils/metadata v0.0.0 // indirect
)

replace pixly/pkg/utils/metadata => ../../pkg/utils/metadata

replace pixly/pkg/utils => ../../pkg/utils
