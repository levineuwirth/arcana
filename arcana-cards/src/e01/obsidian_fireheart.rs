//! Obsidian Fireheart — `{1}{R}{R}{R}` 4/4 red Elemental creature.
//! "{1}{R}{R}: Put a blaze counter on target land without a blaze counter on it.
//! For as long as that land has a blaze counter on it, it has
//! 'At the beginning of your upkeep, this land deals 1 damage to you.'"
//!
//! GAP: The "grant a triggered ability to a permanent for as long as it has a
//! counter" effect is not expressible with the current engine API. We put a
//! blaze counter (Named) on the target land; the triggered-upkeep-damage rider
//! is a GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Obsidian Fireheart");
    let elemental = reg.interner_mut().intern("Elemental");
    let _blaze = reg.interner_mut().intern("blaze");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}{R}: Put a blaze counter on target land without a blaze counter on it.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: put_blaze_counter,
            }),
    )
}

fn put_blaze_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let blaze_name = reg.interner().lookup("blaze").expect("blaze interned");
    // GAP: "for as long as that land has a blaze counter, it has triggered upkeep damage"
    // — granting a conditional triggered ability to another permanent is not expressible.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::Named(blaze_name),
        count: 1,
    }]
}
