//! The Lady of Otaria — `{3}{R}{G}` 5/5 Legendary Avatar.
//! Alternative cost "tap three untapped Dwarves you control" is GAP'd
//! (alternative casting costs aren't expressible). At the beginning of each
//! end step, if a land you controlled was put into a graveyard from the
//! battlefield this turn, reveal the top four cards and take any number of
//! Dwarf cards (effect GAP'd — see below).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP: "You may tap three untapped Dwarves you control rather than pay this
// spell's mana cost." — alternative casting costs are not expressible.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Lady of Otaria");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::Any,
            },
            intervening_if: None,
            effect: end_step_reveal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

// GAP: intervening-if "if a land you controlled was put into a graveyard from
// the battlefield this turn" has no available conditions:: / script:: helper;
// and the payoff "reveal the top four cards, put ANY NUMBER of Dwarf cards
// among them into your hand, rest on bottom" is not expressible (DigTopN is
// single-take only and there is no reveal-N-take-any-number-by-type effect).
fn end_step_reveal(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    Vec::new()
}
