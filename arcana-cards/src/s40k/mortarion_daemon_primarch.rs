//! Mortarion, Daemon Primarch — `{5}{B}` 5/6 Legendary Demon Primarch
//! with Flying.
//!
//! "Primarch of the Death Guard — At the beginning of your end step, you
//! may pay {X}. If you do, create X 2/2 black Astartes Warrior creature
//! tokens with menace. X can't be greater than the amount of life you
//! lost this turn."
//!
//! The end-step ability is GAP'd: `OptionalPaymentKind` has no variable
//! {X} mana form, and there is no primitive to bound X by life lost this
//! turn nor to mint X tokens from a paid X. Flying is the only fully
//! expressible piece.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mortarion, Daemon Primarch");
    let demon = reg.interner_mut().intern("Demon");
    let primarch = reg.interner_mut().intern("Primarch");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(primarch);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: end_step_primarch,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn end_step_primarch(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {X}. If you do, create X 2/2 black Astartes
    // Warrior tokens with menace, where X can't exceed life lost this
    // turn." OptionalPaymentKind has only Mana(fixed) / Life — no
    // variable-X mana payment, no way to mint X tokens from the paid X,
    // and no life-lost-this-turn cap primitive.
    Vec::new()
}
