//! Momir Vig, Simic Visionary — `{3}{G}{U}` 2/2 Legendary Elf Wizard.
//! "Whenever you cast a green creature spell, you may search your
//! library for a creature card, reveal it, then shuffle and put that
//! card on top." — effect GAP'd (no tutor-to-top-of-library effect).
//! "Whenever you cast a blue creature spell, reveal the top card of
//! your library. If it's a creature card, put that card into your
//! hand." — approximated with DigTopN (rest-to-bottom fidelity gap).

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Momir Vig, Simic Visionary");
    let elf = reg.interner_mut().intern("Elf");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let green_creature = ObjectFilter::creature().with_colors(ColorSet::green());
    let blue_creature = ObjectFilter::creature().with_colors(ColorSet::blue());

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(green_creature),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: tutor_to_top,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(blue_creature),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: reveal_top_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn tutor_to_top(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "search your library for a creature card, reveal it, then
    // shuffle and put that card on top." — no tutor-to-top-of-library
    // effect (TutorToHand goes to hand, not the top).
    Vec::new()
}

fn reveal_top_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 1,
        filter: Some(ObjectFilter::creature()),
        rest: DigRest::BottomRandom,
    }]
}
