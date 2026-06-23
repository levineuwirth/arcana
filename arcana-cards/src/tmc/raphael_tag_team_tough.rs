//! Raphael, Tag Team Tough — `{4}{R}{R}` legendary 5/6 Mutant Ninja
//! Turtle with Menace.
//!
//! Oracle:
//! * Menace — keyword.
//! * "Whenever Raphael deals combat damage to a player for the first
//!   time each turn, untap all attacking creatures. After this combat
//!   phase, there is an additional combat phase."
//!   - A `DamageDealt` trigger (source = this creature, target = a
//!     player, combat only) with `OncePerTurn` frequency for "for the
//!     first time each turn".
//!   - "untap all attacking creatures" → `ForEach` over the attacking
//!     creatures with `Effect::Untap`.
//!   - "an additional combat phase" → `Effect::AdditionalCombatPhase`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Raphael, Tag Team Tough");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(turtle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: untap_and_extra_combat,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn untap_and_extra_combat(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let attackers = script::ids_matching(
        state,
        &ObjectFilter::creature().attacking_only(),
        trig.controller,
    );
    vec![
        Effect::ForEach {
            targets: attackers,
            effect: Box::new(Effect::Untap { target: NULL_OBJECT_ID }),
        },
        Effect::AdditionalCombatPhase,
    ]
}
