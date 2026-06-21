//! Go-Shintai of Shared Purpose — `{3}{W}` 1/3 Legendary Enchantment
//! Creature — Shrine with Vigilance.
//!
//! Oracle:
//! * Vigilance.
//! * "At the beginning of your end step, you may pay {1}. If you do,
//!   create a 1/1 colorless Spirit creature token for each Shrine you
//!   control." — an end-step trigger gated by an optional {1} payment;
//!   the token count scales with the number of Shrines you control.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Go-Shintai of Shared Purpose");
    let shrine = reg.interner_mut().intern("Shrine");
    let _spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shrine);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
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
            effect: end_step_spirits,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn end_step_spirits(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spirit = reg
        .interner()
        .lookup("Spirit")
        .expect("Spirit interned during register()");
    let shrines = script::count_matching(
        state,
        &script::subtype_filter(reg, "Shrine").controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let tokens: Vec<Effect> = (0..shrines)
        .map(|_| Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: spirit,
                colors: ColorSet::colorless(),
                types: TypeLine::CREATURE.into(),
                subtypes: subtypes.clone(),
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        })
        .collect();
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}").expect("valid cost")),
        then: Box::new(Effect::Sequence(tokens)),
        else_effect: None,
    }]
}
