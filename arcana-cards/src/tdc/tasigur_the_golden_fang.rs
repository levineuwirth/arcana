//! Tasigur, the Golden Fang — `{5}{B}` 4/5 Legendary Human Shaman.
//! "Delve (Each card you exile from your graveyard while casting this
//!  spell pays for {1}.)
//!  {2}{G/U}{G/U}: Mill two cards, then return a nonland card of an
//!  opponent's choice from your graveyard to your hand."
//!
//! GAP: Delve is a cast-time alternative-cost mechanic with no
//! expressible primitive on this card class (keywords vec is empty —
//! Delve is not a KeywordAbility variant).
//!
//! GAP (within the activated ability): "return a nonland card of an
//! opponent's choice from your graveyard to your hand" has no
//! opponent-chosen graveyard-return primitive; only the Mill 2 portion
//! is emitted.

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
    let name = reg.interner_mut().intern("Tasigur, the Golden Fang");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{G/U}{G/U}: Mill two cards, then return a nonland card of an opponent's choice from your graveyard to your hand.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{G/U}{G/U}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: mill_then_return,
        }),
    )
}

fn mill_then_return(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Only the "Mill two cards" portion is expressible.
    // GAP: "return a nonland card of an opponent's choice from your
    // graveyard to your hand" — no opponent-chosen graveyard return.
    vec![Effect::Mill {
        player: ctx.controller,
        count: 2,
    }]
}
