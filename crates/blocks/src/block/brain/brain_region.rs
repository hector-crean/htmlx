use strum::IntoStaticStr;

#[derive(
    Debug,
    Default,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    specta::Type,
    IntoStaticStr,
)]

pub enum BrainRegionName {
    #[default]
    #[serde(rename = "fronto-subcortical")]
    Frontosubcortical,
    #[serde(rename = "orbitofrontal")]
    Orbitofrontal,
    #[serde(rename = "anterior-cingulated-gyrus")]
    AnteriorCingulatedGyrus,
    #[serde(rename = "bilateral-temporal-cortex")]
    BilateralTemporalCortex,
    #[serde(rename = "parietal-lobe")]
    ParietalLobe,
    #[serde(rename = "thalmus")]
    Thalamus,
    #[serde(rename = "hippocampus")]
    Hippocampus,
    #[serde(rename = "amygdala")]
    Amygdala,
    #[serde(rename = "hypothalamus")]
    Hypothalamus,
    #[serde(rename = "anterior-cingulate-cortex")]
    AnteriorCingulateCortex,
    #[serde(rename = "posterior-cingulate-cortex")]
    PosteriorCingulateCortex,
    #[serde(rename = "striatum")]
    Striatum,
    #[serde(rename = "prefrontal-cortex")]
    PrefrontalCortex,
    #[serde(rename = "ventral-frontal-cortex")]
    VentralFrontalCortex,
    #[serde(rename = "frontal-lobe")]
    FrontalLobe,
    #[serde(rename = "dlpfc")]
    Dlpfc,
    #[serde(rename = "vlpfc")]
    Vlpfc,
    #[serde(rename = "nucleus-accumbens")]
    NucleusAccumbens,
    #[serde(rename = "basal-forebrain")]
    BasalForebrain,
    #[serde(rename = "anterior-caudate")]
    AnteriorCaudate,
    #[serde(rename = "grey-matter")]
    GreyMatter,
    #[serde(rename = "lateral-ventricle")]
    LateralVentricle,
    #[serde(rename = "occipital-lobe")]
    OccipitalLobe,
    #[serde(rename = "auditory-cortex")]
    AuditoryCortex,
    #[serde(rename = "substantia-nigra")]
    SubstantiaNigra,
    #[serde(rename = "nucleus-accumbens-area")]
    NucleusAccumbensArea,
    #[serde(rename = "amyloid-stage-1-mild-region-1")]
    AmyloidStage1MildRegion1,
    #[serde(rename = "amyloid-stage-2-moderate-region-1")]
    AmyloidStage2ModerateRegion1,
    #[serde(rename = "amyloid-stage-2-mild-region-1")]
    AmyloidStage2MildRegion1,
    #[serde(rename = "amyloid-stage-3-severe-region-1")]
    AmyloidStage3SevereRegion1,
    #[serde(rename = "amyloid-stage-3-moderate-region-1")]
    AmyloidStage3ModerateRegion1,
    #[serde(rename = "amyloid-stage-3-moderate-region-2")]
    AmyloidStage3ModerateRegion2,
    #[serde(rename = "locus-coeruleus")]
    LocusCoeruleus,
    #[serde(rename = "insula")]
    Insula,
}
