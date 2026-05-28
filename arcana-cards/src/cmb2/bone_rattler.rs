//! Bone Rattler — `{3}{B}{B}` 4/4 black Skeleton.
//! "When Bone Rattler is put into your graveyard from anywhere, exile it. When you do,
//! create four Reassembling Skeleton token cards and put them into your graveyard."
//! GAP: "create token cards and put them into your graveyard" — CreateToken puts tokens
//! onto battlefield; no catalog variant for creating tokens in graveyard.
//! GAP: "Reassembling Skeleton" token cards are named tokens — conjure-like behavior.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bone Rattler");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skeleton);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_exile_create_skeletons,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_exile_create_skeletons(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile it, then create four Reassembling Skeleton token cards and put them
    // into your graveyard" — no Effect for creating token cards in graveyard.
    vec![Effect::ExileFromGraveyard { target: trig.source }]
}
