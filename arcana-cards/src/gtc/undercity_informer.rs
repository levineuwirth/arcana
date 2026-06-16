//! Undercity Informer — `{2}{B}` 2/3 black Human Rogue.
//! "{1}, Sacrifice a creature: Target player reveals cards from the top of their library until
//! they reveal a land card, then puts those cards into their graveyard."
//!
//! GAP: "Sacrifice a creature" (not self) not expressible in ActivationCost.
//! "Mill until land" is not a discrete Effect variant (Mill mills a fixed count).

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
    let name = reg.interner_mut().intern("Undercity Informer");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Sacrifice a creature: Target player reveals cards from the top of their library until they reveal a land card, then puts those cards into their graveyard.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").unwrap(),
                    // GAP: "Sacrifice a creature" (non-self) not expressible.
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: mill_until_land,
            }),
    )
}

fn mill_until_land(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Mill until land" not a distinct Effect variant; returning Vec::new().
    Vec::new()
}
