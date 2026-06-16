//! Saheeli, Radiant Creator — `{1}{G}{U}{R}` 4/4 legendary Human Artificer.
//! "Whenever you cast an Artificer or artifact spell, you get {E} (an energy
//!  counter)."
//! "At the beginning of combat on your turn, you may pay {E}{E}{E}. When you
//!  do, create a token that's a copy of target permanent you control, except
//!  it's a 5/5 artifact creature in addition to its other types and has
//!  haste. Sacrifice it at the beginning of the next end step."
//!
//! GAP (filter): the first trigger fires on artifact spells you cast; the
//! "Artificer creature spell" alternative (a subtype OR type disjunction) has
//! no single ObjectFilter builder, so only the artifact-type half is wired.
//! GAP (second ability): "you may pay {E}{E}{E}" is an energy payment — not a
//! supported OptionalPaymentKind (only Mana/Life). Without the energy gate
//! the rest of the ability (token-copy of target, made a 5/5 haste artifact,
//! sac at end step) cannot fire correctly, so the combat trigger body is
//! GAP'd. The combat trigger shell is wired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Saheeli, Radiant Creator");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_types(TypeLine::ARTIFACT.into())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: gain_energy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pay_energy_make_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gain_energy(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainEnergy { player: trig.controller, amount: 1 }]
}

fn pay_energy_make_copy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {E}{E}{E}" — energy is not a supported
    // OptionalPaymentKind (only Mana/Life). The dependent token-copy /
    // 5/5-artifact-haste / sac-at-end-step rider is therefore omitted.
    Vec::new()
}
