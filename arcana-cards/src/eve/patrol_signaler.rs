//! Patrol Signaler — `{1}{W}` 1/1 white Kithkin Soldier.
//! "{1}{W}, {Q}: Create a 1/1 white Kithkin Soldier creature token."
//! ({Q} is the untap symbol — treated as a mana-only cost since the untap
//! cost variant is not in the ActivationCost catalog; GAP noted below.)

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Patrol Signaler");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let soldier = reg.interner_mut().intern("Soldier");
    // Pre-intern token subtypes
    let _kithkin2 = reg.interner_mut().intern("Kithkin");
    let _soldier2 = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: {Q} (untap symbol) is not expressible as ActivationCost;
                // modeled as mana-only cost {1}{W}.
                text: "{1}{W}, {Q}: Create a 1/1 white Kithkin Soldier creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: create_kithkin_soldier_token,
            }),
    )
}

fn create_kithkin_soldier_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let kithkin = reg.interner().lookup("Kithkin").expect("Kithkin interned");
    let soldier = reg.interner().lookup("Soldier").expect("Soldier interned");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(kithkin);
    token_subtypes.0.insert(soldier);
    let token = TokenDefinition {
        name: kithkin,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}
