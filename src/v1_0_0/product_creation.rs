use serde::Deserialize;
use std::collections::HashMap;

// SIDD Vol. 1 version 1.0, pg. 34
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename = "ProductCreationType")]
pub struct ProductCreation {
    #[serde(rename = "ProcessorInformation")]
    pub processor_info: ProcessorInformation,
    #[serde(rename = "Classification")]
    pub classification: Classification,
    #[serde(rename = "ProductName")]
    pub product_name: String,
    #[serde(rename = "ProductClass")]
    pub product_class: String,
    #[serde(rename = "ProductType")]
    pub product_type: Option<String>,
    // Vec handles the Optional <ProductCreationExtension> tags
    #[serde(rename = "ProductCreationExtension")]
    pub extensions: Vec<ProductCreationExtension>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ProductCreationExtension {
    // The @ symbol tells serde to look in the attributes, NOT a child tag
    #[serde(rename = "@name")] 
    pub name: String,
    // The $value tells serde to take the text content inside the tag
    #[serde(rename = "$value")] 
    pub value: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Classification {
    // REMOVED "ism:" from here to match your XML
    #[serde(rename = "@DESVersion")]
    pub des_version: i32,
    #[serde(rename = "@resourceElement")]
    pub resource_element: bool,
    #[serde(rename = "@createDate")]
    pub create_date: String,
    #[serde(rename = "@classification")]
    pub classification: String,
    #[serde(rename = "@ownerProducer")]
    pub owner_producer: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ProcessorInformation {
    #[serde(rename = "Application")]
    pub application: String,
    #[serde(rename = "ProcessingDateTime")]
    pub processing_date_time: String,
    #[serde(rename = "Site")]
    pub site: String,
    #[serde(rename = "Profile")]
    pub profile: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::ProductCreation;
    use quick_xml::de::from_str;

    #[test]
    fn test_product_creation() {
        let xml_str = r#"
            <ProductCreationType><ProcessorInformation>
            <Application>a8238b17a+uncommitedChanges</Application>
            <ProcessingDateTime>2024-03-26T23:21:51.255116Z</ProcessingDateTime>
            <Site>Ypsilanti</Site><Profile>0.2</Profile>
            </ProcessorInformation>
            <Classification ism:DESVersion="4" ism:resourceElement="true" ism:createDate="2024-03-26" ism:classification="U" ism:ownerProducer="USA" />
            <ProductName>2023-02-12T01:20:07_Umbra-05-Detected</ProductName>
            <ProductClass>NPAR</ProductClass>
            <ProductType>Uncalibrated</ProductType>
            <ProductCreationExtension name="UserName">UNDEFINED</ProductCreationExtension>
            <ProductCreationExtension name="HostName">06d6babaac76</ProductCreationExtension>
            <ProductCreationExtension name="ReferenceImagePath">2023-02-12-01-20-05_UMBRA-05_SICD.nitf</ProductCreationExtension>
            <ProductCreationExtension name="Runtime (sec)">5.494</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Read (sec)">0.857</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process (sec)">4.199</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Zero padding upsampleCols (sec)">0.012</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Upsample cols (sec)">0.226</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Zero padding upsampleRows (sec)">0.009</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Upsample rows (sec)">0.699</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Deskew (sec)">0.000</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Zero padding SVA (sec)">0.006</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Seg SVA (sec)">0.348</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\MultiLook filtering (sec)">0.053</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Footprint mask (sec)">0.001</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Read GSHHS database (sec)">0.929</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Register land-water mask to image (sec)">1.722</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Register land-water mask to image\Create downsampled land/water mask (sec)">0.000</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Register land-water mask to image\Create downsampled image (sec)">0.015</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Register land-water mask to image\Calculate correlation values (sec)">1.705</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Create land/water mask (sec)">0.013</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Partial footprint mask (sec)">0.000</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Compute min/max/sum (sec)">0.007</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Stats second pass (sec)">0.027</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Calculate CL/CH (sec)">0.000</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Zero padding SpectralShape (sec)">0.006</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Spectral Shape (sec)">0.047</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Zero padding SVR (sec)">0.006</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Remap SVR (sec)">0.021</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Rezero before projection (sec)">0.001</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Project (sec)">0.015</ProductCreationExtension>
            <ProductCreationExtension name="Runtime\Process\Remap to ubyte (sec)">0.011</ProductCreationExtension>
            </ProductCreationType>"#;
        
        match from_str::<ProductCreation>(&xml_str) {
            Ok(_) => println!("successfully deserialized!"),
            Err(e) => panic!("Deserialization FALIED: {:#?}", e),
        }
    }
}
