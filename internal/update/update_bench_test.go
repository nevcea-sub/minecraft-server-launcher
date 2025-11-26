package update

import (
	"fmt"
	"runtime"
	"testing"
)

func BenchmarkNormalizeVersion(b *testing.B) {
	versions := []string{
		"v1.0.0",
		"1.0.0",
		" 1.0.0 ",
		"v2.1.3",
		"3.0",
		"v10.20.30",
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		for _, v := range versions {
			_ = normalizeVersion(v)
		}
	}
}

func BenchmarkCompareVersions(b *testing.B) {
	versionPairs := [][]string{
		{"1.0.0", "1.0.0"},
		{"1.0.1", "1.0.0"},
		{"1.1.0", "1.0.0"},
		{"2.0.0", "1.0.0"},
		{"1.2.3", "1.2.4"},
		{"2.0.0", "1.9.9"},
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		for _, pair := range versionPairs {
			_ = compareVersions(pair[0], pair[1])
		}
	}
}

func BenchmarkGetAssetForCurrentOS(b *testing.B) {
	release := &ReleaseResponse{
		Assets: []Asset{
			{Name: "paper-launcher-windows-amd64.exe"},
			{Name: "paper-launcher-linux-amd64"},
			{Name: "paper-launcher-darwin-amd64"},
			{Name: "paper-launcher-windows-arm64.exe"},
			{Name: "paper-launcher-linux-arm64"},
			{Name: "paper-launcher-darwin-arm64"},
		},
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_ = getAssetForCurrentOS(release)
	}
}

func BenchmarkGetAssetForCurrentOS_ManyAssets(b *testing.B) {
	assets := make([]Asset, 0, 50)
	for i := 0; i < 50; i++ {
		assets = append(assets, Asset{
			Name: fmt.Sprintf("asset-%d", i),
		})
	}

	osName := runtime.GOOS
	arch := runtime.GOARCH
	var targetName string
	switch osName {
	case "windows":
		switch arch {
		case "amd64":
			targetName = "paper-launcher-windows-amd64.exe"
		case "arm64":
			targetName = "paper-launcher-windows-arm64.exe"
		}
	case "linux":
		switch arch {
		case "amd64":
			targetName = "paper-launcher-linux-amd64"
		case "arm64":
			targetName = "paper-launcher-linux-arm64"
		}
	case "darwin":
		switch arch {
		case "amd64":
			targetName = "paper-launcher-darwin-amd64"
		case "arm64":
			targetName = "paper-launcher-darwin-arm64"
		}
	}

	if targetName != "" {
		assets = append(assets, Asset{Name: targetName})
	}

	release := &ReleaseResponse{
		Assets: assets,
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_ = getAssetForCurrentOS(release)
	}
}

func BenchmarkGetCurrentVersion(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = GetCurrentVersion()
	}
}
