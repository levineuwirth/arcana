//! Ocelot Pride — `{W}` 1/1 white Cat with First strike and Lifelink.
//! "Ascend. At the beginning of your end step, if you gained life this turn,
//!  create a 1/1 white Cat creature token. Then if you have the city's
//!  blessing, for each token you control that entered this turn, create a
//!  token that's a copy of it."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ocelot Pride");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);

    // Ascend is not an expressible KeywordAbility variant; the city's blessing
    // tracking is engine debt (see the trigger GAP below).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            // GAP: intervening-if "if you gained life this turn" — no
            // gained-life-this-turn condition helper is available; firing the
            // token-creation unconditionally is the best-effort partial.
            intervening_if: None,
            effect: end_step_make_cat,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn end_step_make_cat(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let cat = reg.interner().lookup("Cat").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    // GAP: "Then if you have the city's blessing, for each token you control
    // that entered this turn, create a token that's a copy of it" — neither
    // the city's-blessing state nor per-token "entered this turn" membership
    // is expressible.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: arcana_core::effects::TokenDefinition {
            name: cat,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
