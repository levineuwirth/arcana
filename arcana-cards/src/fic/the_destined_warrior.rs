//! The Destined Warrior — `{1}{W}{U}{B}` 3/3 Legendary Human Warrior.
//! First strike, vigilance, menace.
//! "Cleric, Rogue, Warrior, and Wizard spells you cast cost {1} less to
//!   cast." — GAP (static cost reduction; not a triggered/activated
//!   ability and no cost-reduction Effect).
//! "At the beginning of combat on your turn, creatures you control get
//!   +1/+0 until end of turn. If you have a full party, creatures you
//!   control get +3/+0 until end of turn instead."
//!
//! The combat trigger pumps each creature you control +1/+0 (base case).
//! GAP: the "full party" upgrade to +3/+0 — there is no party-check
//! predicate, so the larger pump is omitted (the base +1/+0 still fires).

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
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Destined Warrior");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::FirstStrike,
            KeywordAbility::Vigilance,
            KeywordAbility::Menace,
        ],
        ..Default::default()
    };

    // GAP: "Cleric, Rogue, Warrior, and Wizard spells you cast cost {1}
    // less" — static cost reduction.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: pump_team,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn pump_team(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "+3/+0 if you have a full party" — no party predicate; only the
    // base +1/+0 is applied.
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
