//! Ankle Shanker — `{2}{R}{W}{B}` 2/2 Goblin Berserker with Haste.
//!
//! * Haste (keyword).
//! * "Whenever this creature attacks, creatures you control gain first
//!   strike and deathtouch until end of turn." — a `SelfAttacks`
//!   trigger that grants both keywords to every creature you control
//!   (enumerated at resolution).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ankle Shanker");
    let goblin = reg.interner_mut().intern("Goblin");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: grant_fs_and_dt,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn grant_fs_and_dt(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let mut effects = Vec::new();
    for id in ids {
        effects.push(Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::FirstStrike,
            duration: Duration::EndOfTurn,
        });
        effects.push(Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Deathtouch,
            duration: Duration::EndOfTurn,
        });
    }
    vec![Effect::Sequence(effects)]
}
