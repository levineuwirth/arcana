//! Pteramander — `{U}` 1/1 Salamander Drake.
//! Flying.
//! {7}{U}: Adapt 4. This ability costs {1} less to activate for each
//! instant and sorcery card in your graveyard.
//!
//! Flying is a base keyword. The Adapt 4 activated ability is GAP'd:
//! there is no `Effect::Adapt` primitive (Adapt exists only as a
//! `KeywordAbility`, not as a counter-placement effect we can author),
//! and the "{1} less per instant/sorcery in your graveyard" cost
//! reduction is not expressible via `ActivationCost` either.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pteramander");
    let salamander = reg.interner_mut().intern("Salamander");
    let drake = reg.interner_mut().intern("Drake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(salamander);
    subtypes.0.insert(drake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "{7}{U}: Adapt 4. This ability costs {1} less to activate for
    // each instant and sorcery card in your graveyard." There is no
    // Effect::Adapt and no scaling-cost-reduction field on ActivationCost.
    reg.register(CardDefinition::new(name, chars))
}
