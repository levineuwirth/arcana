//! Saruman the White — `{4}{U}` 4/4 Legendary Avatar Wizard.
//!
//! Oracle:
//! * Ward {2}
//! * Whenever you cast your second spell each turn, amass Orcs 2.
//!
//! Ward {2} is a parametrized keyword. The amass trigger fires on a spell you
//! cast; the "second spell each turn" ordinal precondition has no expressible
//! gate (no spell-ordinal accessor / intervening-if for "this is your Nth
//! spell"), so it is GAP'd — the trigger fires on every spell you cast rather
//! than only the second. The "amass Orcs 2" payload itself is expressed via
//! `Effect::Amass`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Saruman the White");
    let avatar = reg.interner_mut().intern("Avatar");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);
    subtypes.0.insert(wizard);

    // Pre-intern subtypes the Amass resolver needs.
    let _army = reg.interner_mut().intern("Army");
    let _orc = reg.interner_mut().intern("Orc");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost"))],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: "your SECOND spell each turn" — no spell-ordinal gate is
            // expressible; this fires on every spell you cast.
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: amass_orcs_2,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn amass_orcs_2(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let army_subtype = reg.interner().lookup("Army").unwrap_or_default();
    let race_subtype = reg.interner().lookup("Orc").unwrap_or_default();
    vec![Effect::Amass {
        controller: trig.controller,
        count: 2,
        army_subtype,
        race_subtype,
    }]
}
