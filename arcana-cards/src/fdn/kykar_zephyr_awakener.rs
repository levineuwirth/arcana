//! Kykar, Zephyr Awakener — `{2}{W}{U}` 3/4 Legendary Bird Wizard with Flying.
//! "Whenever you cast a noncreature spell, choose one —
//!   • Exile another target creature you control. Return that card to the
//!     battlefield under its owner's control at the beginning of the next end
//!     step.
//!   • Create a 1/1 white Spirit creature token with flying."
//!
//! The triggered ability is MODAL ("choose one"); the catalog's modal dispatch
//! is spell-only (no triggered-modal recipe), so the choice itself is a GAP.
//! We implement the second mode (create a flying Spirit token), which is always
//! legal and non-targeted, as the trigger's effect.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kykar, Zephyr Awakener");
    let bird = reg.interner_mut().intern("Bird");
    let wizard = reg.interner_mut().intern("Wizard");
    reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        arcana_core::targets::ObjectFilter::new()
                            .without_types(TypeLine::CREATURE.into()),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_spirit,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_spirit(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the modal "choose one" is not expressible on a triggered ability;
    // mode 0 (exile-and-return another target creature you control) is omitted.
    let spirit = reg.interner().lookup("Spirit").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: spirit,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
