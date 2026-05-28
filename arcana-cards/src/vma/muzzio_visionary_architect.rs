//! Muzzio, Visionary Architect — `{1}{U}{U}` 1/3 blue Legendary Human Artificer.
//! "{3}{U}, {T}: Look at the top X cards of your library, where X is the
//! greatest mana value among artifacts you control. You may put an artifact
//! card from among them onto the battlefield. Put the rest on the bottom of
//! your library in any order."
//!
//! GAP: "look at top X, put artifact to battlefield, rest to bottom in any
//! order" — no Effect variant for this scry-like selection with free cast.
//! TutorToBattlefield is closest but doesn't match the "top X" constraint.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Muzzio, Visionary Architect");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}, {T}: Look at the top X cards of your library where X is the greatest CMC among artifacts you control. You may put an artifact card from among them onto the battlefield.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: muzzio_ability,
            }),
    )
}

fn muzzio_ability(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at top X cards (X = greatest artifact CMC), put one artifact
    // to battlefield, rest to bottom in any order" — not expressible
    Vec::new()
}
