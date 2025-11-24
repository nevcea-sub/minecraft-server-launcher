package backup

import (
	"os"
	"path/filepath"
	"testing"
)

func TestFilterExistingWorlds(t *testing.T) {
	tmpDir := t.TempDir()

	world1 := filepath.Join(tmpDir, "world")
	world2 := filepath.Join(tmpDir, "world_nether")

	if err := os.MkdirAll(world1, 0755); err != nil {
		t.Fatal(err)
	}
	if err := os.MkdirAll(world2, 0755); err != nil {
		t.Fatal(err)
	}

	oldDir, err := os.Getwd()
	if err != nil {
		t.Fatal(err)
	}
	defer func() {
		if err := os.Chdir(oldDir); err != nil {
			t.Errorf("failed to restore directory: %v", err)
		}
	}()
	if err := os.Chdir(tmpDir); err != nil {
		t.Fatal(err)
	}

	result := filterExistingWorlds([]string{"world", "world_nether", "world_the_end"})

	if len(result) != 2 {
		t.Errorf("expected 2 worlds, got %d", len(result))
	}

	if result[0] != "world" && result[0] != "world_nether" {
		t.Errorf("unexpected world: %s", result[0])
	}
	if result[1] != "world" && result[1] != "world_nether" {
		t.Errorf("unexpected world: %s", result[1])
	}
}
