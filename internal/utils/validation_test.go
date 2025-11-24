package utils

import (
	"archive/zip"
	"os"
	"path/filepath"
	"testing"
)

func TestValidateJarFile(t *testing.T) {
	tmpDir := t.TempDir()

	err := ValidateJarFile(filepath.Join(tmpDir, "nonexistent.jar"))
	if err == nil {
		t.Error("expected error for nonexistent file")
	}

	emptyJar := filepath.Join(tmpDir, "empty.jar")
	if err := os.WriteFile(emptyJar, []byte{}, 0644); err != nil {
		t.Fatal(err)
	}
	err = ValidateJarFile(emptyJar)
	if err == nil {
		t.Error("expected error for empty file")
	}

	invalidJar := filepath.Join(tmpDir, "invalid.jar")
	if err := os.WriteFile(invalidJar, []byte{0x00, 0x00, 0x00, 0x00, 0x00}, 0644); err != nil {
		t.Fatal(err)
	}
	err = ValidateJarFile(invalidJar)
	if err == nil {
		t.Error("expected error for invalid magic number")
	}

	validJar := filepath.Join(tmpDir, "valid.jar")
	createValidJar(t, validJar)
	err = ValidateJarFile(validJar)
	if err != nil {
		t.Errorf("unexpected error for valid JAR: %v", err)
	}
}

func TestValidateJarAndCalculateChecksum(t *testing.T) {
	tmpDir := t.TempDir()
	jarPath := filepath.Join(tmpDir, "test.jar")
	createValidJar(t, jarPath)

	checksum, err := ValidateJarAndCalculateChecksum(jarPath)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(checksum) != 64 {
		t.Errorf("expected 64-char checksum, got %d", len(checksum))
	}
}

func TestValidateChecksum(t *testing.T) {
	tmpDir := t.TempDir()
	testFile := filepath.Join(tmpDir, "test.jar")
	createValidJar(t, testFile)

	checksum, err := ValidateJarAndCalculateChecksum(testFile)
	if err != nil {
		t.Fatal(err)
	}

	err = ValidateChecksum(testFile, checksum)
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}

	err = ValidateChecksum(testFile, "0000000000000000000000000000000000000000000000000000000000000000")
	if err == nil {
		t.Error("expected error for invalid checksum")
	}

	err = ValidateChecksum(testFile, "")
	if err != nil {
		t.Errorf("unexpected error for empty checksum: %v", err)
	}
}

func TestLoadAndSaveChecksumFile(t *testing.T) {
	tmpDir := t.TempDir()
	checksumPath := filepath.Join(tmpDir, "test.sha256")
	expectedChecksum := "6ae8a75555209fd6c44157c0aed8016e763ff435a19cf186f76863140143ff72"

	err := SaveChecksumFile(checksumPath, expectedChecksum)
	if err != nil {
		t.Fatalf("failed to save: %v", err)
	}

	loaded, err := LoadChecksumFile(checksumPath)
	if err != nil {
		t.Fatalf("failed to load: %v", err)
	}

	if loaded != expectedChecksum {
		t.Errorf("expected %s, got %s", expectedChecksum, loaded)
	}
}

func TestLoadChecksumFileInvalid(t *testing.T) {
	tmpDir := t.TempDir()

	shortChecksum := filepath.Join(tmpDir, "short.sha256")
	if err := os.WriteFile(shortChecksum, []byte("short"), 0644); err != nil {
		t.Fatal(err)
	}
	_, err := LoadChecksumFile(shortChecksum)
	if err == nil {
		t.Error("expected error for short checksum")
	}

	invalidHex := filepath.Join(tmpDir, "invalid.sha256")
	if err := os.WriteFile(invalidHex, []byte("gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg"), 0644); err != nil {
		t.Fatal(err)
	}
	_, err = LoadChecksumFile(invalidHex)
	if err == nil {
		t.Error("expected error for invalid hex checksum")
	}

	nonexistent := filepath.Join(tmpDir, "nonexistent.sha256")
	loaded, err := LoadChecksumFile(nonexistent)
	if err != nil {
		t.Errorf("unexpected error for nonexistent file: %v", err)
	}
	if loaded != "" {
		t.Errorf("expected empty string, got %s", loaded)
	}
}

func createValidJar(t *testing.T, path string) {
	file, err := os.Create(path)
	if err != nil {
		t.Fatal(err)
	}
	defer func() {
		if err := file.Close(); err != nil {
			t.Errorf("failed to close file: %v", err)
		}
	}()

	w := zip.NewWriter(file)
	defer func() {
		if err := w.Close(); err != nil {
			t.Errorf("failed to close zip writer: %v", err)
		}
	}()

	f, err := w.Create("META-INF/MANIFEST.MF")
	if err != nil {
		t.Fatal(err)
	}
	if _, err := f.Write([]byte("Manifest-Version: 1.0\n")); err != nil {
		t.Fatal(err)
	}

	f, err = w.Create("test.txt")
	if err != nil {
		t.Fatal(err)
	}
	if _, err := f.Write([]byte("test content")); err != nil {
		t.Fatal(err)
	}
}
