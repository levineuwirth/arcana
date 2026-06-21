//! Brimaz, Blight of Oreskos — `{2}{W}{B}` 3/4 Legendary Phyrexian Cat.
//! * "Whenever you cast a Phyrexian creature or artifact creature spell,
//!   incubate X, where X is that spell's mana value."
//! * "At the beginning of each end step, if a Phyrexian died under your
//!   control this turn, proliferate."
//!
//! The Scryfall keywords (Incubate, Transform, Proliferate) are mechanic
//! names, not keyword abilities, so the keyword line is empty.
//!
//! Trigger 1 fires on the cast (filtered to artifact-creature spells —
//! see GAP), but its amount "X = that spell's mana value" is dynamic and
//! there is NO PendingTrigger accessor for the cast spell's object or
//! mana value, so per the dynamic-amount rule the WHOLE Incubate effect
//! is GAP'd. Trigger 2 proliferates each end step, gated by an
//! intervening-if that a Phyrexian died under your control this turn.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brimaz, Blight of Oreskos");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(cat);

    // "a Phyrexian creature or artifact creature spell" — the engine
    // SpellCast filter cannot OR a subtype with a type via the demonstrated
    // API, so this restricts to artifact-creature spells you cast (the
    // Phyrexian-creature half is a documented under-fire).
    let spell_filter = ObjectFilter::new()
        .with_types(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE));

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(spell_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: incubate_x,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_phyrexian_died),
                effect: proliferate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn incubate_x(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "incubate X, where X is that spell's mana value" — the amount is
    // dynamic on the cast spell's mana value, and no PendingTrigger accessor
    // exposes the cast spell or its mana value. Per the dynamic-amount rule
    // the whole effect is GAP'd rather than hardcoding a literal Incubate N.
    Vec::new()
}

fn if_phyrexian_died(
    state: &GameState,
    _src: ObjectId,
    _you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    script::creatures_of_subtype_died_this_turn(state, reg, "Phyrexian") > 0
}

fn proliferate(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Proliferate]
}
