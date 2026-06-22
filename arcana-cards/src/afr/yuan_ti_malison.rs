//! Yuan-Ti Malison — `{1}{U}` 2/1 Snake Rogue.
//! "This creature can't be blocked as long as it's attacking alone."
//! "Whenever this creature deals combat damage to a player, venture into the
//!  dungeon."
//!
//! GAP: "can't be blocked as long as it's attacking alone" is a conditional
//! static evasion (CantBeBlocked is unconditional) and is not expressible — the
//! evasion is omitted. Venture into the dungeon is not an available
//! KeywordAbility variant, so it is not listed in the keyword line, but the
//! combat-damage venture trigger below uses Effect::Venture and is implemented.
//! Fidelity note: the DamageDealt condition's source filter cannot be pinned to
//! "this creature" specifically (no self-source filter primitive), matching the
//! engine's documented combat-damage trigger idiom.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yuan-Ti Malison");
    let snake = reg.interner_mut().intern("Snake");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: venture,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn venture(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Venture { player: trig.controller }]
}
