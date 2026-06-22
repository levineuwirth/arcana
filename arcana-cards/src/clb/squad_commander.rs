//! Squad Commander — `{3}{W}` 3/3 Kor Warrior.
//!
//! Oracle:
//! * "When this creature enters, create a 1/1 white Kor Warrior creature
//!   token for each creature in your party." — GAP: the count is "your
//!   party" (up to one each of Cleric/Rogue/Warrior/Wizard); no
//!   script:: helper computes party size, and the count is dynamic, so
//!   the whole effect is GAP'd rather than emit a wrong fixed count.
//! * "At the beginning of combat on your turn, if you have a full party,
//!   creatures you control get +1/+0 and gain indestructible until end
//!   of turn." — the board-wide pump + indestructible grant is wired;
//!   GAP: the "if you have a full party" intervening-if (no party-size
//!   predicate), so it fires unconditionally (over-fire fidelity gap).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Squad Commander");
    let kor = reg.interner_mut().intern("Kor");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kor);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_party_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                // GAP: "if you have a full party" intervening-if — no
                // party-size predicate available.
                intervening_if: None,
                effect: full_party_buff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_party_tokens(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "create a 1/1 Kor Warrior for each creature in your party" —
    // party size is not computable with available script:: helpers.
    Vec::new()
}

fn full_party_buff(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &arcana_core::targets::ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Sequence(vec![
            Effect::Pump {
                target: NULL_OBJECT_ID,
                power: 1,
                toughness: 0,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            },
            Effect::GrantKeyword {
                target: NULL_OBJECT_ID,
                keyword: KeywordAbility::Indestructible,
                duration: Duration::EndOfTurn,
            },
        ])),
    }]
}
