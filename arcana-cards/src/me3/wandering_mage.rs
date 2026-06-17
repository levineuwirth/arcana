//! Wandering Mage — `{W}{U}{B}` 0/3 Human Cleric Wizard.
//! Three damage-prevention activated abilities.
//! - "{W}, Pay 1 life: Prevent the next 2 damage that would be dealt to
//!   target creature this turn."
//! - "{U}: Prevent the next 1 damage that would be dealt to target
//!   Cleric or Wizard creature this turn."
//! - "{B}, Put a -1/-1 counter on a creature you control: Prevent the
//!   next 2 damage that would be dealt to target player or planeswalker
//!   this turn."
//!
//! The third ability's "put a -1/-1 counter on a creature you control"
//! cost has no matching ActivationCost field — GAP'd (mana cost only).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wandering Mage");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);
    subtypes.0.insert(wizard);

    let cleric_sym = reg.interner_mut().intern("Cleric");
    let wizard_sym = reg.interner_mut().intern("Wizard");
    let cw_filter = ObjectFilter::creature()
        .with_subtypes_any(vec![cleric_sym, wizard_sym]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}, Pay 1 life: Prevent the next 2 damage that would be dealt to target creature this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    life: 1,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_2_creature,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}: Prevent the next 1 damage that would be dealt to target Cleric or Wizard creature this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(cw_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_1_cw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}, Put a -1/-1 counter on a creature you control: Prevent the next 2 damage that would be dealt to target player or planeswalker this turn.".into(),
                // GAP: "Put a -1/-1 counter on a creature you control" cost has no ActivationCost field; mana only.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                // "target player or planeswalker" — no precise filter; use any_target (creature/player/pw).
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_2_player,
            }),
    )
}

fn prevent_2_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(*id),
        amount: Some(2),
        duration: ReplacementDuration::EndOfTurn,
    }]
}

fn prevent_1_cw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(*id),
        amount: Some(1),
        duration: ReplacementDuration::EndOfTurn,
    }]
}

fn prevent_2_player(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            arcana_core::targets::ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            arcana_core::targets::ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::PreventDamage {
        target: dt,
        amount: Some(2),
        duration: ReplacementDuration::EndOfTurn,
    }]
}
