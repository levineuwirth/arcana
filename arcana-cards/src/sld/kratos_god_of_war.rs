//! Kratos, God of War — `{R}{R}{R}` 2/3 Legendary Creature — God Warrior.
//!
//! Oracle:
//! * Double strike.
//! * All creatures have haste. — GAP: no global static "all creatures have
//!   <keyword>" grant on the supported surface.
//! * At the beginning of each player's end step, Kratos deals damage to
//!   that player equal to the number of creatures that player controls that
//!   didn't attack this turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kratos, God of War");
    let god = reg.interner_mut().intern("God");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::Any,
            },
            intervening_if: None,
            effect: end_step_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn end_step_damage(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let them = state.active_player();
    // All creatures, then keep those `them` controls that didn't attack.
    let ids = script::ids_matching(state, &ObjectFilter::creature(), trig.controller);
    let n = ids
        .into_iter()
        .filter(|&id| {
            state.objects.get(id).map(|o| o.controller) == Some(them)
                && !script::creature_attacked_this_turn(state, id)
        })
        .count() as u32;
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(them),
        amount: n,
    }]
}
