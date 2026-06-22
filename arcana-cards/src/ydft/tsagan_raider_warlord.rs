//! Tsagan, Raider Warlord — `{R}{W}{B}` 1/4 Legendary Dinosaur Berserker.
//!
//! Double strike
//! Start your engines!
//! Whenever Tsagan attacks, creatures you control get +1/+0 until end of
//! turn for each creature you control with first strike or double strike.
//! Max speed — Tsagan has deathtouch. Other creatures you control have
//! first strike.
//!
//! Decomposed as: a keyword line (Double strike) plus one attack trigger.
//! "Start your engines!" / "Max speed" are not in the usable keyword
//! surface and the Max-speed static grants are continuous statics, so
//! they are GAP'd. The attack trigger pumps each creature you control by
//! +N/+0, where N is the number of creatures you control with first
//! strike or double strike (computed via two keyword counts).

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tsagan, Raider Warlord");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(berserker);

    // GAP: keyword "Start your engines!" not in the usable keyword surface.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    // GAP: "Max speed — Tsagan has deathtouch. Other creatures you control
    // have first strike." is a max-speed-gated continuous static (no
    // trigger/cost) and is not expressible here.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: pump_team_per_striker,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn pump_team_per_striker(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .with_keywords_any(vec![
                KeywordAbility::FirstStrike,
                KeywordAbility::DoubleStrike,
            ]),
        trig.controller,
    ) as i32;
    if n == 0 {
        return Vec::new();
    }
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    ids.into_iter()
        .map(|id| Effect::Pump {
            target: id,
            power: n,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        })
        .collect()
}
