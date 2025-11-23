package server

import (
	"fmt"
	"os"
	"os/exec"
	"runtime"
	"strings"

	"github.com/shirou/gopsutil/v3/mem"
)

const (
	minJavaVersion = 17
	javaCmd        = "java"
)

var aikarFlags = []string{
	"-XX:+UseG1GC",
	"-XX:+ParallelRefProcEnabled",
	"-XX:MaxGCPauseMillis=200",
	"-XX:+UnlockExperimentalVMOptions",
	"-XX:+DisableExplicitGC",
	"-XX:+AlwaysPreTouch",
	"-XX:G1NewSizePercent=30",
	"-XX:G1MaxNewSizePercent=40",
	"-XX:G1HeapRegionSize=8M",
	"-XX:G1ReservePercent=20",
	"-XX:G1HeapWastePercent=5",
	"-XX:G1MixedGCCountTarget=4",
	"-XX:InitiatingHeapOccupancyPercent=15",
	"-XX:G1MixedGCLiveThresholdPercent=90",
	"-XX:G1RSetUpdatingPauseTimePercent=5",
	"-XX:SurvivorRatio=32",
	"-XX:+PerfDisableSharedMem",
	"-XX:MaxTenuringThreshold=1",
	"-Dusing.aikars.flags=https://mcflags.emc.gs",
	"-Daikars.new.flags=true",
	"-Dfile.encoding=UTF-8",
}

var zgcFlags = []string{
	"-XX:+UseZGC",
	"-XX:+ZGenerational",
	"-XX:+DisableExplicitGC",
	"-XX:+AlwaysPreTouch",
	"-XX:+PerfDisableSharedMem",
	"-Dfile.encoding=UTF-8",
}

func CheckJava() (string, error) {
	cmd := exec.Command(javaCmd, "-version")
	output, err := cmd.CombinedOutput()
	if err != nil {
		return "", fmt.Errorf("Java is not installed or not in PATH")
	}

	version := extractJavaVersion(string(output))
	return version, nil
}

func extractJavaVersion(output string) string {
	lines := strings.Split(output, "\n")
	for _, line := range lines {
		if strings.Contains(line, "version") {
			parts := strings.Split(line, "\"")
			if len(parts) >= 2 {
				return parts[1]
			}
		}
	}
	return "unknown"
}

func GetSystemRAM() (totalGB int, availableGB int, err error) {
	if runtime.GOOS == "windows" || runtime.GOOS == "linux" || runtime.GOOS == "darwin" {
		v, err := mem.VirtualMemory()
		if err != nil {
			return 0, 0, err
		}
		totalGB = int(v.Total / (1024 * 1024 * 1024))
		availableGB = int(v.Available / (1024 * 1024 * 1024))
		return totalGB, availableGB, nil
	}
	return 0, 0, fmt.Errorf("unsupported OS")
}

func CalculateSmartRAM(configMax, percentage, minRAM int) int {
	_, available, err := GetSystemRAM()
	if err != nil {
		if configMax > 0 {
			return configMax
		}
		return minRAM + 2
	}

	if configMax > 0 {
		if configMax > available {
			fmt.Printf("Warning: Configured MaxRAM (%dGB) is greater than available system RAM (%dGB). Adjusting to safe limit.\n", configMax, available-1)
			safe := available - 1
			if safe < minRAM {
				return minRAM
			}
			return safe
		}
		return configMax
	}

	calculated := int(float64(available) * (float64(percentage) / 100.0))

	if available-calculated < 1 {
		calculated = available - 1
	}

	if calculated < minRAM {
		return minRAM
	}

	return calculated
}

func RunServer(jarFile string, minRAM, maxRAM int, useZGC bool, serverArgs []string) error {
	args := []string{
		fmt.Sprintf("-Xms%dG", minRAM),
		fmt.Sprintf("-Xmx%dG", maxRAM),
	}

	if useZGC {
		if maxRAM < 4 {
			fmt.Println("Warning: ZGC is enabled but MaxRAM is low (< 4GB). G1GC might perform better.")
		}
		fmt.Println("Using Z Garbage Collector (ZGC)")
		args = append(args, zgcFlags...)
	} else {
		fmt.Println("Using G1 Garbage Collector (G1GC)")
		args = append(args, aikarFlags...)
	}

	args = append(args, "-jar", jarFile)
	args = append(args, serverArgs...)

	cmd := exec.Command(javaCmd, args...)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	if err := cmd.Start(); err != nil {
		return fmt.Errorf("failed to start server: %w", err)
	}

	if err := cmd.Wait(); err != nil {
		return fmt.Errorf("server stopped with error: %w", err)
	}

	return nil
}
