use std::error::Error;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use log::{info, warn, error};
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;
use chrono::Utc;
use crate::io::core::Archiver;

pub struct EmlArchiver;

impl EmlArchiver {
    /// Creates a new EmlArchiver instance
    pub fn new() -> Self {
        info!("Creating new EmlArchiver");
        Self {}
    }

    /// Generates a MIME boundary for multipart messages
    fn generate_boundary() -> String {
        format!("--boundary_{}", Uuid::new_v4())
    }

    /// Creates EML headers with the given boundary
    fn create_eml_headers(boundary: &str) -> String {
        let date = Utc::now().format("%a, %d %b %Y %H:%M:%S %z").to_string();
        let mut headers = String::new();

        headers.push_str(&format!("Date: {}\r\n", date));
        headers.push_str("From: EmlArchiver <archiver@example.com>\r\n");
        headers.push_str("To: User <user@example.com>\r\n");
        headers.push_str("Subject: Archived Files\r\n");
        headers.push_str("MIME-Version: 1.0\r\n");
        headers.push_str(&format!("Content-Type: multipart/mixed; boundary=\"{}\"\r\n\r\n", boundary));

        headers
    }

    /// Creates an introduction part for the EML file
    fn create_introduction_part(boundary: &str) -> String {
        let mut intro = String::new();

        intro.push_str(&format!("--{}\r\n", boundary));
        intro.push_str("Content-Type: text/plain; charset=UTF-8\r\n");
        intro.push_str("Content-Transfer-Encoding: 8bit\r\n\r\n");
        intro.push_str("This is an archived collection of files created by EmlArchiver.\r\n\r\n");

        intro
    }

    /// Creates a file part header for the EML file
    fn create_file_part_header(boundary: &str, file_name: &str) -> String {
        let mut header = String::new();

        header.push_str(&format!("--{}\r\n", boundary));
        header.push_str("Content-Type: text/plain; charset=UTF-8\r\n");
        header.push_str("Content-Transfer-Encoding: 8bit\r\n");
        header.push_str(&format!("Content-Disposition: attachment; filename=\"{}\"\r\n\r\n", file_name));

        header
    }

    /// Reads a file and returns its contents as a UTF-8 string
    async fn read_file_as_text(file_path: &PathBuf) -> Result<String, Box<dyn Error + Send + Sync>> {
        let mut file = File::open(file_path).await
            .map_err(|e| {
                error!("Failed to open file {}: {}", file_path.display(), e);
                Box::new(e) as Box<dyn Error + Send + Sync>
            })?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await
            .map_err(|e| {
                error!("Failed to read file {}: {}", file_path.display(), e);
                Box::new(e) as Box<dyn Error + Send + Sync>
            })?;

        String::from_utf8(buffer)
            .map_err(|e| {
                error!("File {} is not valid UTF-8: {}", file_path.display(), e);
                Box::new(e) as Box<dyn Error + Send + Sync>
            })
    }

    /// Checks if a file is valid for inclusion in the archive
    fn is_valid_file(file_path: &PathBuf) -> bool {
        if !file_path.exists() {
            warn!("File does not exist: {}", file_path.display());
            return false;
        }

        if file_path.file_name().is_none() {
            warn!("Could not extract filename from path: {}", file_path.display());
            return false;
        }

        true
    }

    /// Creates the EML file at the specified path
    async fn write_eml_file(output_path: &PathBuf, content: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Ensure parent directory exists
        if let Some(parent) = output_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await
                    .map_err(|e| {
                        error!("Failed to create directory {}: {}", parent.display(), e);
                        Box::new(e) as Box<dyn Error + Send + Sync>
                    })?;
            }
        }

        // Create and write to output file
        let mut file = File::create(output_path).await
            .map_err(|e| {
                error!("Failed to create output file {}: {}", output_path.display(), e);
                Box::new(e) as Box<dyn Error + Send + Sync>
            })?;

        file.write_all(content.as_bytes()).await
            .map_err(|e| {
                error!("Failed to write EML content: {}", e);
                Box::new(e) as Box<dyn Error + Send + Sync>
            })?;

        Ok(())
    }

    /// Process a single file and add it to the EML content
    async fn process_file(
        file_path: &PathBuf,
        boundary: &str,
        eml_content: &mut String
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !Self::is_valid_file(file_path) {
            return Ok(());
        }

        let file_name = file_path.file_name().unwrap().to_string_lossy();
        info!("Processing file: {}", file_name);

        // Read file content as text
        match Self::read_file_as_text(file_path).await {
            Ok(text_content) => {
                // Add file part header
                eml_content.push_str(&Self::create_file_part_header(boundary, &file_name));

                // Add file content
                eml_content.push_str(&text_content);
                eml_content.push_str("\r\n\r\n");

                Ok(())
            },
            Err(e) => {
                warn!("Skipping file {}: {}", file_path.display(), e);
                Ok(())
            }
        }
    }
}

impl Archiver for EmlArchiver {
    fn archive<'life>(
        &'life self,
        output_path: &'life PathBuf,
        file_paths: &'life [PathBuf],
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send + 'life>> {
        Box::pin(async move {
            info!("Starting EML archive creation at: {}", output_path.display());

            if file_paths.is_empty() {
                warn!("No files to archive, creating empty EML file");
            }

            // Generate boundary and create headers
            let boundary = Self::generate_boundary();
            let mut eml_content = Self::create_eml_headers(&boundary);

            // Add introduction
            eml_content.push_str(&Self::create_introduction_part(&boundary));

            // Process each file
            for file_path in file_paths {
                if let Err(e) = Self::process_file(file_path, &boundary, &mut eml_content).await {
                    error!("Error processing file {}: {}", file_path.display(), e);
                    // Continue with other files instead of failing completely
                }
            }

            // Close the multipart message
            eml_content.push_str(&format!("--{}--\r\n", boundary));

            // Write the EML file
            info!("Writing EML archive to: {}", output_path.display());
            Self::write_eml_file(output_path, &eml_content).await?;

            info!("Successfully created EML archive at: {}", output_path.display());
            Ok(())
        })
    }
}
