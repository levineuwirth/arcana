//! Flummoxed Cyclops — `{3}{R}` 4/4 Creature — Cyclops. R.
//! Reach.
//! "Whenever two or more creatures your opponents control attack, this
//! creature can't block this combat." — CreatureAttacks (opponents'
//! creatures) → ForbidBlocking on self. GAP (fidelity): the "two or more"
//! count gate has no expressible predicate, so it fires even on a single
//! attacker — documented deviation.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flummoxed Cyclops");
    let cyclops = reg.interner_mut().intern("Cyclops");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cyclops);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP (fidelity): "two or more" attacker-count gate not expressible.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Opponent),
                },
                intervening_if: None,
                effect: forbid_self_block,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn forbid_self_block(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ForbidBlocking {
        target: trig.source,
        duration: Duration::EndOfTurn,
    }]
}
