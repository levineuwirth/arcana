//! Abzan Devotee — `{1}{B}` 2/1 Dog Cleric.
//! "{1}: Add {W}, {B}, or {G}. Activate only once each turn."
//! "{2}{B}: Return this card from your graveyard to your hand."
//!
//! The graveyard-recursion ability is wired fully. The mana ability is
//! a documented GAP: there is no any-of-three-colors AddMana primitive
//! (only fixed-color ManaUnit::plain), so a color choice can't be
//! posted — same posture as Necra Disciple.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Abzan Devotee");
    let dog = reg.interner_mut().intern("Dog");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "{1}: Add {W}, {B}, or {G}. Activate only once each turn." —
    // no any-of-three-colors AddMana primitive to post a color choice.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{B}: Return this card from your graveyard to your hand.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: return_self_to_hand,
        }),
    )
}

fn return_self_to_hand(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToHand { target: ctx.source }]
}
