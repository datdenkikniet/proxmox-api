use crate::nodes::node::storage::storage::upload::{ChecksumAlgorithm, Content, PostParams};
use crate::types::bounded_string::BoundedString;

impl crate::client::AsFilename for PostParams {
    fn as_filename(&self) -> String {
        self.filename.get_value().to_string()
    }
}

impl IntoIterator for PostParams {
    type Item = (String, String);
    type IntoIter = std::vec::IntoIter<(String, String)>;

    fn into_iter(self) -> Self::IntoIter {
        let mut fields = Vec::new();

        fields.push((
            "content".to_string(),
            match self.content {
                Content::Import => "import",
                Content::Iso => "iso",
                Content::Vztmpl => "vztmpl",
            }
            .to_string(),
        ));

        if let Some(checksum) = self.checksum {
            fields.push(("checksum".to_string(), checksum));
        }

        if let Some(algo) = self.checksum_algorithm {
            fields.push((
                "checksum-algorithm".to_string(),
                match algo {
                    ChecksumAlgorithm::Md5 => "md5",
                    ChecksumAlgorithm::Sha1 => "sha1",
                    ChecksumAlgorithm::Sha224 => "sha224",
                    ChecksumAlgorithm::Sha256 => "sha256",
                    ChecksumAlgorithm::Sha384 => "sha384",
                    ChecksumAlgorithm::Sha512 => "sha512",
                }
                .to_string(),
            ));
        }

        if let Some(tmpfilename) = self.tmpfilename {
            fields.push((
                "tmpfilename".to_string(),
                tmpfilename.get_value().to_string(),
            ));
        }

        for (k, v) in self.additional_properties {
            if let Some(s) = v.as_str() {
                fields.push((k, s.to_string()));
            }
        }

        fields.into_iter()
    }
}
