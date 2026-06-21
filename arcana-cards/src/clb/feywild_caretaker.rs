//! Feywild Caretaker — `{4}{U}` 3/4 Orc Wizard.
//!
//! Oracle:
//! * When this creature enters, you take the initiative. (no initiative
//!   primitive — effect GAP'd)
//! * At the beginning of your end step, if you have the initiative, create a
//!   1/1 blue Faerie Dragon creature token with flying.
//!
//! The initiative mechanic has no expressible primitive: "take the initiative"
//! has no effect, and "if you have the initiative" has no condition predicate.
//! The ETB take-initiative effect is GAP'd. The end-step trigger's token
//! creation IS expressible, so it is emitted; its "if you have the initiative"
//! intervening-if is GAP'd (set to None — fires unconditionally) since no
//! predicate exists.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Feywild Caretaker");
    let orc = reg.interner_mut().intern("Orc");
    let wizard = reg.interner_mut().intern("Wizard");
    // Pre-intern the token subtype for the resolver.
    let _faerie = reg.interner_mut().intern("Faerie");
    let _dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: take_initiative,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                // GAP: intervening-if — "if you have the initiative" has no
                // predicate (initiative is unmodeled); fires unconditionally.
                intervening_if: None,
                effect: make_faerie_dragon,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn take_initiative(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you take the initiative." The initiative mechanic is unmodeled.
    Vec::new()
}

fn make_faerie_dragon(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let faerie = reg.interner().lookup("Faerie").unwrap_or_default();
    let dragon = reg.interner().lookup("Dragon").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(dragon);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: faerie,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
