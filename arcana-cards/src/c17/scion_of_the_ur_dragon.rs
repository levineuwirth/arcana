//! Scion of the Ur-Dragon — `{W}{U}{B}{R}{G}` 4/4 Legendary Dragon Avatar.
//!
//! * Flying.
//! * `{2}: Search your library for a Dragon permanent card and put it into your
//!   graveyard. If you do, Scion of the Ur-Dragon becomes a copy of that card
//!   until end of turn. Then shuffle.` — GAP: there is no tutor-to-graveyard
//!   effect, and "becomes a copy of that card (in your graveyard) until end of
//!   turn" is not expressible (`CopyPermanent` mints a token copy of a
//!   battlefield permanent; it does not transform this object into a graveyard
//!   card's copy). The activated ability is emitted with an empty resolver.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::effects::Effect;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scion of the Ur-Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black()
            | ColorSet::green()
            | ColorSet::red()
            | ColorSet::blue()
            | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}: Search your library for a Dragon permanent card and put it into your \
                   graveyard. If you do, Scion of the Ur-Dragon becomes a copy of that card \
                   until end of turn. Then shuffle."
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
            effect: tutor_and_copy,
        }),
    )
}

/// `{2}`: tutor a Dragon permanent card to graveyard, become a copy of it EOT.
fn tutor_and_copy(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no tutor-to-graveyard effect, and "becomes a copy of that card (a
    // graveyard card) until end of turn" is not expressible — CopyPermanent
    // mints a token copy of a battlefield permanent, not a self-transform into
    // a graveyard card's copy.
    Vec::new()
}
