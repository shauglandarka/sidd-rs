use serde::Deserialize;

// SIDD Vol. 1 version 1.0, pg. 71
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename = "ProductProcessingType")]
pub struct ProductProcessing {
    #[serde(rename = "ProcessingModule")]
    pub processing_module: Vec<ProcessingModule>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ProcessingModule {
    #[serde(rename = "ModuleName")]
    pub module_name: ModuleName,
    #[serde(rename = "ModuleParameter", default)]
    pub module_parmeters: Vec<ModuleParameter>,
    #[serde(rename = "ProcessingModule", default)] //recursive
    pub sub_modules: Vec<ProcessingModule>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ModuleName {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub content: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ModuleParameter {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub value: ParamValue,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(untagged)] // Try different types until one fits
pub enum ParamValue{
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}


#[cfg(test)]
mod tests {
    use super::ProductProcessing;
    use quick_xml::de::from_str;

    #[test]
    fn test_product_processing() {
        let xml_str = r#"
            <ProductProcessingType><ProcessingModule>
            <ModuleName name="">ChainParameters</ModuleName>
            <ProcessingModule>
            <ModuleParameter name="Order">1</ModuleParameter>
            <ModuleParameter name="Enabled">false</ModuleParameter>
            <ModuleParameter name="SpatiallyVariant">false</ModuleParameter>
            <ModuleName name="">ApertureDeweight</ModuleName>
            </ProcessingModule><ProcessingModule>
            <ModuleParameter name="Order">2</ModuleParameter>
            <ModuleParameter name="Enabled">false</ModuleParameter>
            <ModuleParameter name="SpatiallyVariant">false</ModuleParameter>
            <ModuleName name="">Autofocus</ModuleName>
            </ProcessingModule><ProcessingModule>
            <ModuleParameter name="Order">3</ModuleParameter>
            <ModuleParameter name="Enabled">true</ModuleParameter>
            <ModuleParameter name="SpatiallyVariant">false</ModuleParameter>
            <ModuleName name="">Upsample</ModuleName>
            </ProcessingModule><ProcessingModule>
            <ModuleParameter name="Order">4</ModuleParameter>
            <ModuleParameter name="Enabled">true</ModuleParameter>
            <ModuleParameter name="PeakAmplification_dB">5</ModuleParameter>
            <ModuleParameter name="SidelobesPreserved ">false</ModuleParameter>
            <ModuleParameter name="SpatiallyVariant">true</ModuleParameter>
            <ModuleParameter name="Type">SEG_SVA</ModuleParameter>
            <ModuleName name="">Apodization</ModuleName>
            </ProcessingModule><ProcessingModule>
            <ModuleParameter name="Order">5</ModuleParameter>
            <ModuleParameter name="Enabled">true</ModuleParameter>
            <ModuleParameter name="AzimuthFilterLength">10</ModuleParameter>
            <ModuleParameter name="AzimuthResolutionFactor">2.634005e-01</ModuleParameter>
            <ModuleParameter name="RangeFilterLength">10</ModuleParameter>
            <ModuleParameter name="RangeResolutionFactor">2.634005e-01</ModuleParameter>
            <ModuleParameter name="SpatiallyVariant">false</ModuleParameter>
            <ModuleName name="">Multilook</ModuleName>
            </ProcessingModule><ProcessingModule>
            <ModuleParameter name="Order">6</ModuleParameter>
            <ModuleParameter name="Enabled">true</ModuleParameter>
            <ModuleParameter name="SpatiallyVariant">false</ModuleParameter>
            <ModuleName name="">NoiseSubtract</ModuleName>
            </ProcessingModule><ProcessingModule>
            <ModuleParameter name="Order">7</ModuleParameter>
            <ModuleParameter name="Enabled">true</ModuleParameter>
            <ModuleParameter name="SpatiallyVariant">true</ModuleParameter>
            <ModuleName name="">SpectralShape</ModuleName>
            </ProcessingModule><ProcessingModule>
            <ModuleParameter name="Order">8</ModuleParameter>
            <ModuleParameter name="Enabled">true</ModuleParameter>
            <ModuleParameter name="ClipHigh">1.156142e+00</ModuleParameter>
            <ModuleParameter name="ClipLow">2.108300e-02</ModuleParameter>
            <ModuleParameter name="DensityMax">255</ModuleParameter>
            <ModuleParameter name="DensityMin">30</ModuleParameter>
            <ModuleParameter name="SpatiallyVariant">true</ModuleParameter>
            <ModuleParameter name="Type">SVR</ModuleParameter>
            <ModuleName name="">Remap</ModuleName>
            </ProcessingModule><ProcessingModule>
            <ModuleParameter name="Order">9</ModuleParameter>
            <ModuleParameter name="Enabled">true</ModuleParameter>
            <ModuleParameter name="SpatiallyVariant">false</ModuleParameter>
            <ModuleName name="">PlaneProject</ModuleName>
            </ProcessingModule><ProcessingModule>
            <ModuleParameter name="Order">10</ModuleParameter>
            <ModuleParameter name="Enabled">true</ModuleParameter>
            <ModuleParameter name="DensityBreak">1.275000e+02</ModuleParameter>
            <ModuleParameter name="DensityScaling">2</ModuleParameter>
            <ModuleParameter name="MinimumValidValue">1</ModuleParameter>
            <ModuleParameter name="SpatiallyVariant">false</ModuleParameter>
            <ModuleName name="">DataRemapping</ModuleName>
            </ProcessingModule></ProcessingModule>
            </ProductProcessingType>"#;
                                 
        match from_str::<ProductProcessing>(&xml_str) {
            Ok(_) => {
                println!("Successfully Deserialized!");
            },
            Err(e) => panic!("Deserialization FAILED: {:#?}", e),
        }

    }
}
