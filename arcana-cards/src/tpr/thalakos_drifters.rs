//! Thalakos Drifters — `{2}{U}{U}` 3/3 blue Thalakos. "Discard a card: This
//! creature gains shadow until end of turn."
//!
//! GAP: "Discard a card" activation cost not expressible via ActivationCost.
//! Shadow is a supported keyword so the effect is wired.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thalakos Drifters");
    let thalakos = reg.interner_mut().intern("Thalakos");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thalakos);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Discard a card: This creature gains shadow until end of turn.".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_shadow,
            }),
    )
}

fn gain_shadow(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Discard a card" activation cost not expressible in ActivationCost.
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Shadow,
        duration: Duration::EndOfTurn,
    }]
}
