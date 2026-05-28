//! Goldmeadow Lookout — `{3}{W}` 2/2 white Kithkin Spellshaper.
//! "{W}, {T}, Discard a card: Create a 1/1 white Kithkin Soldier creature token named
//! Goldmeadow Harrier. It has '{W}, {T}: Tap target creature.'"
//! GAP: Token with an activated ability ('{W}, {T}: Tap target creature') — TokenDefinition
//! does not support embedded activated abilities.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::effects::DiscardChoice;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goldmeadow Lookout");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let spellshaper = reg.interner_mut().intern("Spellshaper");
    let _soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(spellshaper);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}, {T}, Discard a card: Create a 1/1 white Kithkin Soldier creature token named Goldmeadow Harrier.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").unwrap(),
                    tap: true,
                    discard_self: false,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: create_kithkin_token,
            }),
    )
}

fn create_kithkin_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: DiscardChoice from hand (not discard_self) not expressed in ActivationCost —
    // the discard cost is approximated by ActivationCost fields.
    // GAP: Token has activated ability not supported in TokenDefinition.
    let soldier = reg.interner().lookup("Soldier").expect("Soldier interned during register()");
    let kithkin = reg.interner().lookup("Kithkin").expect("Kithkin interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(kithkin);
    token_subtypes.0.insert(soldier);
    let token = TokenDefinition {
        name: soldier,
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
