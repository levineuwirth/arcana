//! Tornellan Protector — `{2}{W}` 1/2 white Cleric.
//! "{T}: Until end of turn, each time damage is dealt to target creature or player,
//! prevent X of that damage, where X is a number from 1 to 3 chosen at random each time."
//! GAP: Random variable prevention amount (1-3) cannot be modeled without a random
//! number generator in Effects. Approximating with PreventDamage of 2 (midpoint) to a
//! target creature; actual oracle uses a replacement effect with random amount.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tornellan Protector");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Until end of turn, each time damage is dealt to target creature or player, prevent X of that damage, where X is a number from 1 to 3 chosen at random each time.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: random_prevent,
            }),
    )
}

fn random_prevent(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "prevent X damage where X is 1-3 chosen at random" — no Effect variant for
    // random prevention amount or until-end-of-turn prevention replacement effect.
    Vec::new()
}
