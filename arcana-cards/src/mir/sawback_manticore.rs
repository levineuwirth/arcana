//! Sawback Manticore — `{3}{R}{G}` 2/4 Manticore (G/R).
//! `{4}: This creature gains flying until end of turn.`
//! `{1}: This creature deals 2 damage to target attacking or blocking
//! creature. Activate only if this creature is attacking or blocking and only
//! once each turn.`
//!
//! First ability grants flying to itself. Second deals 2 damage to a target
//! creature, once each turn. The "attacking or blocking" target restriction
//! and the "only if this creature is attacking or blocking" activation gate
//! are not in the documented filter / condition surface, so they are GAP'd
//! (the damage and once-per-turn limit are expressed).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::events::DamageTarget;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sawback Manticore");
    let manticore = reg.interner_mut().intern("Manticore");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(manticore);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}: This creature gains flying until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_flying,
            })
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: "target attacking or blocking creature" restriction and
                // "Activate only if this creature is attacking or blocking"
                // gate are outside the documented filter / condition surface.
                text: "{1}: This creature deals 2 damage to target attacking or \
                       blocking creature. Activate only if this creature is \
                       attacking or blocking and only once each turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: deal_two_creature,
            }),
    )
}

fn gain_flying(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Flying,
        duration: Duration::EndOfTurn,
    }]
}

fn deal_two_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: 2,
    }]
}
