//! Bashful Beastie — `{4}{G}` 5/4 green Creature — Beast.
//! "When this creature dies, manifest dread."
//! GAP: effect — Manifest Dread (look at top two, put one face-down as 2/2,
//! other to graveyard) is distinct from the basic Manifest catalog variant.
//! The basic Effect::Manifest puts only the top card; Manifest Dread's
//! "look at top two" variant is not in the catalog. Effect fn uses
//! Effect::Manifest as the closest approximation but notes the gap.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bashful Beastie");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies_manifest_dread,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_dies_manifest_dread(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: effect — Manifest Dread (look at top 2, put one face-down as 2/2,
    // mill the other) is not the same as Effect::Manifest (top card only).
    // Using Effect::Manifest as closest approximation.
    vec![Effect::Manifest { player: trig.controller }]
}
