use serde::Deserialize;

// SIDD Vol. 1 version 1.0, pg.37
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename = "ProductDisplayType")]
#[serde(rename_all = "PascalCase")]
pub struct Display {
    #[serde(rename = "PixelType")]
    pub pixel_type: PixelType,
    #[serde(rename = "MagnificationMethod")]
    pub magnification_method: Option<MagnificationMethod>,
    #[serde(rename = "DecimationMethod")]
    pub decimation_method: Option<DecimationMethod>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct PixelType {
    #[serde(rename = "$text")]
    pub value: PixelTypeEnum,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum PixelTypeEnum {
    MONO8I,
    MONO8LU,
    MONO16I,
    RGB8LU,
    RGB24I,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MagnificationMethod {
    #[serde(rename = "$text")]
    pub value: MagnificationMethodEnum,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum MagnificationMethodEnum {
    NEAREST_NEIGHBOR,
    BILINEAR,
    LAGRANGE,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct DecimationMethod {
    #[serde(rename = "$text")]
    pub value: DecimationMethodEnum,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum DecimationMethodEnum {
    NEAREST_NEIGHBOR,
    BILINEAR,
    BRIGHTEST_PIXEL,
    LAGRANGE,
}



#[cfg(test)]
mod tests {
    use super::Display;
    use quick_xml::de::from_str;

    #[test]
    fn test_display() {
        let xml_str = r#"
            <ProductDisplayType><PixelType>MONO8I</PixelType>
            <MagnificationMethod>NEAREST_NEIGHBOR</MagnificationMethod>
            <DecimationMethod>BRIGHTEST_PIXEL</DecimationMethod>
            </ProductDisplayType>"#;
                
        match from_str::<Display>(&xml_str) {
            Ok(_) => {
                println!("Successfully Deserialized!");
            },
            Err(e) => panic!("Deserialization FAILED: {:#?}", e),
        }

    }
}
