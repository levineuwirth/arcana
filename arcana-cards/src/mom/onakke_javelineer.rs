//! Onakke Javelineer — `{4}{R}` 5/4 Ogre Spirit (R). Reach.
//! `{T}: This creature deals 2 damage to target player or battle.`
//!
//! Reach is a base keyword. The activated ability deals 2 damage; battles are
//! not a modeled target type, so the target is restricted to a player and the
//! "or battle" alternative is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Onakke Javelineer");
    let ogre = reg.interner_mut().intern("Ogre");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: "or battle" — battles are not a modeled target type;
                // restricted to target player.
                text: "{T}: This creature deals 2 damage to target player or battle."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_two_to_player,
            }),
    )
}

fn deal_two_to_player(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Player(*p),
        amount: 2,
    }]
}
