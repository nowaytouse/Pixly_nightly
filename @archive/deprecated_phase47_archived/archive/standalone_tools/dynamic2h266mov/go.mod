module dynamic2h266mov

go 1.25.3

require (
	github.com/karrick/godirwalk v1.17.0
	pixly/utils v0.0.0-00010101000000-000000000000
)

require github.com/h2non/filetype v1.1.3 // indirect

replace (
	github.com/pixly/archive/shared => ../shared
	pixly/pkg/i18n => ../../pkg/i18n
	pixly/pkg/utils => ../../pkg/utils
	pixly/pkg/utils/metadata => ../../pkg/utils/metadata
	pixly/utils => ../utils
)
