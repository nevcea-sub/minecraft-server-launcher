package server

import (
	"testing"
)

func TestCalculateSmartRAM(t *testing.T) {
	res := CalculateSmartRAM(0, 85, 2)
	if res < 2 {
		t.Errorf("CalculateSmartRAM returned %d, expected >= 2", res)
	}
}

func TestExtractJavaVersion(t *testing.T) {
	tests := []struct {
		name   string
		output string
		want   string
	}{
		{
			name:   "java 17",
			output: `openjdk version "17.0.2" 2022-01-18`,
			want:   "17.0.2",
		},
		{
			name:   "java 21",
			output: `openjdk version "21.0.1" 2023-10-17`,
			want:   "21.0.1",
		},
		{
			name:   "unknown",
			output: "some random output",
			want:   "unknown",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := extractJavaVersion(tt.output)
			if got != tt.want {
				t.Errorf("extractJavaVersion() = %s, want %s", got, tt.want)
			}
		})
	}
}

func TestParseJavaVersion(t *testing.T) {
	tests := []struct {
		name      string
		version   string
		want      int
		wantError bool
	}{
		{
			name:      "java 17",
			version:   "17.0.2",
			want:      17,
			wantError: false,
		},
		{
			name:      "java 21",
			version:   "21.0.1",
			want:      21,
			wantError: false,
		},
		{
			name:      "java 8",
			version:   "1.8.0_352",
			want:      8,
			wantError: false,
		},
		{
			name:      "java 11",
			version:   "11.0.19",
			want:      11,
			wantError: false,
		},
		{
			name:      "invalid format",
			version:   "invalid",
			want:      0,
			wantError: true,
		},
		{
			name:      "empty string",
			version:   "",
			want:      0,
			wantError: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := parseJavaVersion(tt.version)
			if (err != nil) != tt.wantError {
				t.Errorf("parseJavaVersion() error = %v, wantError %v", err, tt.wantError)
				return
			}
			if got != tt.want {
				t.Errorf("parseJavaVersion() = %d, want %d", got, tt.want)
			}
		})
	}
}
