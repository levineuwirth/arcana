//! Handy Dandy Clone Machine — `{3}` artifact (Unstable).
//! "{2}, {T}: Create a 2/2 colorless Homunculus creature token. It
//! must be represented by a unique hand and two fingers at all times,
//! or it ceases to exist." The token-making activation is wired; the
//! Un-set physical-representation rider is unmodeled and is a GAP.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Handy Dandy Clone Machine");
    let _homunculus = reg.interner_mut().intern("Homunculus");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{2}, {T}: Create a 2/2 colorless Homunculus creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_homunculus,
            },
        ),
    )
}

fn make_homunculus(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "It must be represented by a unique hand and two fingers at all
    // times, or it ceases to exist." — Un-set physical-representation rule,
    // unmodeled.
    let homunculus = reg.interner().lookup("Homunculus").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(homunculus);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: homunculus,
            colors: ColorSet::new(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
