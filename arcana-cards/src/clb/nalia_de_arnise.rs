//! Nalia de'Arnise — `{1}{W}{B}` 3/3 Legendary Human Rogue.
//!
//! Oracle:
//! * You may look at the top card of your library any time. (static — GAP)
//! * You may cast Cleric, Rogue, Warrior, and Wizard spells from the top
//!   of your library. (static play permission — GAP)
//! * At the beginning of combat on your turn, if you have a full party,
//!   put a +1/+1 counter on each creature you control and those creatures
//!   gain deathtouch until end of turn.
//!
//! The combat trigger's effect (board-wide +1/+1 + deathtouch) is wired;
//! the "if you have a full party" intervening-if is GAP'd (no helper for
//! the four-role party check), so the trigger currently fires
//! unconditionally — a documented over-fire.

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
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nalia de'Arnise");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "You may look at the top card of your library any time." (static)
    // GAP: "You may cast Cleric, Rogue, Warrior, and Wizard spells from the
    //       top of your library." (static play permission)
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::You,
            },
            // GAP: intervening-if "if you have a full party" — no full-party
            //      predicate is available; fires unconditionally.
            intervening_if: None,
            effect: full_party_buff,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn full_party_buff(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![
        Effect::ForEach {
            targets: ids.clone(),
            effect: Box::new(Effect::AddCounters {
                target: NULL_OBJECT_ID,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            }),
        },
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::GrantKeyword {
                target: NULL_OBJECT_ID,
                keyword: KeywordAbility::Deathtouch,
                duration: Duration::EndOfTurn,
            }),
        },
    ]
}
