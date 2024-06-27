use strum::{AsRefStr, IntoStaticStr};

#[derive(
    Debug,
    Default,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    specta::Type,
    IntoStaticStr,
    AsRefStr,
)]
pub enum BrainRegionName {
    #[default]
    #[serde(rename = "fronto-subcortical")]
    #[strum(serialize = "fronto-subcortical")]
    Frontosubcortical,
    #[serde(rename = "orbitofrontal")]
    #[strum(serialize = "orbitofrontal")]
    Orbitofrontal,
    #[serde(rename = "anterior-cingulated-gyrus")]
    #[strum(serialize = "anterior-cingulated-gyrus")]
    AnteriorCingulatedGyrus,
    #[serde(rename = "bilateral-temporal-cortex")]
    #[strum(serialize = "bilateral-temporal-cortex")]
    BilateralTemporalCortex,
    #[serde(rename = "parietal-lobe")]
    #[strum(serialize = "parietal-lobe")]
    ParietalLobe,
    #[serde(rename = "thalamus")]
    #[strum(serialize = "thalamus")]
    Thalamus,
    #[serde(rename = "hippocampus")]
    #[strum(serialize = "hippocampus")]
    Hippocampus,
    #[serde(rename = "amygdala")]
    #[strum(serialize = "amygdala")]
    Amygdala,
    #[serde(rename = "hypothalamus")]
    #[strum(serialize = "hypothalamus")]
    Hypothalamus,
    #[serde(rename = "anterior-cingulate-cortex")]
    #[strum(serialize = "anterior-cingulate-cortex")]
    AnteriorCingulateCortex,
    #[serde(rename = "posterior-cingulate-cortex")]
    #[strum(serialize = "posterior-cingulate-cortex")]
    PosteriorCingulateCortex,
    #[serde(rename = "striatum")]
    #[strum(serialize = "striatum")]
    Striatum,
    #[serde(rename = "prefrontal-cortex")]
    #[strum(serialize = "prefrontal-cortex")]
    PrefrontalCortex,
    #[serde(rename = "ventral-frontal-cortex")]
    #[strum(serialize = "ventral-frontal-cortex")]
    VentralFrontalCortex,
    #[serde(rename = "frontal-lobe")]
    #[strum(serialize = "frontal-lobe")]
    FrontalLobe,
    #[serde(rename = "dlpfc")]
    #[strum(serialize = "dlpfc")]
    Dlpfc,
    #[serde(rename = "vlpfc")]
    #[strum(serialize = "vlpfc")]
    Vlpfc,
    #[serde(rename = "nucleus-accumbens")]
    #[strum(serialize = "nucleus-accumbens")]
    NucleusAccumbens,
    #[serde(rename = "basal-forebrain")]
    #[strum(serialize = "basal-forebrain")]
    BasalForebrain,
    #[serde(rename = "anterior-caudate")]
    #[strum(serialize = "anterior-caudate")]
    AnteriorCaudate,
    #[serde(rename = "grey-matter")]
    #[strum(serialize = "grey-matter")]
    GreyMatter,
    #[serde(rename = "lateral-ventricle")]
    #[strum(serialize = "lateral-ventricle")]
    LateralVentricle,
    #[serde(rename = "occipital-lobe")]
    #[strum(serialize = "occipital-lobe")]
    OccipitalLobe,
    #[serde(rename = "auditory-cortex")]
    #[strum(serialize = "auditory-cortex")]
    AuditoryCortex,
    #[serde(rename = "substantia-nigra")]
    #[strum(serialize = "substantia-nigra")]
    SubstantiaNigra,
    #[serde(rename = "nucleus-accumbens-area")]
    #[strum(serialize = "nucleus-accumbens-area")]
    NucleusAccumbensArea,
    #[serde(rename = "amyloid-stage-1-mild-region-1")]
    #[strum(serialize = "amyloid-stage-1-mild-region-1")]
    AmyloidStage1MildRegion1,
    #[serde(rename = "amyloid-stage-2-moderate-region-1")]
    #[strum(serialize = "amyloid-stage-2-moderate-region-1")]
    AmyloidStage2ModerateRegion1,
    #[serde(rename = "amyloid-stage-2-mild-region-1")]
    #[strum(serialize = "amyloid-stage-2-mild-region-1")]
    AmyloidStage2MildRegion1,
    #[serde(rename = "amyloid-stage-3-severe-region-1")]
    #[strum(serialize = "amyloid-stage-3-severe-region-1")]
    AmyloidStage3SevereRegion1,
    #[serde(rename = "amyloid-stage-3-moderate-region-1")]
    #[strum(serialize = "amyloid-stage-3-moderate-region-1")]
    AmyloidStage3ModerateRegion1,
    #[serde(rename = "amyloid-stage-3-moderate-region-2")]
    #[strum(serialize = "amyloid-stage-3-moderate-region-2")]
    AmyloidStage3ModerateRegion2,
    #[serde(rename = "locus-coeruleus")]
    #[strum(serialize = "locus-coeruleus")]
    LocusCoeruleus,
    #[serde(rename = "insula")]
    #[strum(serialize = "insula")]
    Insula,
}
