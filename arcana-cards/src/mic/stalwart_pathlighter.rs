//! Stalwart Pathlighter — `{2}{W}` 3/1 Creature — Human Soldier.
//!
//! Oracle:
//! * Vigilance.
//! * Coven — At the beginning of combat on your turn, if you control three or
//!   more creatures with different powers, creatures you control gain
//!   indestructible until end of turn.
//!
//! Decomposition: Vigilance keyword + one triggered ability on combat begins.
//! The effect grants indestructible to every creature you control (via
//! ForEach + Pump-keyword).
//!
//! GAP: the Coven intervening-if ("if you control three or more creatures with
//! different powers") is not expressible — no distinct-powers predicate in the
//! demonstrated conditions API. The trigger fires unconditionally (best-effort).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stalwart Pathlighter");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::You,
            },
            // GAP: Coven intervening-if "three or more creatures with different
            //      powers" — no distinct-powers predicate available.
            intervening_if: None,
            effect: grant_indestructible,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn grant_indestructible(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::GrantKeyword {
            target: NULL_OBJECT_ID,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        }),
    }]
}
