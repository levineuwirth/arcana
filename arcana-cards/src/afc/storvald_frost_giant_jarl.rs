//! Storvald, Frost Giant Jarl — `{4}{G}{W}{U}` 7/7 Legendary Giant with Ward {3}.
//!
//! Oracle:
//! * Ward {3}.
//! * Other creatures you control have ward {3}. (No static keyword-grant
//!   primitive here — GAP'd.)
//! * Whenever Storvald enters or attacks, choose one or both —
//!     • Target creature has base P/T 7/7 until end of turn.
//!     • Target creature has base P/T 1/1 until end of turn.
//!   (The choose-one-or-both modal isn't expressible on a triggered ability,
//!   so each of the two triggers — enters and attacks — wires only the 7/7
//!   mode against a single target creature; the modal and the 1/1 mode are
//!   GAP'd.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Storvald, Frost Giant Jarl");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Ward(
            ManaCost::parse("{3}").expect("valid cost"),
        )],
        ..Default::default()
    };

    // GAP: "Other creatures you control have ward {3}" — no static
    // keyword-grant primitive available; static omitted.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: set_seven_seven,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: set_seven_seven,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn set_seven_seven(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: choose-one-or-both modal on a triggered ability isn't expressible;
    // only the base 7/7 mode is wired (the 1/1 mode is omitted).
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::SetBasePT {
        target: *id,
        power: 7,
        toughness: 7,
        duration: Duration::EndOfTurn,
    }]
}
