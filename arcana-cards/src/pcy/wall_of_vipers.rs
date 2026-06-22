//! Wall of Vipers — `{2}{B}` 2/4 Snake Wall with Defender.
//!
//! * Defender.
//! * `{3}: Destroy this creature and target creature it's blocking. Any player
//!   may activate this ability.` GAP: "it's blocking" has no documented
//!   blocking-pairing TargetFilter (broadened to target creature); GAP: "any
//!   player may activate" has no documented activation field.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wall of Vipers");
    let snake = reg.interner_mut().intern("Snake");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}: Destroy this creature and target creature it's blocking.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                ..ActivationCost::default()
            },
            // GAP: "it's blocking" restriction unexpressible — broadened to target creature.
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: destroy_self_and_target,
        }),
    )
}

fn destroy_self_and_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::DestroyPermanent { target: ctx.source }];
    if let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() {
        effects.push(Effect::DestroyPermanent { target: *id });
    }
    effects
}
