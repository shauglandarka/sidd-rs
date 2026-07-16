use super::{ArpPoly, Coef, Ecef, PolyDim, RowCol, TimeCoaPoly, Vector3};
use serde::Deserialize;

// SIDD Vol. 1 version 1.0, pg. 52
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename = "MeasurementType")]
pub struct Measurement {
    #[serde(rename = "PlaneProjection")]
    pub plane_projection: PlaneProjection,
    #[serde(rename = "PixelFootprint")]
    pub pixel_footprint: RowCol,
    #[serde(rename = "ARPPoly")]
    pub arp_poly: ArpPoly,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct PlaneProjection {
    #[serde(rename = "ReferencePoint")]
    pub reference_point: ReferencePoint,
    #[serde(rename = "SampleSpacing")]
    pub sample_spacing: RowCol,
    #[serde(rename = "TimeCOAPoly")]
    pub time_coa_poly: TimeCoaPoly,
    #[serde(rename = "ProductPlane")]
    pub product_plane: ProductPlane,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ReferencePoint {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "ECEF")]
    pub ecef: Ecef,
    #[serde(rename = "Point")]
    pub point: RowCol,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ProductPlane {
    #[serde(rename = "RowUnitVector")]
    pub row_unit_vector: Vector3,
    #[serde(rename = "ColUnitVector")]
    pub col_unit_vector: Vector3,
}

#[cfg(test)]
mod tests {
    use super::Measurement;
    use quick_xml::de::from_str;

    #[test]
    fn test_measurement() {
        let xml_str = r#"
            <MeasurementType><PlaneProjection>
            <ReferencePoint name="SCP"><sicommon:ECEF>
            <sicommon:X>2755214.25</sicommon:X>
            <sicommon:Y>-4475440</sicommon:Y>
            <sicommon:Z>-3601780.75</sicommon:Z>
            </sicommon:ECEF><sicommon:Point>
            <sicommon:Row>1652.8160524076081</sicommon:Row>
            <sicommon:Col>1652.8160524073216</sicommon:Col>
            </sicommon:Point></ReferencePoint><SampleSpacing>
            <sicommon:Row>1.210099804301114</sicommon:Row>
            <sicommon:Col>1.210099804301114</sicommon:Col>
            </SampleSpacing>
            <TimeCOAPoly order1="0" order2="0">
            <sicommon:Coef exponent1="0" exponent2="0">1.5807564064278097</sicommon:Coef>
            </TimeCOAPoly><ProductPlane><RowUnitVector>
            <sicommon:X>-0.89028804946660367</sicommon:X>
            <sicommon:Y>-0.43245685105969311</sicommon:Y>
            <sicommon:Z>-0.1427174164160939</sicommon:Z>
            </RowUnitVector><ColUnitVector>
            <sicommon:X>-0.14555684570181165</sicommon:X>
            <sicommon:Y>0.56717545259580548</sicommon:Y>
            <sicommon:Z>-0.81063259904970664</sicommon:Z>
            </ColUnitVector></ProductPlane></PlaneProjection>
            <PixelFootprint><sicommon:Row>3305</sicommon:Row>
            <sicommon:Col>3305</sicommon:Col></PixelFootprint>
            <ARPPoly><sicommon:X order1="5">
            <sicommon:Coef exponent1="0">3523634.989128918</sicommon:Coef>
            <sicommon:Coef exponent1="1">1252.954628894629</sicommon:Coef>
            <sicommon:Coef exponent1="2">-2.4261369191838096</sicommon:Coef>
            <sicommon:Coef exponent1="3">-0.00012950493563723862</sicommon:Coef>
            <sicommon:Coef exponent1="4">-1.6389780045490147e-07</sicommon:Coef>
            <sicommon:Coef exponent1="5">2.1153190857491497e-08</sicommon:Coef>
            </sicommon:X><sicommon:Y order1="5">
            <sicommon:Coef exponent1="0">-4558440.4693010785</sicommon:Coef>
            <sicommon:Coef exponent1="1">-4249.554337181816</sicommon:Coef>
            <sicommon:Coef exponent1="2">2.6463095425146066</sicommon:Coef>
            <sicommon:Coef exponent1="3">0.000978629030427375</sicommon:Coef>
            <sicommon:Coef exponent1="4">1.5863349235062844e-07</sicommon:Coef>
            <sicommon:Coef exponent1="5">-2.0440492996753347e-08</sicommon:Coef>
            </sicommon:Y><sicommon:Z order1="5">
            <sicommon:Coef exponent1="0">-3817375.7189675374</sicommon:Coef>
            <sicommon:Coef exponent1="1">6255.072454528905</sicommon:Coef>
            <sicommon:Coef exponent1="2">2.3091750472376065</sicommon:Coef>
            <sicommon:Coef exponent1="3">-0.0012530232552907018</sicommon:Coef>
            <sicommon:Coef exponent1="4">1.417114461889858e-07</sicommon:Coef>
            <sicommon:Coef exponent1="5">-1.834143210486639e-08</sicommon:Coef>
            </sicommon:Z></ARPPoly></MeasurementType>"#;

        match from_str::<Measurement>(&xml_str) {
            Ok(_) => println!("Successfully Deserialized!"),
            Err(e) => panic!("Deserialization FALIED: {:#?}", e),
        }
    }
}
