//! Fluttershy — `{1}{G}{W}` 0/4 Legendary Pegasus with Defender and Flying.
//! "{1}, {T}: Put a +1/+1 counter on each creature with a tail target player
//!  controls. Stare down up to one target creature until end of turn. (It can't
//!  attack or block as long as you're looking directly at it.)"
//!
//! Defender + Flying are base keywords. The activation targets up to one
//! creature and forbids it from attacking and blocking (the "stare down" half,
//! modeled as until-end-of-turn ForbidAttacking + ForbidBlocking). The
//! "+1/+1 counter on each creature with a tail target player controls" half is
//! GAP'd: "with a tail" is not a creature-subtype filter.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fluttershy");
    let pegasus = reg.interner_mut().intern("Pegasus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pegasus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender, KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}: Put a +1/+1 counter on each creature with a tail target player controls. Stare down up to one target creature until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: stare_down,
            }),
    )
}

/// "Stare down up to one target creature until end of turn" — it can't attack or
/// block. GAP: the "+1/+1 counter on each creature with a tail target player
/// controls" half is omitted ("with a tail" is not a subtype filter).
fn stare_down(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::ForbidAttacking { target: *id, duration: Duration::EndOfTurn },
        Effect::ForbidBlocking { target: *id, duration: Duration::EndOfTurn },
    ]
}
