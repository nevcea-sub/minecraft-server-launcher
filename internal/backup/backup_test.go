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
	
	os.MkdirAll(world1, 0755)
	os.MkdirAll(world2, 0755)
	
	oldDir, _ := os.Getwd()
	defer os.Chdir(oldDir)
	os.Chdir(tmpDir)
	
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

