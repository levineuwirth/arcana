//! Scourge of the Throne — `{4}{R}{R}` 5/5 Dragon.
//! Flying. Dethrone.
//! Whenever this creature attacks for the first time each turn, if it's
//! attacking the player with the most life or tied for most life, untap
//! all attacking creatures. After this phase, there is an additional
//! combat phase.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scourge of the Throne");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Dethrone],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: intervening-if "if it's attacking the player with the
            // most life or tied for most life" has no conditions helper;
            // the trigger fires unconditionally on the first attack.
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: untap_attackers_extra_combat,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::OncePerTurn,
            target_requirements: Vec::new(),
        }),
    )
}

fn untap_attackers_extra_combat(
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
