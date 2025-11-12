module video2mov

go 1.23.0

require (
	pixly/pkg/converter v0.0.0
	pixly/pkg/pipeline v0.0.0
	pixly/pkg/protection v0.0.0
	pixly/pkg/scanner v0.0.0
	pixly/pkg/validation v0.0.0
	pixly/utils v0.0.0
)

replace pixly/pkg/converter => ../../pkg/converter

replace pixly/pkg/pipeline => ../../pkg/pipeline

replace pixly/pkg/protection => ../../pkg/protection

replace pixly/pkg/scanner => ../../pkg/scanner

replace pixly/pkg/validation => ../../pkg/validation

replace pixly/utils => ../utils
