//! Mesmerizing Benthid — `{3}{U}{U}` 4/5 Octopus.
//! "When this creature enters, create two 0/2 blue Illusion creature
//!  tokens with 'Whenever this token blocks a creature, that creature
//!  doesn't untap during its controller's next untap step.'"
//! "This creature has hexproof as long as you control an Illusion."
//!  (static conditional keyword — GAP, not a triggered/activated ability.)
//!
//! Decomposed as: no keyword line, one ETB triggered ability creating the
//! two Illusion tokens (their embedded block-trigger is GAP'd — TokenDefinition
//! has no documented way to carry that triggered ability) plus a GAP'd static.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mesmerizing Benthid");
    let octopus = reg.interner_mut().intern("Octopus");
    let _illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(octopus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: static "has hexproof as long as you control an Illusion" —
        // a conditional keyword grant, not a triggered/activated ability.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_make_illusions,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_make_illusions(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let illusion = reg.interner().lookup("Illusion").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);
    // GAP: the token's embedded triggered ability ("Whenever this token blocks
    // a creature, that creature doesn't untap during its controller's next untap
    // step") has no documented TokenDefinition path — emit the token body only.
    let make = || Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: illusion,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: subtypes.clone(),
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    };
    vec![make(), make()]
}
