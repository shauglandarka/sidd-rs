use serde::Deserialize;

// SIDD Vol. 1 version 1.0, pg. 47
// Only doing footprint, rest is incomplete
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename = "GeoGraphicAndTargetType")]
pub struct GeographicAndTarget {
    #[serde(rename = "GeographicCoverage")]
    pub geographic_coverage: GeographicCoverage,
    //#[serde(rename = "TargetInformation")]
    #[serde(skip_deserializing)]
    pub target_information: Option<Vec<TargetInformation>>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
// TODO
pub struct TargetInformation {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct GeographicCoverage {
    #[serde(rename = "Footprint")]
    pub footprint: Footprint,
    pub geographic_info: Option<()>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Footprint {
    #[serde(rename = "@size")]
    pub size: u32,
    #[serde(rename = "Vertex")]
    pub vertices: Vec<Vertex>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Vertex {
    #[serde(rename = "@index")]
    pub index: u32,
    #[serde(rename = "Lat")]
    pub lat: f64,
    #[serde(rename = "Lon")]
    pub lon: f64,
}


#[cfg(test)]
mod tests {
    use super::GeographicAndTarget;
    use quick_xml::de::from_str;

    #[test]
    fn test_display() {
        let xml_str = r#"
             <GeographicAndTargetType><GeographicCoverage>
             <Footprint size="4"><Vertex index="1">
             <sicommon:Lat>-34.582781028721506</sicommon:Lat>
             <sicommon:Lon>-58.364620049845485</sicommon:Lon>
             </Vertex><Vertex index="2">
             <sicommon:Lat>-34.618274827746241</sicommon:Lat>
             <sicommon:Lon>-58.357053494431256</sicommon:Lon>
             </Vertex><Vertex index="3">
             <sicommon:Lat>-34.624525292465648</sicommon:Lat>
             <sicommon:Lon>-58.399990026051022</sicommon:Lon>
             </Vertex><Vertex index="4">
             <sicommon:Lat>-34.589028870944624</sicommon:Lat>
             <sicommon:Lon>-58.407538882532251</sicommon:Lon>
             </Vertex></Footprint><GeographicInfo />
             </GeographicCoverage></GeographicAndTargetType>"#;
                         
        match from_str::<GeographicAndTarget>(&xml_str) {
            Ok(display) => {
                println!("Successfully Deserialized!");
            },
            Err(e) => panic!("Deserialization FAILED: {:#?}", e),
        }

    }
}
