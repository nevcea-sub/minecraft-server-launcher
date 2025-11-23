use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaperProject {
    pub versions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PaperBuildsResponse {
    pub builds: Vec<PaperBuildInfo>,
}

#[derive(Debug, Deserialize)]
pub struct PaperBuildInfo {
    pub build: u32,
}

#[derive(Debug, Deserialize)]
pub struct PaperDownload {
    pub downloads: PaperDownloads,
}

#[derive(Debug, Deserialize)]
pub struct PaperDownloads {
    pub application: PaperApplication,
}

#[derive(Debug, Deserialize)]
pub struct PaperApplication {
    pub name: String,
}

pub const API_BASE: &str = "https://api.papermc.io/v2/projects/paper";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paper_project_deserialize() {
        let json = r#"{"versions": ["1.21.1", "1.21"]}"#;
        let project: Result<PaperProject, _> = serde_json::from_str(json);
        assert!(project.is_ok());
        let project = project.unwrap();
        assert_eq!(project.versions.len(), 2);
    }

    #[test]
    fn test_paper_builds_response_deserialize() {
        let json = r#"{"builds": [{"build": 100}, {"build": 101}]}"#;
        let response: Result<PaperBuildsResponse, _> = serde_json::from_str(json);
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.builds.len(), 2);
        assert_eq!(response.builds[0].build, 100);
    }

    #[test]
    fn test_paper_download_deserialize() {
        let json = r#"{"downloads": {"application": {"name": "paper-1.21.1-100.jar"}}}"#;
        let download: Result<PaperDownload, _> = serde_json::from_str(json);
        assert!(download.is_ok());
        let download = download.unwrap();
        assert_eq!(download.downloads.application.name, "paper-1.21.1-100.jar");
    }

    #[test]
    fn test_paper_project_empty_versions() {
        let json = r#"{"versions": []}"#;
        let project: Result<PaperProject, _> = serde_json::from_str(json);
        assert!(project.is_ok());
        assert!(project.unwrap().versions.is_empty());
    }

    #[test]
    fn test_paper_builds_response_empty() {
        let json = r#"{"builds": []}"#;
        let response: Result<PaperBuildsResponse, _> = serde_json::from_str(json);
        assert!(response.is_ok());
        assert!(response.unwrap().builds.is_empty());
    }

    #[test]
    fn test_paper_builds_response_large_build_numbers() {
        let json = r#"{"builds": [{"build": 999999}, {"build": 1000000}]}"#;
        let response: Result<PaperBuildsResponse, _> = serde_json::from_str(json);
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.builds[0].build, 999999);
        assert_eq!(response.builds[1].build, 1000000);
    }

    #[test]
    fn test_paper_download_complex_name() {
        let json = r#"{"downloads": {"application": {"name": "paper-1.21.1-115-af06383.jar"}}}"#;
        let download: Result<PaperDownload, _> = serde_json::from_str(json);
        assert!(download.is_ok());
        let download = download.unwrap();
        assert_eq!(download.downloads.application.name, "paper-1.21.1-115-af06383.jar");
    }

    #[test]
    fn test_paper_project_many_versions() {
        let versions: Vec<String> = (1..=100).map(|i| format!("1.21.{}", i)).collect();
        let json = format!(r#"{{"versions": {:?}}}"#, versions);
        let project: Result<PaperProject, _> = serde_json::from_str(&json);
        assert!(project.is_ok());
        assert_eq!(project.unwrap().versions.len(), 100);
    }
}

