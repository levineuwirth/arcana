//! Zirda, the Dawnwaker — `{1}{R/W}{R/W}` 3/3 Legendary Creature — Elemental Fox.
//!
//! * Companion — Each permanent card in your starting deck has an activated
//!   ability. (Deck-building keyword; not a battlefield ability.)
//! * Abilities you activate that aren't mana abilities cost {2} less to activate.
//! * {1}, {T}: Target creature can't block this turn.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zirda, the Dawnwaker");
    let elemental = reg.interner_mut().intern("Elemental");
    let fox = reg.interner_mut().intern("Fox");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(fox);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R/W}{R/W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: "Companion" keyword (deck-building restriction) is not a usable
        // KeywordAbility variant. The static "non-mana activated abilities you
        // control cost {2} less" reduction is wired via a SelfEntersBattlefield
        // `ContinuousEffect::ability_cost_modifier` (the engine auto-exempts
        // mana abilities and floors the generic component at 0).
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_discount,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}: Target creature can't block this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: forbid_block,
            }),
    )
}

/// ETB: install "non-mana activated abilities of permanents you control
/// cost {2} less to activate", lasting while Zirda is on the
/// battlefield. The engine exempts mana abilities and floors the
/// generic component at 0.
fn etb_install_discount(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::ability_cost_modifier(
            trig.source,
            ObjectFilter::permanent().controlled_by(ControllerConstraint::You),
            -2,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn forbid_block(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ForbidBlocking {
        target: *id,
        duration: Duration::EndOfTurn,
    }]
}
