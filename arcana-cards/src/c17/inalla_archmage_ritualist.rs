//! Inalla, Archmage Ritualist — `{2}{U}{B}{R}` 4/5 Legendary Human Wizard.
//! Eminence is not a supported keyword — keyword line empty.
//! "Eminence — Whenever another nontoken Wizard you control enters, if Inalla is in
//! the command zone or on the battlefield, you may pay {1}. If you do, create a token
//! that's a copy of that Wizard. The token gains haste. Exile it at the beginning of
//! the next end step." Modeled as a ZoneChange trigger → pay {1} → token copy of the
//! entering Wizard. GAP: the "command zone or battlefield" intervening-if (no helper),
//! and the "gains haste / exile at next end step" riders on the minted copy.
//! "Tap five untapped Wizards you control: Target player loses 7 life."

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inalla, Archmage Ritualist");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    let wizard_filter = script::subtype_filter(reg, "Wizard")
        .controlled_by(ControllerConstraint::You)
        .nontoken();
    let tap_filter = script::subtype_filter(reg, "Wizard").controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: wizard_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                // GAP: intervening-if "Inalla is in the command zone or on the
                // battlefield" — no helper for command-zone presence.
                intervening_if: None,
                effect: eminence_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap five untapped Wizards you control: Target player loses 7 life.".into(),
                cost: ActivationCost {
                    tap_other: Some(tap_filter),
                    tap_other_count: 5,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: target_loses_7,
            }),
    )
}

fn eminence_copy(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(id) = trig.entering_object() else { return Vec::new(); };
    // GAP: "the token gains haste" and "exile it at the beginning of the next end
    // step" riders on the minted copy are not attachable to CopyPermanent.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}").expect("valid cost")),
        then: Box::new(Effect::CopyPermanent { target: id }),
        else_effect: None,
    }]
}

fn target_loses_7(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![Effect::LoseLife { player: *p, amount: 7 }]
}
