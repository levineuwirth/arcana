//! Imodane, the Pyrohammer — `{2}{R}{R}` 4/4 red legendary creature
//! (Human Knight).
//! "Whenever an instant or sorcery spell you control that targets only a
//! single creature deals damage to that creature, Imodane deals that much
//! damage to each opponent."
//!
//! GAP: trigger — "instant or sorcery you control that targets only a
//! single creature deals damage to that creature" is not a single
//! TriggerCondition variant; using DamageDealt with source_filter for
//! instant/sorcery as best effort. The "targets only a single creature"
//! constraint is not expressible.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Imodane, the Pyrohammer");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "instant/sorcery you control deals damage
                // to a single targeted creature" not precisely modelable;
                // using DamageDealt filtered to you-controlled source.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new()
                        .controlled_by(ControllerConstraint::You)
                        .with_types(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    target_filter: TargetFilter::Creature,
                    combat_only: false,
                },
                intervening_if: None,
                effect: on_damage_dealt,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_damage_dealt(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    let opponents = script::opponents(state, trig.controller);
    let effects: Vec<Effect> = opponents
        .into_iter()
        .map(|p| Effect::DealDamage {
            target: DamageTarget::Player(p),
            amount: n,
            source: trig.source,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
