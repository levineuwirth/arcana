//! Nullhide Ferox — `{2}{G}{G}` 6/6 Beast with Hexproof.
//!
//! Hexproof.
//! You can't cast noncreature spells.
//! {2}: This creature loses all abilities until end of turn. Any
//!   player may activate this ability.
//! If a spell or ability an opponent controls causes you to discard
//!   this card, put it onto the battlefield instead of putting it into
//!   your graveyard.
//!
//! Hexproof and the "{2}: loses all abilities" activated ability are
//! expressible. GAP'd: the "you can't cast noncreature spells" static
//! (a casting restriction), the "any player may activate" rider on the
//! activated ability (activation is modeled as the controller's only),
//! and the discard-to-battlefield replacement effect.

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
    let name = reg.interner_mut().intern("Nullhide Ferox");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };

    // GAP: static "You can't cast noncreature spells." — casting
    // restriction, not expressible.
    // GAP: replacement "If a spell or ability an opponent controls
    // causes you to discard this card, put it onto the battlefield
    // instead of putting it into your graveyard." — discard-to-play
    // replacement, not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            // GAP: "Any player may activate this ability." — activation
            // is modeled as the controller's; the open-activation rider
            // is omitted.
            text: "{2}: This creature loses all abilities until end of \
                   turn. Any player may activate this ability."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: lose_all_abilities,
        }),
    )
}

fn lose_all_abilities(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::LoseAllAbilities {
        target: ctx.source,
        duration: Duration::EndOfTurn,
    }]
}
