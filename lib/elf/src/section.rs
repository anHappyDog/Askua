pub trait Section {}

impl Section for TextSection {}
impl Section for SelfDefSection {}
impl Section for DataSection {}
impl Section for BssSection {}
impl Section for RodataSection {}
impl Section for RelaSection {}
impl Section for RelSection {}
impl Section for SymSection {}
impl Section for StrSection {}
impl Section for ShStrSection {}
impl Section for DynSection {}

pub struct TextSection {}

pub struct SelfDefSection {}

pub struct DataSection {}

pub struct BssSection {}

pub struct RodataSection {}

pub struct RelaSection {}

pub struct RelSection {}

pub struct SymSection {}

pub struct StrSection {}

pub struct ShStrSection {}

pub struct DynSection {}
