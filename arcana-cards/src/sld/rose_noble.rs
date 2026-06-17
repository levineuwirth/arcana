//! Rose Noble — `{3}{U}` 2/3 Legendary Human (blue) with Ward {2}.
//! "Ward {2}
//!  Whenever you cast a Doctor spell or creature spell with doctor's
//!  companion, draw a card.
//!  Doctor's companion."
//!
//! Ward {2} is wired. Doctor's companion is a deck-construction keyword with
//! no in-game effect and is not a supported keyword → GAP'd. The cast trigger
//! is wired for the "Doctor spell" half (a creature spell with the Doctor
//! subtype); the "or creature spell with doctor's companion" half can't be
//! expressed (no companion-keyword spell filter) → GAP'd as a partial.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rose Noble");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    let doctor_filter = script::subtype_filter(reg, "Doctor");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Ward(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        // GAP: Doctor's companion — not a supported keyword (deck-construction
        // rule with no in-game effect).
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP (partial): only the "Doctor spell" half is matched; the "or
            // creature spell with doctor's companion" half is not expressible.
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(doctor_filter),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: draw_a_card,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn draw_a_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
