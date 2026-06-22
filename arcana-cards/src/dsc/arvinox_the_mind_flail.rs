//! Arvinox, the Mind Flail — `{4}{B}{B}{B}` 9/9 Legendary Enchantment
//! Creature — Horror.
//!
//! * Static: "Arvinox isn't a creature unless you control three or more
//!   permanents you don't own." — a continuous characteristic-defining /
//!   type-removing static, not a triggered or activated ability.
//! * "At the beginning of your end step, exile the bottom card of each
//!   opponent's library face down. For as long as those cards remain
//!   exiled, you may look at them, you may cast permanent spells from
//!   among them, …"

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arvinox, the Mind Flail");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(9)),
        ..Default::default()
    };
    // GAP: static "Arvinox isn't a creature unless you control three or more
    // permanents you don't own" — a continuous type-removing static, not a
    // triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_steal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_steal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile the bottom card of each opponent's library face down; for
    // as long as exiled you may look at and cast permanent spells from among
    // them, spending mana as any color" — no effect exiles the bottom card of
    // a library with a play-permission/any-color rider.
    Vec::new()
}
